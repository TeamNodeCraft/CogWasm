//! One long-lived rustc.wasm guest that serves many compile requests.
//!
//! A [`ServeSession`] owns a worker thread with the store and instance. The
//! guest is started with `COGWASM_SERVE=1`, so instead of reading its argv once
//! it asks the host for a request, runs one compiler entrypoint, reports the
//! exit code and spawn records, and asks for the next request. The host keeps
//! the Wasmtime engine and module loaded, and the guest keeps its metadata
//! cache and allocator warm.
//!
//! The session is deliberately small: it does not decide what to compile or
//! when to restart. [`module_supports_serve`] tells a caller whether an older
//! one-shot `rustc.wasm` must use the per-job path, and [`ServeSession::is_dead`]
//! tells it when a trap or the memory cap requires a fresh session.

use anyhow::{anyhow, bail, Result};
use cogwasm_runtime::{
    encode_request, prepare_instance, ArgvRecorder, ServeEvent, ServeHooks, SpawnSummary,
    StoreOptions,
};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::Instant;
use wasmtime::{Engine, Module};
use wasmtime_wasi::I32Exit;

use crate::compile::CompileOutcome;

/// Whether `module` was built with the serve patch and can run a
/// [`ServeSession`]. Old one-shot modules have no `cogwasm` imports.
#[must_use]
pub fn module_supports_serve(module: &Module) -> bool {
    module
        .imports()
        .any(|import| import.module() == "cogwasm" && import.name() == "next_request")
}

/// A running rustc.wasm guest that serves requests until dropped.
pub struct ServeSession {
    /// `None` once the session has been dropped, which closes the channel and
    /// lets the guest's `next_request` return 0.
    requests: Option<Sender<Vec<u8>>>,
    events: Receiver<ServeEvent>,
    thread: Option<JoinHandle<()>>,
    dead: AtomicBool,
    max_memory_bytes: usize,
}

impl ServeSession {
    /// Instantiate one guest and wait until it is ready to accept a request.
    ///
    /// The worker owns the store for the whole session; the caller only sees
    /// the channels. The engine and module are cloned handles to the already
    /// loaded objects, so no module is compiled here. `preopens` are the
    /// directories later requests will use (work root, sysroot); each is
    /// preopened at its own path so the guest's file calls through it are
    /// cheap.
    ///
    /// # Errors
    ///
    /// Returns an error when the worker cannot be spawned, the guest cannot be
    /// instantiated, or its `_start` exits before the first request.
    pub fn new(
        engine: &Engine,
        module: &Module,
        linker: &Path,
        preopens: &[PathBuf],
    ) -> Result<Self> {
        let (request_tx, request_rx) = channel();
        let (event_tx, event_rx) = channel();
        let options = StoreOptions {
            env: vec![("COGWASM_SERVE".to_owned(), "1".to_owned())],
            serve: Some(ServeHooks::new(request_rx, event_tx.clone())),
            preopens: preopens.to_vec(),
            ..StoreOptions::default()
        };
        let worker_engine = engine.clone();
        let worker_module = module.clone();
        let worker_linker = linker.to_path_buf();
        let ready = event_tx.clone();
        let thread = thread::Builder::new()
            .name("cogwasm-serve".to_owned())
            .spawn(move || {
                let result = serve_thread(
                    &worker_engine,
                    &worker_module,
                    &worker_linker,
                    options,
                    &ready,
                );
                let ended = match result {
                    Ok(()) => "rustc.wasm serve loop ended".to_owned(),
                    Err(error) => format!("{error:#}"),
                };
                let _ = event_tx.send(ServeEvent::Ended(ended));
            })
            .map_err(|error| anyhow!("spawning the rustc.wasm serve thread: {error}"))?;

        let session = Self {
            requests: Some(request_tx),
            events: event_rx,
            thread: Some(thread),
            dead: AtomicBool::new(false),
            max_memory_bytes: max_memory_bytes(),
        };
        match session.events.recv() {
            Ok(ServeEvent::Ready) => Ok(session),
            Ok(ServeEvent::Ended(message)) => {
                session.dead.store(true, Ordering::Release);
                Err(anyhow!("{message}"))
            }
            Ok(ServeEvent::Finished(_)) => {
                session.dead.store(true, Ordering::Release);
                Err(anyhow!("rustc.wasm reported a request before it was ready"))
            }
            Err(_) => {
                session.dead.store(true, Ordering::Release);
                Err(anyhow!("rustc.wasm serve thread ended before it was ready"))
            }
        }
    }

    /// Whether a trap, an unexpected exit or the memory cap retired the
    /// session. A dead session must be replaced, not reused.
    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.dead.load(Ordering::Acquire)
    }

    /// Send one argv to the guest and wait for its exit code and timings.
    ///
    /// `instantiate_ms` is 0 because the guest was instantiated when the
    /// session started. `rustc_total_ms` is the request's full round trip.
    ///
    /// # Errors
    ///
    /// Returns an error for a zero-argument/closed channel, a trap or exit
    /// during the request, a non-zero rustc exit code, or a failed linker
    /// spawn. The session is marked dead when it cannot serve again.
    pub fn compile_args(&self, args: &[String]) -> Result<CompileOutcome> {
        if self.is_dead() {
            bail!("rustc.wasm serve session is not running");
        }
        let request = encode_request(args);
        let started = Instant::now();
        let sent = self
            .requests
            .as_ref()
            .ok_or_else(|| anyhow!("rustc.wasm serve session is closed"))?
            .send(request);
        if sent.is_err() {
            self.dead.store(true, Ordering::Release);
            bail!("rustc.wasm serve session is not running");
        }

        loop {
            match self.events.recv() {
                Ok(ServeEvent::Ready) => {}
                Ok(ServeEvent::Finished(finish)) => {
                    if finish.memory_bytes > self.max_memory_bytes {
                        self.dead.store(true, Ordering::Release);
                        eprintln!(
                            "[cogwasm-host] rustc.wasm memory is {} MiB after a request \
                             (COGWASM_SERVE_MAX_MEMORY_MB={}); restarting the session",
                            finish.memory_bytes / (1 << 20),
                            self.max_memory_bytes / (1 << 20),
                        );
                    }
                    if finish.code != 0 {
                        bail!("rustc.wasm exited with an error: exit {}", finish.code);
                    }
                    let spawns: Vec<SpawnSummary> = finish.spawns;
                    if spawns.iter().any(|spawn| spawn.status != 0) {
                        bail!("linker failed: {spawns:?}");
                    }
                    let link_ms = spawns.iter().map(|spawn| spawn.ms).sum::<f64>();
                    let rustc_total_ms = started.elapsed().as_secs_f64() * 1e3;
                    return Ok(CompileOutcome {
                        instantiate_ms: 0.0,
                        rustc_total_ms,
                        link_ms,
                        compile_ms: (rustc_total_ms - link_ms).max(0.0),
                        spawns,
                    });
                }
                Ok(ServeEvent::Ended(message)) => {
                    self.dead.store(true, Ordering::Release);
                    bail!("{message}");
                }
                Err(_) => {
                    self.dead.store(true, Ordering::Release);
                    bail!("rustc.wasm serve session ended before replying");
                }
            }
        }
    }
}

impl Drop for ServeSession {
    fn drop(&mut self) {
        // Closing the request channel is what makes the guest's
        // `next_request` return 0; after that `_start` returns and the worker
        // can be joined.
        self.requests.take();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn serve_thread(
    engine: &Engine,
    module: &Module,
    linker: &Path,
    options: StoreOptions,
    ready: &Sender<ServeEvent>,
) -> Result<()> {
    let prepared = prepare_instance(
        engine,
        module,
        &[],
        linker,
        "[cogwasm-host]",
        options,
        ArgvRecorder::default(),
    )?;
    let mut store = prepared.store;
    let instance = prepared.instance;
    let start = instance
        .get_typed_func::<(), ()>(&mut store, "_start")
        .map_err(|error| anyhow!("rustc.wasm _start: {error}"))?;
    let _ = ready.send(ServeEvent::Ready);
    match start.call(&mut store, ()) {
        Ok(()) => Ok(()),
        Err(error)
            if error
                .downcast_ref::<I32Exit>()
                .is_some_and(|exit| exit.0 == 0) =>
        {
            Ok(())
        }
        Err(error) => Err(anyhow!("rustc.wasm serve loop failed: {error}")),
    }
}

fn max_memory_bytes() -> usize {
    const MIB: usize = 1 << 20;
    std::env::var("COGWASM_SERVE_MAX_MEMORY_MB")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1024)
        .saturating_mul(MIB)
}

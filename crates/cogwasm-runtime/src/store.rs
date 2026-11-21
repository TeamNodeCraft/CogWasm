//! Store construction shared by the `CogWasm` hosts.
//!
//! The two hosts differ only in options: the product host inherits stderr,
//! uses the OS random source, and sets neither fuel nor a resource limiter;
//! the benchmark host captures stderr, can pin randomness and fuel, and tracks
//! peak memory. [`StoreOptions`] defaults reproduce the product host exactly.

use crate::Wt;
use crate::spawn::{SpawnRecorder, add_playground_spawn};
use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Linker, Module, ResourceLimiter, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::pipe::MemoryOutputPipe;
use wasmtime_wasi::{Deterministic, FsPerms, WasiCtxBuilder};

/// WASM page size used by peak-memory accounting.
pub const PAGE_SIZE: u64 = 65_536;

/// Capacity of the captured-stderr pipe, matching the benchmark host's old
/// constant.
pub const STDERR_CAPACITY: usize = 8 << 20;

/// A resource limiter that only tracks the largest memory size requested.
///
/// It never denies a growth request: the benchmark uses `max_pages` to report
/// peak memory, not to bound the guest.
#[derive(Clone, Copy, Debug, Default)]
pub struct Limits {
    /// Largest number of pages the guest has grown to, as observed so far.
    pub max_pages: u64,
}

impl ResourceLimiter for Limits {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        let pages = u64::try_from(desired).unwrap_or(u64::MAX) / PAGE_SIZE;
        self.max_pages = self.max_pages.max(pages);
        Ok(true)
    }

    fn table_growing(
        &mut self,
        _current: usize,
        _desired: usize,
        _maximum: Option<usize>,
    ) -> wasmtime::Result<bool> {
        Ok(true)
    }
}

/// Per-store host state shared by both hosts.
pub struct HostState<R: SpawnRecorder> {
    wasi: WasiP1Ctx,
    /// Peak-memory tracker installed only when [`StoreOptions::limits`] is set.
    pub limits: Limits,
    recorder: R,
}

impl<R: SpawnRecorder> HostState<R> {
    /// Build state with an explicit WASI context, limiter state and recorder.
    pub fn new(wasi: WasiP1Ctx, limits: Limits, recorder: R) -> Self {
        HostState {
            wasi,
            limits,
            recorder,
        }
    }

    /// The spawn recorder installed for this store.
    pub fn recorder(&self) -> &R {
        &self.recorder
    }
}

/// Options for [`prepare_instance`].
///
/// `StoreOptions::default()` is the product host's configuration: stderr is
/// inherited, randomness comes from the OS, and there is no fuel or resource
/// limiter.
#[derive(Clone, Copy, Debug, Default)]
pub struct StoreOptions {
    /// Capture guest stderr into the returned [`PreparedInstance::stderr`]
    /// instead of inheriting the host's stderr.
    pub capture_stderr: bool,
    /// Pin the WASI random sources to a deterministic stream derived from this
    /// seed. Seed 0 is the benchmark host's historical stream: secure bytes
    /// `0..=255` and insecure bytes `255..=0`; the same seed is also returned
    /// from `wasi:random/insecure-seed`.
    pub deterministic_random: Option<u64>,
    /// Wasmtime fuel limit for the store.
    pub fuel: Option<u64>,
    /// Install a [`Limits`] tracker. The value's initial `max_pages` is
    /// preserved.
    pub limits: Option<Limits>,
}

/// A freshly instantiated guest and the store that owns it.
pub struct PreparedInstance<R: SpawnRecorder> {
    /// Store holding WASI state, the limiter and the spawn recorder.
    pub store: Store<HostState<R>>,
    /// The instantiated rustc.wasm module.
    pub instance: wasmtime::Instance,
    /// Captured stderr, present exactly when [`StoreOptions::capture_stderr`]
    /// is set.
    pub stderr: Option<MemoryOutputPipe>,
}

/// Build the full linker (WASI p1 and `playground.spawn`), the store, and one
/// rustc.wasm instance.
///
/// `log_prefix` is passed through to the spawn import, for example
/// `[cogwasm-host]`.
pub fn prepare_instance<R: SpawnRecorder>(
    engine: &Engine,
    module: &Module,
    args: &[String],
    linker_path: &Path,
    log_prefix: &'static str,
    options: StoreOptions,
    recorder: R,
) -> Result<PreparedInstance<R>> {
    let stderr = options
        .capture_stderr
        .then(|| MemoryOutputPipe::new(STDERR_CAPACITY));
    let mut wasi = WasiCtxBuilder::new();
    wasi.inherit_stdout().args(args);
    match &stderr {
        Some(pipe) => {
            wasi.stderr(pipe.clone());
        }
        None => {
            wasi.inherit_stderr();
        }
    }
    wasi.env("RUST_MIN_STACK", "16777216")
        .env("RUST_BACKTRACE", "1");
    if let Some(seed) = options.deterministic_random {
        let low_byte = seed.to_le_bytes()[0];
        let secure = (0..=255_u8)
            .map(|byte| byte.wrapping_add(low_byte))
            .collect::<Vec<_>>();
        let insecure = (0..=255_u8)
            .rev()
            .map(|byte| byte.wrapping_add(low_byte))
            .collect::<Vec<_>>();
        wasi.secure_random(Deterministic::new(secure))
            .insecure_random(Deterministic::new(insecure))
            .insecure_random_seed(u128::from(seed));
    }
    // cg_clif's strict mode is read inside the guest.
    if let Ok(value) = std::env::var("CG_CLIF_WASM_STRICT") {
        wasi.env("CG_CLIF_WASM_STRICT", value);
    }
    // rustc sees the host filesystem at the same absolute paths, so the native
    // Wild process can consume the linker arguments unchanged.
    wasi.preopened_dir(Path::new("/"), "/", FsPerms::ReadWrite)
        .wt("preopening the host root as WASI /")?;

    let limits = options.limits.unwrap_or_default();
    let mut store = Store::new(engine, HostState::new(wasi.build_p1(), limits, recorder));
    if let Some(fuel) = options.fuel {
        store.set_fuel(fuel).wt("setting fuel")?;
    }
    if options.limits.is_some() {
        store.limiter(|state| &mut state.limits);
    }

    let mut linker: Linker<HostState<R>> = Linker::new(engine);
    p1::add_to_linker_sync(&mut linker, |state: &mut HostState<R>| &mut state.wasi)
        .wt("adding WASI p1 imports")?;
    add_playground_spawn(&mut linker, linker_path, log_prefix)?;
    let instance = linker
        .instantiate(&mut store, module)
        .wt("instantiating rustc.wasm")?;
    Ok(PreparedInstance {
        store,
        instance,
        stderr,
    })
}

//! The `cogwasm.next_request` and `cogwasm.finish` imports.
//!
//! A rustc guest built with the serve patch waits for an argument vector on
//! `next_request` instead of reading the process environment once. Every host
//! defines both imports: when serving is off, `next_request` returns 0 before
//! the guest reaches the loop, and `finish` does nothing.
//!
//! A request is a NUL-separated argument vector. `next_request` returns its
//! length, `-n` when the guest's buffer must be at least `n` bytes (the
//! request stays pending for the retry), or 0 when the channel is closed.
//! `finish` reports the request's exit code, its spawn records and the guest
//! memory size, so the host can decide to restart the session.

use crate::Wt;
use crate::mem::guest_memory;
use crate::spawn::{SpawnRecorder, SpawnSummary};
use crate::store::HostState;
use anyhow::Result;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use wasmtime::{Caller, Extern, Linker};

/// One completed request, sent to the host by `cogwasm.finish`.
#[derive(Clone, Debug)]
pub struct ServeFinish {
    /// rustc's exit code: 0 for a successful compilation, 1 for a rejection.
    pub code: i32,
    /// Guest-measured time from receiving the request to finishing it.
    pub elapsed: Duration,
    /// The `playground.spawn` calls this request made.
    pub spawns: Vec<SpawnSummary>,
    /// Current linear-memory size, so the host can bound per-request growth.
    pub memory_bytes: usize,
}

/// What the host observes from one session.
#[derive(Clone, Debug)]
pub enum ServeEvent {
    /// The guest was instantiated and is about to enter its request loop.
    Ready,
    /// One request finished.
    Finished(ServeFinish),
    /// `_start` returned or trapped. The session cannot serve another request.
    Ended(String),
}

/// The guest-facing side of a serving session.
///
/// The host keeps the sender side of `requests` and the receiver side of
/// `finished`; the guest's store owns this value.
pub struct ServeHooks {
    requests: Receiver<Vec<u8>>,
    finished: Sender<ServeEvent>,
    pending: Option<Vec<u8>>,
    started: Instant,
}

impl ServeHooks {
    /// Build the hooks around the two channel ends.
    #[must_use]
    pub fn new(requests: Receiver<Vec<u8>>, finished: Sender<ServeEvent>) -> Self {
        Self {
            requests,
            finished,
            pending: None,
            started: Instant::now(),
        }
    }

    /// The next request, or `End` once the host has dropped its sender.
    ///
    /// A request that does not fit stays pending, so the guest's retry with
    /// the returned required size observes the same bytes.
    pub fn next_request(&mut self, capacity: usize) -> NextRequest<'_> {
        if self.pending.is_none() {
            match self.requests.recv() {
                Ok(request) => {
                    self.pending = Some(request);
                    self.started = Instant::now();
                }
                Err(_) => return NextRequest::End,
            }
        }
        let request = self.pending.as_ref().expect("a request was just received");
        if request.len() > capacity {
            NextRequest::Need(request.len())
        } else {
            NextRequest::Write(request)
        }
    }

    /// Mark the pending request as written into guest memory.
    pub fn request_written(&mut self) {
        self.pending = None;
    }

    /// Report a finished request to the host.
    pub fn finish(&self, code: i32, spawns: Vec<SpawnSummary>, memory_bytes: usize) {
        let _ = self.finished.send(ServeEvent::Finished(ServeFinish {
            code,
            elapsed: self.started.elapsed(),
            spawns,
            memory_bytes,
        }));
    }

    /// Send the `Ready` event after instantiation.
    pub fn ready(&self) {
        let _ = self.finished.send(ServeEvent::Ready);
    }
}

/// The guest's view of the next request.
#[derive(Debug)]
pub enum NextRequest<'a> {
    /// The host closed the request channel.
    End,
    /// The buffer was too small; retry with at least this many bytes.
    Need(usize),
    /// Write these bytes into the guest's buffer.
    Write(&'a [u8]),
}

/// Encode an argument vector as the guest's NUL-separated request.
#[must_use]
pub fn encode_request(args: &[String]) -> Vec<u8> {
    args.join("\0").into_bytes()
}

/// Decode a NUL-separated request, dropping one trailing empty element.
#[must_use]
pub fn decode_request(bytes: &[u8]) -> Vec<String> {
    let mut args = String::from_utf8_lossy(bytes)
        .split('\0')
        .map(str::to_owned)
        .collect::<Vec<_>>();
    if args.last().is_some_and(String::is_empty) {
        args.pop();
    }
    args
}

/// Define the `cogwasm` imports on `linker`.
///
/// Serving is a per-store option, so the same linker can instantiate one-shot
/// and serving guests: without [`crate::StoreOptions::serve`],
/// `next_request` reports that there are no requests and `finish` does
/// nothing.
pub fn add_serve_imports<R: SpawnRecorder>(linker: &mut Linker<HostState<R>>) -> Result<()> {
    linker
        .func_wrap(
            "cogwasm",
            "next_request",
            |mut caller: Caller<'_, HostState<R>>, pointer: u32, capacity: u32| -> i32 {
                if caller.data().serve.is_none() {
                    return 0;
                }
                // No spawn can have happened while the guest is asking for
                // the next request, so clearing here starts each request with
                // an empty log even if a previous request was retried.
                caller.data().recorder().reset();

                let capacity = usize::try_from(capacity).unwrap_or(usize::MAX);
                let request = {
                    let Some(serve) = caller.data_mut().serve.as_mut() else {
                        return 0;
                    };
                    match serve.next_request(capacity) {
                        NextRequest::End => return 0,
                        NextRequest::Need(length) => {
                            return -i32::try_from(length).unwrap_or(i32::MAX);
                        }
                        NextRequest::Write(bytes) => bytes.to_vec(),
                    }
                };
                let length = match i32::try_from(request.len()) {
                    Ok(length) => length,
                    Err(_) => {
                        eprintln!("[cogwasm-runtime] request is larger than 2 GiB");
                        return 0;
                    }
                };
                let memory = match guest_memory(&mut caller) {
                    Ok(memory) => memory,
                    Err(error) => {
                        eprintln!("[cogwasm-runtime] next_request: {error:#}");
                        return 0;
                    }
                };
                if let Err(error) = memory.write(
                    &mut caller,
                    usize::try_from(pointer).unwrap_or(usize::MAX),
                    &request,
                ) {
                    eprintln!("[cogwasm-runtime] next_request: {error:#}");
                    return 0;
                }
                if let Some(serve) = caller.data_mut().serve.as_mut() {
                    serve.request_written();
                }
                length
            },
        )
        .wt("defining cogwasm::next_request")?;

    linker
        .func_wrap(
            "cogwasm",
            "finish",
            |mut caller: Caller<'_, HostState<R>>, code: i32| {
                if caller.data().serve.is_none() {
                    return;
                }
                let memory_bytes = match caller.get_export("memory") {
                    Some(Extern::Memory(memory)) => memory.data_size(&caller),
                    _ => 0,
                };
                let spawns = caller.data().recorder().summaries();
                if let Some(serve) = caller.data().serve.as_ref() {
                    serve.finish(code, spawns, memory_bytes);
                }
            },
        )
        .wt("defining cogwasm::finish")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_round_trip_the_argv() {
        let args = vec![
            "rustc".to_owned(),
            "--crate-name".to_owned(),
            "c".to_owned(),
        ];
        assert_eq!(encode_request(&args), b"rustc\0--crate-name\0c");
        assert_eq!(decode_request(&encode_request(&args)), args);
    }

    #[test]
    fn decode_drops_one_trailing_empty_element() {
        assert_eq!(decode_request(b"rustc\0--emit\0"), ["rustc", "--emit"]);
        assert_eq!(decode_request(b""), Vec::<String>::new());
        // An empty argument in the middle is a real argument, not padding.
        assert_eq!(decode_request(b"rustc\0\0-c"), ["rustc", "", "-c"]);
    }

    #[test]
    fn a_request_that_does_not_fit_stays_pending_for_the_retry() {
        let (requests, receiver) = std::sync::mpsc::channel();
        let (finished, _events) = std::sync::mpsc::channel();
        let mut hooks = ServeHooks::new(receiver, finished);
        let request = encode_request(&["rustc".to_owned(), "-o".to_owned(), "out.wasm".to_owned()]);
        requests.send(request.clone()).expect("sending a request");

        match hooks.next_request(4) {
            NextRequest::Need(needed) => assert_eq!(needed, request.len()),
            other => panic!("expected Need, got {other:?}"),
        }
        match hooks.next_request(request.len()) {
            NextRequest::Write(bytes) => assert_eq!(bytes, request.as_slice()),
            other => panic!("expected Write, got {other:?}"),
        }
        hooks.request_written();
        drop(requests);
        assert!(matches!(hooks.next_request(4), NextRequest::End));
    }

    #[test]
    fn a_closed_channel_ends_the_loop() {
        let (requests, receiver) = std::sync::mpsc::channel();
        let (finished, _events) = std::sync::mpsc::channel();
        let mut hooks = ServeHooks::new(receiver, finished);
        drop(requests);
        assert!(matches!(hooks.next_request(4096), NextRequest::End));
    }
}

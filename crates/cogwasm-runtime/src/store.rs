//! Store construction shared by the `CogWasm` hosts.
//!
//! The two hosts differ only in options: the product host inherits stderr,
//! uses the OS random source and clock, and sets neither fuel nor a resource
//! limiter; the benchmark host captures stderr, can pin randomness, the clock
//! and fuel, and tracks peak memory. [`StoreOptions`] defaults reproduce the
//! product host exactly.

use crate::Wt;
use crate::spawn::{SpawnRecorder, add_playground_spawn};
use anyhow::Result;
use std::path::Path;
use wasmtime::{Engine, Linker, Module, ResourceLimiter, Store};
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::p2::pipe::MemoryOutputPipe;
use wasmtime_wasi::{Deterministic, FsPerms, HostMonotonicClock, HostWallClock, WasiCtxBuilder};

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
/// inherited, randomness and the clock come from the OS, and there is no fuel
/// or resource limiter.
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
    /// Give WASI a fixed wall and monotonic clock and a fixed `TZ=UTC`
    /// instead of the host's.
    ///
    /// Fuel runs need every input to the guest to be identical. rustc formats
    /// a wall-clock timestamp into the name of its ICE-report file, so an
    /// advancing clock (or a different host time zone) changes the fuel cost
    /// of the formatter. Timed runs keep the real clock and time zone.
    pub deterministic_clock: bool,
}

/// Wall-clock instant frozen for deterministic fuel runs:
/// 2020-01-01T00:00:00Z.
const FIXED_WALL_NS: u64 = 1_577_836_800_000_000_000;

/// Monotonic clock value frozen for deterministic fuel runs.
const FIXED_MONOTONIC_NS: u64 = 0;

/// A wall clock frozen at a fixed instant.
struct FixedWallClock(u64);

impl HostWallClock for FixedWallClock {
    fn resolution(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(1)
    }

    fn now(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.0)
    }
}

/// A monotonic clock frozen at a fixed tick.
struct FixedMonotonicClock(u64);

impl HostMonotonicClock for FixedMonotonicClock {
    fn resolution(&self) -> u64 {
        1
    }

    fn now(&self) -> u64 {
        self.0
    }
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
    if options.deterministic_clock {
        // These constants are part of the guest's input, so frozen values keep
        // every fuel sample's input identical.
        wasi.env("TZ", "UTC")
            .wall_clock(FixedWallClock(FIXED_WALL_NS))
            .monotonic_clock(FixedMonotonicClock(FIXED_MONOTONIC_NS));
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SummaryRecorder;

    /// A one-page module whose `_start` calls `clock_time_get` for the wall
    /// clock (id 0) and the monotonic clock (id 1), leaving the two returned
    /// timestamps at offsets 0 and 8 in memory.
    const CLOCK_MODULE: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, // magic and version
        0x01, 0x0b, 0x02, // type section: two types
        0x60, 0x00, 0x00, // () -> ()
        0x60, 0x03, 0x7f, 0x7e, 0x7f, 0x01, 0x7f, // (i32, i64, i32) -> i32
        0x02, 0x29, 0x01, // import section: one import
        0x16, b'w', b'a', b's', b'i', b'_', b's', b'n', b'a', b'p', b's', b'h', b'o', b't', b'_',
        b'p', b'r', b'e', b'v', b'i', b'e', b'w', b'1', // wasi_snapshot_preview1
        0x0e, b'c', b'l', b'o', b'c', b'k', b'_', b't', b'i', b'm', b'e', b'_', b'g', b'e', b't',
        0x00, 0x01, // function, type 1
        0x03, 0x02, 0x01, 0x00, // function section: one function of type 0
        0x05, 0x03, 0x01, 0x00, 0x01, // memory section: one page
        0x07, 0x13, 0x02, // export section: two exports
        0x06, b'm', b'e', b'm', b'o', b'r', b'y', 0x02, 0x00, // memory 0
        0x06, b'_', b's', b't', b'a', b'r', b't', 0x00, 0x01, // _start (function 1)
        0x0a, 0x16, 0x01, // code section: one body
        0x14, 0x00, // body size, no locals
        0x41, 0x00, 0x42, 0x00, 0x41, 0x00, 0x10, 0x00, 0x1a, // clock_time_get(0, 0, 0)
        0x41, 0x01, 0x42, 0x00, 0x41, 0x08, 0x10, 0x00, 0x1a, // clock_time_get(1, 0, 8)
        0x0b, // end
    ];

    #[test]
    fn deterministic_clock_is_frozen_across_stores() -> Result<()> {
        let engine = Engine::default();
        let module = Module::new(&engine, CLOCK_MODULE).wt("compiling clock test module")?;
        let mut samples = Vec::new();
        for _ in 0..2 {
            let options = StoreOptions {
                capture_stderr: true,
                deterministic_clock: true,
                ..StoreOptions::default()
            };
            let prepared = prepare_instance(
                &engine,
                &module,
                &[],
                Path::new("/nonexistent/clock-test"),
                "[test]",
                options,
                SummaryRecorder::default(),
            )?;
            let mut store = prepared.store;
            prepared
                .instance
                .get_typed_func::<(), ()>(&mut store, "_start")
                .wt("clock test _start export")?
                .call(&mut store, ())
                .wt("calling clock test _start")?;
            let memory = prepared
                .instance
                .get_memory(&mut store, "memory")
                .expect("clock test module exports memory");
            let mut bytes = [0u8; 16];
            memory.read(&store, 0, &mut bytes)?;
            let wall = u64::from_le_bytes(bytes[0..8].try_into().expect("eight bytes"));
            let monotonic = u64::from_le_bytes(bytes[8..16].try_into().expect("eight bytes"));
            samples.push((wall, monotonic));
        }
        assert_eq!(samples[0], samples[1], "two stores saw different clocks");
        assert_eq!(samples[0], (FIXED_WALL_NS, FIXED_MONOTONIC_NS));
        Ok(())
    }
}

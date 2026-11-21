//! Host-side supervisor for the `CogWasm` compile pipeline.
//!
//! The compiler stays a pure function of (source, options); this crate owns the
//! parts that are policy rather than compilation:
//!
//! - the option space ([`CompileOptions`]) and its named presets ([`Preset`]);
//! - a content-addressed artifact store ([`ArtifactStore`]);
//! - the scheduler seam that drops a build when a newer request supersedes it
//!   ([`Supervisor`]);
//! - the swap seam that replaces a running instance at a quiescent point
//!   ([`Swapper`]);
//! - the real backends: [`RealCompiler`] spawns `cogwasm-host raw` per job, and
//!   [`ResidentCompiler`] keeps one loaded `rustc.wasm` for the process and
//!   instantiates a fresh guest per job.
//!
//! [M1] ships the contract, a deterministic fake compiler and a fake runtime, so
//! the scheduling, caching and swap ordering are tested without a toolchain.
//! The real compiler backend and the Wasmtime runtime land in M2 and M3; see
//! `../../../docs/notes/supervisor.md`.
//!
//! [M1]: ../../../docs/notes/supervisor.md

pub mod compiler;
pub mod job;
pub mod real;
pub mod resident;
pub mod runtime;
pub mod select;
pub mod store;
pub mod supervisor;
pub mod swap;
pub mod wasmtime_runtime;
pub mod workspace;

pub use compiler::{CompileError, CompileOutput, Compiler, FakeCompiler};
pub use job::{
    Budgets, CompileOptions, CrateKind, Emit, ExternSpec, JobSpec, OptLevel, Preset, SCHEMA,
    SourceSpec, Strip,
};
pub use real::{RealCompiler, RealCompilerConfig};
pub use resident::ResidentCompiler;
pub use runtime::{FakeRuntime, Instance, Runtime};
pub use select::{Measurement, Objective, Scoreboard};
pub use store::{Artifact, ArtifactMeta, ArtifactStore};
pub use supervisor::{Begin, BuildOutcome, BuildTicket, Supervisor};
pub use swap::Swapper;
pub use wasmtime_runtime::{WasmtimeRuntime, wat_artifact};
pub use workspace::{CrateBuild, CrateDef, Workspace};

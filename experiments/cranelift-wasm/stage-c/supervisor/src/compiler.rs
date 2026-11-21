//! The compiler seam.
//!
//! The supervisor does not know how a build happens. [`Compiler`] is the seam
//! the real backend (M2, the stage-C host driving `rustc.wasm` and Wild) plugs
//! into; [`FakeCompiler`] is the deterministic implementation the M1 tests use.

use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use crate::job::{JobSpec, Tier};

/// Why a build did not produce an artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompileError {
    /// The compiler ran and rejected the source.
    Rejected(String),
    /// The compiler could not be started or its environment is incomplete.
    Unavailable(String),
}

impl fmt::Display for CompileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rejected(reason) => write!(formatter, "compile rejected: {reason}"),
            Self::Unavailable(reason) => write!(formatter, "compiler unavailable: {reason}"),
        }
    }
}

impl std::error::Error for CompileError {}

/// Bytes and timings produced by one build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileOutput {
    pub bytes: Vec<u8>,
    /// Wall time of the build, when the backend can measure it.
    pub compile_ms: Option<u64>,
    /// Link time, when the backend reports it separately from compilation.
    pub link_ms: Option<u64>,
}

impl CompileOutput {
    /// Output without timings, for backends that do not measure them.
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            compile_ms: None,
            link_ms: None,
        }
    }
}

/// Turns a job into artifact bytes.
///
/// Implementations must be safe to call from more than one thread; the
/// supervisor may run several tiers at once.
pub trait Compiler: Send + Sync {
    /// A stable identity for cache keys (compiler version or binary hash).
    fn id(&self) -> &str;

    /// Compile `job` to an artifact under `key`.
    ///
    /// # Errors
    ///
    /// Returns [`CompileError`] when the build fails or cannot start.
    fn compile(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError>;
}

/// Lets a test hold a build open so it can submit a competing request.
#[derive(Debug, Default)]
pub struct Gate {
    state: Mutex<bool>,
    opened: Condvar,
}

impl Gate {
    /// A closed gate: [`Gate::wait`] blocks until [`Gate::open`].
    #[must_use]
    pub fn closed() -> Self {
        Self::default()
    }

    /// Block until the gate is opened.
    pub fn wait(&self) {
        let mut open = self.state.lock().expect("gate lock is not poisoned");
        while !*open {
            open = self
                .opened
                .wait(open)
                .expect("gate lock is not poisoned while waiting");
        }
    }

    /// Release every waiter and every later call.
    pub fn open(&self) {
        *self.state.lock().expect("gate lock is not poisoned") = true;
        self.opened.notify_all();
    }
}

/// A deterministic compiler for tests: the artifact is a pure function of the
/// key, and invocations are counted.
#[derive(Debug)]
pub struct FakeCompiler {
    id: String,
    invocations: AtomicUsize,
    gate: Option<Arc<Gate>>,
    reject_tier: Option<Tier>,
}

impl FakeCompiler {
    /// A fake compiler called `fake-0`.
    #[must_use]
    pub fn new() -> Self {
        Self::with_id("fake-0")
    }

    /// A fake compiler with an explicit identity, so cache invalidation can be
    /// tested.
    #[must_use]
    pub fn with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            invocations: AtomicUsize::new(0),
            gate: None,
            reject_tier: None,
        }
    }

    /// Block inside [`Compiler::compile`] until `gate` opens.
    #[must_use]
    pub fn through_gate(mut self, gate: Arc<Gate>) -> Self {
        self.gate = Some(gate);
        self
    }

    /// Reject every build for `tier`, as a broken optimising tier would.
    #[must_use]
    pub fn rejecting(mut self, tier: Tier) -> Self {
        self.reject_tier = Some(tier);
        self
    }

    /// How many times a build was attempted.
    #[must_use]
    pub fn invocations(&self) -> usize {
        self.invocations.load(Ordering::SeqCst)
    }
}

impl Default for FakeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler for FakeCompiler {
    fn id(&self) -> &str {
        &self.id
    }

    fn compile(&self, job: &JobSpec, key: &str) -> Result<CompileOutput, CompileError> {
        self.invocations.fetch_add(1, Ordering::SeqCst);
        if let Some(gate) = &self.gate {
            gate.wait();
        }
        if self.reject_tier == Some(job.tier) {
            return Err(CompileError::Rejected(format!(
                "the {} tier is broken in this fake",
                job.tier.as_str()
            )));
        }
        Ok(CompileOutput::new(
            format!(
                "fake artifact\nkey: {key}\ntier: {}\nsource: {}\n",
                job.tier.as_str(),
                job.source.path.display()
            )
            .into_bytes(),
        ))
    }
}

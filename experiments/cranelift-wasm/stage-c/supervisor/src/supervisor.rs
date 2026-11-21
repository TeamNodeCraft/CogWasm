//! The scheduler seam: cache lookup, supersession and the synchronous build.
//!
//! A build is a ticket. [`Supervisor::begin`] returns a cached artifact or a
//! ticket carrying the key and a generation; [`Supervisor::finish`] stores the
//! result only if no newer request for the same key has started meanwhile.
//! That lets a background tier finish late without overwriting a newer build,
//! and it is the behaviour the M1 tests pin down.

use std::collections::HashMap;
use std::sync::Mutex;

use anyhow::{Result, anyhow};

use crate::compiler::{CompileOutput, Compiler};
use crate::job::JobSpec;
use crate::store::{Artifact, ArtifactStore};

/// The result of a successful build request.
#[derive(Debug, Clone)]
pub enum BuildOutcome {
    /// The store already had the artifact.
    Hit(Artifact),
    /// The compiler produced the artifact during this request.
    Built(Artifact),
}

impl BuildOutcome {
    /// The artifact either way.
    #[must_use]
    pub fn artifact(&self) -> &Artifact {
        match self {
            Self::Hit(artifact) | Self::Built(artifact) => artifact,
        }
    }

    /// Whether the artifact came from the cache.
    #[must_use]
    pub fn was_hit(&self) -> bool {
        matches!(self, Self::Hit(_))
    }
}

/// What [`Supervisor::begin`] decided.
#[derive(Debug)]
pub enum Begin {
    /// No compile needed.
    Cached(Artifact),
    /// A compile is needed; hand the ticket back to [`Supervisor::finish`].
    Build(BuildTicket),
}

/// A reservation for one build of one key.
#[derive(Debug, Clone)]
pub struct BuildTicket {
    pub key: String,
    pub generation: u64,
}

/// Cache-aware build orchestration.
#[derive(Debug)]
pub struct Supervisor<C: Compiler> {
    store: ArtifactStore,
    compiler: C,
    latest: Mutex<HashMap<String, u64>>,
}

impl<C: Compiler> Supervisor<C> {
    /// A supervisor over `store` and `compiler`.
    pub fn new(store: ArtifactStore, compiler: C) -> Self {
        Self {
            store,
            compiler,
            latest: Mutex::new(HashMap::new()),
        }
    }

    /// The compiler in use.
    #[must_use]
    pub fn compiler(&self) -> &C {
        &self.compiler
    }

    /// The artifact store in use.
    #[must_use]
    pub fn store(&self) -> &ArtifactStore {
        &self.store
    }

    /// Look up a job without compiling.
    ///
    /// # Errors
    ///
    /// Returns an error when the job is invalid or the store cannot be read.
    pub fn cached(&self, job: &JobSpec) -> Result<Option<Artifact>> {
        job.validate()?;
        let key = ArtifactStore::key_for(job, self.compiler.id())?;
        self.store.get(&key)
    }

    /// Check the cache, or reserve a build.
    ///
    /// # Errors
    ///
    /// Returns an error when the job is invalid, the source cannot be read or
    /// the store cannot be read.
    pub fn begin(&self, job: &JobSpec) -> Result<Begin> {
        job.validate()?;
        let key = ArtifactStore::key_for(job, self.compiler.id())?;
        if let Some(artifact) = self.store.get(&key)? {
            return Ok(Begin::Cached(artifact));
        }
        let mut latest = self
            .latest
            .lock()
            .map_err(|_| anyhow!("the supersession log is poisoned"))?;
        let generation = latest.entry(key.clone()).or_insert(0);
        *generation += 1;
        Ok(Begin::Build(BuildTicket {
            key,
            generation: *generation,
        }))
    }

    /// Store the result of a ticket, unless a newer request superseded it.
    ///
    /// # Errors
    ///
    /// Returns an error when the artifact cannot be described or written.
    pub fn finish(
        &self,
        job: &JobSpec,
        ticket: &BuildTicket,
        output: &CompileOutput,
    ) -> Result<Option<Artifact>> {
        {
            let latest = self
                .latest
                .lock()
                .map_err(|_| anyhow!("the supersession log is poisoned"))?;
            if latest.get(&ticket.key).copied() != Some(ticket.generation) {
                return Ok(None);
            }
        }
        let artifact = self.store.artifact_for(job, self.compiler.id(), output)?;
        self.store.put(&artifact)?;
        Ok(Some(artifact))
    }

    /// Compile if needed, storing the artifact.
    ///
    /// # Errors
    ///
    /// Returns an error when the job is invalid, the source cannot be read,
    /// the compiler fails, or a concurrent request supersedes this build.
    pub fn build(&self, job: &JobSpec) -> Result<BuildOutcome> {
        match self.begin(job)? {
            Begin::Cached(artifact) => Ok(BuildOutcome::Hit(artifact)),
            Begin::Build(ticket) => {
                let output = self
                    .compiler
                    .compile(job, &ticket.key)
                    .map_err(anyhow::Error::new)?;
                match self.finish(job, &ticket, &output)? {
                    Some(artifact) => Ok(BuildOutcome::Built(artifact)),
                    None => Err(anyhow!("build for {} was superseded", ticket.key)),
                }
            }
        }
    }
}

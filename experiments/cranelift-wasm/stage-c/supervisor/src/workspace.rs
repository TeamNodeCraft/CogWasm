//! Multi-crate workspaces (M5).
//!
//! A workspace manifest names crates and their dependencies. The scheduler
//! builds them in dependency order: a crate that others depend on becomes an
//! `rlib`, the roots become `cdylib` plugins. Each crate is a separate
//! [`JobSpec`], so each has its own cache entry; an unchanged crate hits the
//! cache, and a changed `rlib` changes its dependents' keys because the key
//! covers the content hash of every extern.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::compiler::Compiler;
use crate::job::{CompileOptions, CrateKind, ExternSpec, JobSpec, SourceSpec};
use crate::store::{Artifact, ArtifactStore};
use crate::supervisor::Supervisor;

/// One crate in a workspace manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrateDef {
    pub name: String,
    pub source: PathBuf,
    #[serde(default)]
    pub deps: Vec<String>,
}

/// A workspace manifest: the crates and their dependencies.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(default)]
    pub crates: Vec<CrateDef>,
}

/// What happened to one crate.
#[derive(Debug, Clone, Serialize)]
pub struct CrateBuild {
    pub name: String,
    pub kind: CrateKind,
    pub key: String,
    pub bytes: u64,
    pub compile_ms: Option<u64>,
    pub hit: bool,
}

impl Workspace {
    /// Read a workspace manifest from JSON.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or parsed.
    pub fn from_json(path: &Path) -> Result<Self> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    /// Check names, dependencies and cycles.
    ///
    /// # Errors
    ///
    /// Returns an error for a duplicate name, an unknown dependency or a
    /// dependency cycle.
    pub fn validate(&self) -> Result<()> {
        if self.crates.is_empty() {
            bail!("the workspace has no crates");
        }
        let mut names = BTreeSet::new();
        for item in &self.crates {
            if item.name.is_empty() {
                bail!("a crate has an empty name");
            }
            if !names.insert(item.name.as_str()) {
                bail!("duplicate crate {:?}", item.name);
            }
        }
        for item in &self.crates {
            for dep in &item.deps {
                if !names.contains(dep.as_str()) {
                    bail!("crate {:?} depends on unknown {:?}", item.name, dep);
                }
            }
        }
        self.plan()?;
        Ok(())
    }

    /// The crates in dependency order.
    ///
    /// # Errors
    ///
    /// Returns an error when the dependencies contain a cycle.
    pub fn plan(&self) -> Result<Vec<&CrateDef>> {
        let by_name: BTreeMap<&str, &CrateDef> = self
            .crates
            .iter()
            .map(|item| (item.name.as_str(), item))
            .collect();
        let mut indegree: BTreeMap<&str, usize> = by_name.keys().map(|name| (*name, 0)).collect();
        let mut dependents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for item in &self.crates {
            for dep in &item.deps {
                *indegree.entry(item.name.as_str()).or_insert(0) += 1;
                dependents
                    .entry(dep.as_str())
                    .or_default()
                    .push(item.name.as_str());
            }
        }
        let mut ready: BTreeSet<&str> = indegree
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(name, _)| *name)
            .collect();
        let mut order = Vec::new();
        while let Some(name) = ready.pop_first() {
            let item = by_name
                .get(name)
                .copied()
                .with_context(|| format!("crate {name:?} is missing"))?;
            order.push(item);
            if let Some(children) = dependents.get(name) {
                for child in children {
                    release(child, &mut indegree, &mut ready)?;
                }
            }
        }
        if order.len() != self.crates.len() {
            let done: BTreeSet<&str> = order.iter().map(|item| item.name.as_str()).collect();
            let stuck: Vec<&str> = self
                .crates
                .iter()
                .map(|item| item.name.as_str())
                .filter(|name| !done.contains(name))
                .collect();
            bail!("dependency cycle among {stuck:?}");
        }
        Ok(order)
    }

    /// Build every crate in dependency order.
    ///
    /// # Errors
    ///
    /// Returns an error when the workspace is invalid or any build fails.
    pub fn build<C: Compiler>(
        &self,
        supervisor: &Supervisor<C>,
        options: &CompileOptions,
        work_root: &Path,
    ) -> Result<Vec<CrateBuild>> {
        self.validate()?;
        let mut artifacts: BTreeMap<String, Artifact> = BTreeMap::new();
        let mut builds = Vec::new();
        for item in self.plan()? {
            let kind = if self.is_dependency(&item.name) {
                CrateKind::Rlib
            } else {
                CrateKind::Cdylib
            };
            let mut job = JobSpec::new(
                SourceSpec::new(item.source.clone(), item.name.clone()),
                options.clone(),
            );
            job.kind = kind;
            job.externs = item
                .deps
                .iter()
                .map(|dep| {
                    let artifact = artifacts
                        .get(dep)
                        .with_context(|| format!("{} was not built before {}", dep, item.name))?;
                    Ok(ExternSpec {
                        name: dep.clone(),
                        path: supervisor.store().artifact_path(artifact),
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            if options.incremental {
                let key = ArtifactStore::key_for(&job, supervisor.compiler().id())?;
                job.incremental_dir = Some(work_root.join("incremental").join(key));
            }
            let outcome = supervisor.build(&job)?;
            let artifact = outcome.artifact();
            builds.push(CrateBuild {
                name: item.name.clone(),
                kind,
                key: artifact.key.clone(),
                bytes: artifact.meta.bytes,
                compile_ms: artifact.meta.compile_ms,
                hit: outcome.was_hit(),
            });
            artifacts.insert(item.name.clone(), artifact.clone());
        }
        Ok(builds)
    }

    /// Whether `name` is a dependency of any crate.
    #[must_use]
    pub fn is_dependency(&self, name: &str) -> bool {
        self.crates
            .iter()
            .any(|item| item.deps.iter().any(|dep| dep == name))
    }
}

fn release<'a>(
    child: &'a str,
    indegree: &mut BTreeMap<&'a str, usize>,
    ready: &mut BTreeSet<&'a str>,
) -> Result<()> {
    let count = indegree
        .get_mut(child)
        .with_context(|| format!("crate {child:?} is missing"))?;
    *count = count
        .checked_sub(1)
        .with_context(|| format!("crate {child:?} has an inconsistent dependency count"))?;
    if *count == 0 {
        ready.insert(child);
    }
    Ok(())
}

/// A convenience for hosts: the extern path a dependent would pass.
impl ArtifactStore {
    /// Where `artifact` lives on disk.
    #[must_use]
    pub fn artifact_path(&self, artifact: &Artifact) -> PathBuf {
        self.root()
            .join(&artifact.key)
            .join(artifact.meta.file_name())
    }
}

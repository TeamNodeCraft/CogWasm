//! CLI for the reference supervisor.
//!
//! M1 wires the cache, the tier map and the fake compiler; `build` therefore
//! reports what the real backend will do in M2 without running a toolchain.

use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use cogwasm_supervisor::{ArtifactStore, FakeCompiler, JobSpec, Supervisor};

#[derive(Parser)]
#[command(
    name = "cogwasm-supervisor",
    about = "Reference host-side compile supervisor"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print the cache key a job would be stored under.
    Key {
        #[arg(long)]
        job: PathBuf,
        /// Compiler identity used in the key.
        #[arg(long, default_value = "fake-0")]
        compiler: String,
    },
    /// Cache-check or build a job with the reference (fake) compiler.
    Build {
        #[arg(long)]
        job: PathBuf,
        #[arg(long)]
        store: PathBuf,
        /// Compiler identity used in the key.
        #[arg(long, default_value = "fake-0")]
        compiler: String,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Key { job, compiler } => {
            let job = load_job(&job)?;
            let key = ArtifactStore::key_for(&job, &compiler)?;
            println!("{key}");
        }
        Command::Build {
            job,
            store,
            compiler,
        } => {
            let job = load_job(&job)?;
            let supervisor =
                Supervisor::new(ArtifactStore::new(store), FakeCompiler::with_id(compiler));
            let outcome = supervisor.build(&job)?;
            let artifact = outcome.artifact();
            println!(
                "{} {} ({} bytes)",
                if outcome.was_hit() { "hit" } else { "built" },
                artifact.key,
                artifact.bytes.len()
            );
        }
    }
    Ok(())
}

fn load_job(path: &Path) -> Result<JobSpec> {
    let file = File::open(path).with_context(|| format!("opening {}", path.display()))?;
    serde_json::from_reader(file).with_context(|| format!("parsing {}", path.display()))
}

//! Static and dynamic analysis of `CogWasm` output against LLVM `-O2`.
//!
//! The tool has three jobs:
//!   1. rewrite a wasm module so every basic block bumps an exported `i64`
//!      counter (the original instructions are untouched);
//!   2. execute the instrumented module in Wasmtime and read the counters back;
//!   3. report static sizes and per-function bytes.
//!
//! The analysis scripts in `bench/gap` turn the JSON into the report.

mod instrument;
mod measure;
mod sizes;
mod wat;

use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .context("usage: gap-tool <instrument|measure|sizes|wat> ...")?;
    let rest: Vec<String> = args.collect();
    match command.as_str() {
        "instrument" => instrument::run(&rest),
        "measure" => measure::run(&rest),
        "sizes" => sizes::run(&rest),
        "wat" => wat::run(&rest),
        other => bail!("unknown gap-tool command: {other}"),
    }
}

/// Parse `--name value` or `--name=value` style arguments without pulling in a
/// command-line parser: the tool has a handful of options and is not shipped.
pub(crate) fn option(rest: &[String], name: &str) -> Result<Option<String>> {
    let mut found = None;
    let mut index = 0;
    while index < rest.len() {
        let item = &rest[index];
        if item == name {
            let value = rest
                .get(index + 1)
                .with_context(|| format!("{name} needs a value"))?;
            found = Some(value.clone());
            index += 2;
        } else if let Some(value) = item.strip_prefix(&format!("{name}=")) {
            found = Some(value.to_owned());
            index += 1;
        } else {
            index += 1;
        }
    }
    Ok(found)
}

pub(crate) fn required_option(rest: &[String], name: &str) -> Result<String> {
    option(rest, name)?.with_context(|| format!("missing required option {name}"))
}

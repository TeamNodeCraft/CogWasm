//! Print a module as WAT, optionally only one function's text.

use crate::{option, required_option};
use anyhow::{Context, Result};

pub fn run(rest: &[String]) -> Result<()> {
    let wasm = required_option(rest, "--wasm")?;
    let output = required_option(rest, "--out")?;
    let text = wasmprinter::print_file(&wasm).with_context(|| format!("printing {wasm}"))?;
    if let Some(name) = option(rest, "--function")? {
        let body = extract_function(&text, &name)
            .with_context(|| format!("function {name} not found in {wasm}"))?;
        std::fs::write(&output, body)?;
    } else {
        std::fs::write(&output, text)?;
    }
    Ok(())
}

/// Return the parenthesised `(func ...)` form for `name`, balancing nested
/// parentheses. `wasmprinter` prints one `(func $name ...)` per function.
fn extract_function(text: &str, name: &str) -> Option<String> {
    let needle = format!("(func ${name} ");
    let start = text.find(&needle)?;
    let mut depth = 0i32;
    for (offset, character) in text[start..].char_indices() {
        match character {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    let end = start + offset + 1;
                    return Some(text[start..end].to_owned());
                }
            }
            _ => {}
        }
    }
    None
}

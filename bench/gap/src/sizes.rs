//! Static size breakdown of wasm modules: sections, functions and data.

use crate::{option, required_option, write_output};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::collections::BTreeMap;
use wasmparser::{Name, Parser, Payload};

#[derive(Serialize)]
struct SectionSize {
    id: u8,
    name: String,
    bytes: u64,
}

#[derive(Serialize)]
struct FunctionSize {
    index: u32,
    name: String,
    demangled: String,
    bytes: u64,
    static_instructions: u64,
}

#[derive(Serialize)]
struct OnlyInFirst {
    name: String,
    demangled: String,
    bytes: u64,
}

#[derive(Serialize)]
struct SizesResult {
    wasm: String,
    bytes: u64,
    sections: Vec<SectionSize>,
    functions: Vec<FunctionSize>,
    data_segments: usize,
    data_bytes: u64,
    custom_sections: u64,
    compare: Option<String>,
    only_in_first: Vec<OnlyInFirst>,
    common_functions: usize,
    first_function_bytes: u64,
    second_function_bytes: u64,
    second_sections: Vec<SectionSize>,
    second_data_segments: usize,
    second_data_bytes: u64,
    second_custom_sections: u64,
}

pub fn run(rest: &[String]) -> Result<()> {
    let wasm = required_option(rest, "--wasm")?;
    let out = option(rest, "--out")?;
    let compare = option(rest, "--compare")?;
    let first = analyze(&wasm)?;
    let mut only_in_first = Vec::new();
    let mut common = 0usize;
    let mut second_function_bytes = 0u64;
    let mut second_sections = Vec::new();
    let mut second_data_segments = 0usize;
    let mut second_data_bytes = 0u64;
    let mut second_custom_sections = 0u64;
    if let Some(second_path) = &compare {
        let second = analyze(second_path)?;
        second_function_bytes = second.functions.iter().map(|function| function.bytes).sum();
        second_sections = second.sections;
        second_data_segments = second.data_segments;
        second_data_bytes = second.data_bytes;
        second_custom_sections = second.custom_sections;
        let second_names: std::collections::HashSet<&str> = second
            .functions
            .iter()
            .map(|function| function.demangled.as_str())
            .collect();
        for function in &first.functions {
            if second_names.contains(function.demangled.as_str()) {
                common += 1;
            } else {
                only_in_first.push(OnlyInFirst {
                    name: function.name.clone(),
                    demangled: function.demangled.clone(),
                    bytes: function.bytes,
                });
            }
        }
        only_in_first.sort_by_key(|function| std::cmp::Reverse(function.bytes));
    }
    let result = SizesResult {
        wasm: wasm.clone(),
        bytes: first.bytes,
        sections: first.sections,
        functions: first.functions,
        data_segments: first.data_segments,
        data_bytes: first.data_bytes,
        custom_sections: first.custom_sections,
        compare,
        only_in_first,
        common_functions: common,
        first_function_bytes: first.function_bytes,
        second_function_bytes,
        second_sections,
        second_data_segments,
        second_data_bytes,
        second_custom_sections,
    };
    let text = serde_json::to_string_pretty(&result)?;
    if let Some(path) = out {
        write_output(&path, text.as_bytes())?;
    } else {
        println!("{text}");
    }
    Ok(())
}

struct Analysis {
    bytes: u64,
    sections: Vec<SectionSize>,
    functions: Vec<FunctionSize>,
    data_segments: usize,
    data_bytes: u64,
    custom_sections: u64,
    function_bytes: u64,
}

#[expect(
    clippy::excessive_nesting,
    reason = "the payload loop mirrors the wasm section order"
)]
fn analyze(path: &str) -> Result<Analysis> {
    let bytes = std::fs::read(path).with_context(|| format!("reading {path}"))?;
    let mut names = BTreeMap::new();
    for payload in Parser::new(0).parse_all(&bytes) {
        if let Payload::CustomSection(section) = payload?
            && let wasmparser::KnownCustom::Name(reader) = section.as_known()
        {
            for subsection in reader {
                if let Name::Function(map) = subsection? {
                    for naming in map {
                        let naming = naming?;
                        names.insert(naming.index, naming.name.to_owned());
                    }
                }
            }
        }
    }
    let mut functions = Vec::new();
    let mut data_segments = 0usize;
    let mut data_bytes = 0u64;
    let mut defined_index = 0u32;
    let imported = count_imported_functions(&bytes)?;
    for payload in Parser::new(0).parse_all(&bytes) {
        match payload? {
            Payload::CodeSectionEntry(body) => {
                let index = imported + defined_index;
                defined_index += 1;
                let name = names
                    .get(&index)
                    .cloned()
                    .unwrap_or_else(|| format!("func_{index}"));
                let mut static_instructions = 0u64;
                let mut reader = body.get_operators_reader()?;
                while !reader.eof() {
                    reader.read()?;
                    static_instructions += 1;
                }
                functions.push(FunctionSize {
                    index,
                    demangled: demangle(&name),
                    name,
                    bytes: body.range().end - body.range().start,
                    static_instructions,
                });
            }
            Payload::DataSection(section) => {
                data_segments = section.count() as usize;
                for datum in section {
                    let datum = datum?;
                    data_bytes += u64::try_from(datum.data.len()).unwrap_or(0);
                }
            }
            _ => {}
        }
    }
    functions.sort_by_key(|function| function.index);
    let mut sections = Vec::new();
    let mut custom_sections = 0u64;
    for section in raw_sections(&bytes)? {
        let name = section_name(section.0).to_owned();
        if section.0 == 0 {
            custom_sections += u64::try_from(section.1.len()).unwrap_or(0);
        }
        sections.push(SectionSize {
            id: section.0,
            name,
            bytes: u64::try_from(section.1.len()).unwrap_or(0),
        });
    }
    let function_bytes = functions.iter().map(|function| function.bytes).sum();
    Ok(Analysis {
        bytes: u64::try_from(bytes.len()).unwrap_or(u64::MAX),
        sections,
        functions,
        data_segments,
        data_bytes,
        custom_sections,
        function_bytes,
    })
}

fn count_imported_functions(bytes: &[u8]) -> Result<u32> {
    let mut count = 0;
    for payload in Parser::new(0).parse_all(bytes) {
        if let Payload::ImportSection(section) = payload? {
            for import in section.into_imports() {
                if matches!(import?.ty, wasmparser::TypeRef::Func(_)) {
                    count += 1;
                }
            }
        }
    }
    Ok(count)
}

fn raw_sections(bytes: &[u8]) -> Result<Vec<(u8, Vec<u8>)>> {
    if bytes.len() < 8 || &bytes[..4] != b"\0asm" {
        bail!("not a wasm module");
    }
    let mut offset = 8;
    let mut sections = Vec::new();
    while offset < bytes.len() {
        let id = bytes[offset];
        offset += 1;
        let (length, next) = read_leb(bytes, offset)?;
        offset = next;
        let end = offset
            .checked_add(usize::try_from(length)?)
            .context("section length overflow")?;
        sections.push((id, bytes[offset..end].to_vec()));
        offset = end;
    }
    Ok(sections)
}

fn read_leb(bytes: &[u8], offset: usize) -> Result<(u64, usize)> {
    let mut value = 0u64;
    let mut shift = 0;
    let mut index = offset;
    loop {
        let byte = *bytes.get(index).context("truncated LEB128")?;
        index += 1;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok((value, index));
        }
        shift += 7;
        if shift >= 64 {
            bail!("LEB128 too long");
        }
    }
}

fn section_name(id: u8) -> &'static str {
    match id {
        0 => "custom",
        1 => "type",
        2 => "import",
        3 => "function",
        4 => "table",
        5 => "memory",
        6 => "global",
        7 => "export",
        8 => "start",
        9 => "element",
        10 => "code",
        11 => "data",
        12 => "data_count",
        13 => "tag",
        _ => "unknown",
    }
}

fn demangle(name: &str) -> String {
    rustc_demangle::try_demangle(name)
        .map_or_else(|_| name.to_owned(), |value| format!("{value:#}"))
}

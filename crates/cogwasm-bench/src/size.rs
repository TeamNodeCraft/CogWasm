//! Wasm size analysis and the custom-section stripping ported from
//! `ci/strip-wasm.py`.

use anyhow::{Context, Result, bail};
use flate2::Compression;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use std::io::Write as _;
use wasmparser::{Parser, Payload};

/// Size of one raw wasm section, excluding its id and length prefix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SectionSize {
    pub id: u8,
    pub name: String,
    pub bytes: u64,
}

/// The per-build binary-size breakdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SizeBreakdown {
    pub total_bytes: u64,
    pub code_bytes: u64,
    pub data_bytes: u64,
    pub custom_bytes: u64,
    pub name_bytes: u64,
    pub import_bytes: u64,
    pub export_bytes: u64,
    pub functions: u32,
    pub data_segments: u64,
    pub sections: Vec<SectionSize>,
}

/// Remove every custom section (id 0), copying every other section byte for
/// byte, in order. This is the Rust port of `ci/strip-wasm.py`.
///
/// # Errors
///
/// Returns an error when the input is not a wasm module or a section runs past
/// the end of the file.
pub fn strip_custom_sections(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 8 || &data[..4] != b"\0asm" {
        bail!("not a wasm module");
    }
    let mut output = Vec::with_capacity(data.len());
    output.extend_from_slice(&data[..8]);
    let mut position = 8usize;
    while position < data.len() {
        let start = position;
        let section_id = data[position];
        let (size, body) = read_leb_u32(data, position + 1)
            .with_context(|| format!("reading the size of the section at {start}"))?;
        let end = body
            .checked_add(usize::try_from(size).context("section size")?)
            .with_context(|| format!("section {section_id} at {start} overflows"))?;
        if end > data.len() {
            bail!("section {section_id} at {start} runs past the end of the module");
        }
        if section_id != 0 {
            output.extend_from_slice(&data[start..end]);
        }
        position = end;
    }
    Ok(output)
}

/// The length of `data` after gzip compression.
///
/// # Errors
///
/// Returns an error when compression fails.
pub fn gzip_len(data: &[u8]) -> Result<u64> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .context("gzip-compressing the module")?;
    let compressed = encoder.finish().context("finishing gzip compression")?;
    Ok(u64::try_from(compressed.len()).unwrap_or(u64::MAX))
}

/// Parse a wasm module into the size breakdown the report shows.
///
/// # Errors
///
/// Returns an error when the module is malformed or a section runs past the
/// end of the file.
pub fn analyze_module(data: &[u8]) -> Result<SizeBreakdown> {
    let raw = raw_sections(data)?;
    let mut code_bytes = 0u64;
    let mut data_bytes = 0u64;
    let mut custom_bytes = 0u64;
    let mut name_bytes = 0u64;
    let mut functions = 0u32;
    let mut data_segments = 0u64;
    for payload in Parser::new(0).parse_all(data) {
        match payload.context("parsing the wasm module")? {
            Payload::CodeSectionStart { count, size, .. } => {
                functions = count;
                code_bytes = u64::from(size);
            }
            Payload::DataSection(reader) => {
                for segment in reader {
                    data_bytes = data_bytes.saturating_add(
                        u64::try_from(segment.context("reading a data segment")?.data.len())
                            .unwrap_or(0),
                    );
                    data_segments += 1;
                }
            }
            Payload::CustomSection(section) => {
                let bytes = u64::try_from(section.data().len()).unwrap_or(0);
                custom_bytes = custom_bytes.saturating_add(bytes);
                if section.name() == "name" {
                    name_bytes = name_bytes.saturating_add(bytes);
                }
            }
            _ => {}
        }
    }
    let mut sections = Vec::with_capacity(raw.len());
    for section in raw {
        sections.push(SectionSize {
            id: section.id,
            name: section_name(section.id).to_owned(),
            bytes: u64::try_from(section.data.len()).unwrap_or(u64::MAX),
        });
    }
    let byte_count = |id: u8| {
        sections
            .iter()
            .find(|section| section.id == id)
            .map_or(0, |section| section.bytes)
    };
    Ok(SizeBreakdown {
        total_bytes: u64::try_from(data.len()).unwrap_or(u64::MAX),
        code_bytes,
        data_bytes,
        custom_bytes,
        name_bytes,
        import_bytes: byte_count(2),
        export_bytes: byte_count(7),
        functions,
        data_segments,
        sections,
    })
}

struct RawSection<'a> {
    id: u8,
    data: &'a [u8],
}

fn raw_sections(data: &[u8]) -> Result<Vec<RawSection<'_>>> {
    if data.len() < 8 || &data[..4] != b"\0asm" {
        bail!("not a wasm module");
    }
    let mut offset = 8usize;
    let mut sections = Vec::new();
    while offset < data.len() {
        let id = data[offset];
        offset += 1;
        let (length, next) = read_leb_u32(data, offset)?;
        let end = next
            .checked_add(usize::try_from(length).context("section length")?)
            .context("section length overflow")?;
        if end > data.len() {
            bail!("section {id} runs past the end of the module");
        }
        sections.push(RawSection {
            id,
            data: &data[next..end],
        });
        offset = end;
    }
    Ok(sections)
}

fn read_leb_u32(data: &[u8], position: usize) -> Result<(u32, usize)> {
    let mut result = 0u32;
    let mut shift = 0u32;
    let mut index = position;
    loop {
        let byte = *data
            .get(index)
            .with_context(|| format!("truncated LEB128 at byte {index}"))?;
        index += 1;
        result |= u32::from(byte & 0x7f)
            .checked_shl(shift)
            .context("LEB128 value is longer than 32 bits")?;
        if byte < 0x80 {
            return Ok((result, index));
        }
        shift += 7;
        if shift > 35 {
            bail!("LEB128 value longer than 5 bytes");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        wat::parse_str(
            r#"
            (module
              (import "env" "f" (func))
              (memory 1)
              (func (export "run") (result i32) i32.const 7)
              (data (i32.const 0) "hello")
              (@custom "name" (after code) "not-a-real-name-section")
              (@custom "producer" (after data) "cogwasm"))
            "#,
        )
        .expect("valid wat")
    }

    #[test]
    fn strip_removes_only_custom_sections() {
        let bytes = fixture();
        let stripped = strip_custom_sections(&bytes).expect("strip");
        assert!(stripped.len() < bytes.len());
        let original = raw_sections(&bytes).expect("sections");
        let remaining = raw_sections(&stripped).expect("sections");
        let original_non_custom: Vec<u8> = original
            .iter()
            .filter(|section| section.id != 0)
            .map(|section| section.id)
            .collect();
        let remaining_ids: Vec<u8> = remaining.iter().map(|section| section.id).collect();
        assert_eq!(original_non_custom, remaining_ids);
        assert!(remaining.iter().all(|section| section.id != 0));
    }

    #[test]
    fn analysis_counts_code_data_custom_import_and_functions() {
        let breakdown = analyze_module(&fixture()).expect("analysis");
        assert_eq!(breakdown.functions, 1);
        assert!(breakdown.code_bytes > 0);
        assert_eq!(breakdown.data_bytes, 5);
        assert_eq!(breakdown.data_segments, 1);
        assert!(breakdown.custom_bytes > 0);
        assert_eq!(breakdown.name_bytes, "not-a-real-name-section".len() as u64);
        assert!(breakdown.import_bytes > 0);
        assert!(breakdown.export_bytes > 0);
        assert_eq!(breakdown.total_bytes, fixture().len() as u64);
    }

    #[test]
    fn malformed_modules_are_rejected() {
        assert!(strip_custom_sections(b"not wasm").is_err());
        assert!(analyze_module(b"not wasm").is_err());
    }

    #[test]
    fn gzip_is_smaller_for_repetitive_input() {
        let data = vec![b'a'; 100_000];
        assert!(gzip_len(&data).expect("gzip") < 1_000);
    }
}

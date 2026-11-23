//! Basic-block instruction counting for compiled benchmark modules.
//!
//! The rewrite inserts one weighted counter update at the start of every basic
//! block: `global.get total; i64.const <block instructions>; i64.add;
//! global.set total`. The original instruction stream is re-encoded unchanged,
//! so control flow, block types and branch targets are untouched. After
//! `run(seed)` the exported `__cogwasm_bench_instructions` global is exactly
//! the sum of `executions x instructions` over blocks, the same definition the
//! `bench/gap` instrumenter uses.
//!
//! A single total (rather than one global per block) keeps the instrumented
//! module small and makes the read-back a single global read.

use anyhow::{Context, Result, anyhow, bail};
use wasm_encoder::reencode::{Reencode, RoundtripReencoder, utils};
use wasm_encoder::{
    CodeSection, ConstExpr, ExportKind, ExportSection, GlobalSection, GlobalType, Instruction,
    Module, RawSection, ValType,
};
use wasmparser::{ExternalKind, Operator, Parser, Payload, TypeRef};

/// The exported global the measurement reads back.
pub const COUNTER_EXPORT: &str = "__cogwasm_bench_instructions";

/// An instrumented module plus the static facts about its blocks.
#[derive(Debug, Clone)]
pub struct Instrumented {
    pub bytes: Vec<u8>,
    pub blocks: u64,
    pub static_instructions: u64,
}

struct BlockSpan {
    start: usize,
    instructions: u64,
}

struct Parsed {
    imported_globals: u32,
    defined_globals: Vec<(GlobalType, ConstExpr)>,
}

struct RawSourceSection {
    id: u8,
    data: Vec<u8>,
}

/// Rewrite `wasm` so every basic block adds its instruction count to one
/// exported counter.
///
/// # Errors
///
/// Returns an error for a malformed module, a module without an export or code
/// section, or an instruction the round-trip re-encoder rejects.
pub fn instrument(wasm: &[u8]) -> Result<Instrumented> {
    let parsed = parse_module(wasm)?;
    let sections = scan_sections(wasm)?;
    let global_index = parsed
        .imported_globals
        .checked_add(u32::try_from(parsed.defined_globals.len()).context("global count")?)
        .context("global index overflow")?;
    let mut code = CodeSection::new();
    let mut blocks = 0u64;
    let mut static_instructions = 0u64;
    for (body_index, body) in collect_bodies(wasm)?.iter().enumerate() {
        let spans = block_spans(body)?;
        blocks = blocks.saturating_add(u64::try_from(spans.len()).unwrap_or(0));
        static_instructions = static_instructions
            .saturating_add(spans.iter().map(|span| span.instructions).sum::<u64>());
        instrument_body(body, &spans, global_index, &mut code)
            .with_context(|| format!("instrumenting function body {body_index}"))?;
    }
    let mut module = Module::new();
    let mut inserted_globals = false;
    let mut replaced_code = false;
    for section in &sections {
        match section.id {
            6 => {
                module.section(&encode_globals(&parsed, global_index));
                inserted_globals = true;
            }
            7 => {
                if !inserted_globals {
                    module.section(&encode_globals(&parsed, global_index));
                    inserted_globals = true;
                }
                module.section(&encode_exports(wasm, global_index)?);
            }
            10 => {
                module.section(&code);
                replaced_code = true;
            }
            _ => {
                module.section(&RawSection {
                    id: section.id,
                    data: section.data.as_slice(),
                });
            }
        }
    }
    if !inserted_globals {
        bail!("module has no export section to place the counter before");
    }
    if !replaced_code {
        bail!("module has no code section");
    }
    Ok(Instrumented {
        bytes: module.finish(),
        blocks,
        static_instructions,
    })
}

fn encode_globals(parsed: &Parsed, counter_index: u32) -> GlobalSection {
    let mut globals = GlobalSection::new();
    for (ty, init) in &parsed.defined_globals {
        globals.global(*ty, init);
    }
    let _ = counter_index;
    globals.global(
        GlobalType {
            val_type: ValType::I64,
            mutable: true,
            shared: false,
        },
        &ConstExpr::i64_const(0),
    );
    globals
}

fn encode_exports(wasm: &[u8], counter_index: u32) -> Result<ExportSection> {
    let mut exports = ExportSection::new();
    for payload in Parser::new(0).parse_all(wasm) {
        if let Payload::ExportSection(section) = payload? {
            for export in section {
                let export = export?;
                let kind = match export.kind {
                    ExternalKind::Func | ExternalKind::FuncExact => ExportKind::Func,
                    ExternalKind::Table => ExportKind::Table,
                    ExternalKind::Memory => ExportKind::Memory,
                    ExternalKind::Global => ExportKind::Global,
                    ExternalKind::Tag => ExportKind::Tag,
                };
                exports.export(export.name, kind, export.index);
            }
        }
    }
    exports.export(COUNTER_EXPORT, ExportKind::Global, counter_index);
    Ok(exports)
}

fn instrument_body(
    body: &wasmparser::FunctionBody<'_>,
    spans: &[BlockSpan],
    counter_index: u32,
    code: &mut CodeSection,
) -> Result<()> {
    let mut reencoder = RoundtripReencoder;
    let mut function = utils::new_function_with_parsed_locals(&mut reencoder, body)?;
    let mut reader = body.get_operators_reader()?;
    let mut op_index = 0usize;
    let mut span_index = 0usize;
    while !reader.eof() {
        while span_index < spans.len() && spans[span_index].start == op_index {
            if spans[span_index].instructions > 0 {
                let count = i64::try_from(spans[span_index].instructions)
                    .context("a basic block has more instructions than i64::MAX")?;
                function.instruction(&Instruction::GlobalGet(counter_index));
                function.instruction(&Instruction::I64Const(count));
                function.instruction(&Instruction::I64Add);
                function.instruction(&Instruction::GlobalSet(counter_index));
            }
            span_index += 1;
        }
        let op = reader.read()?;
        let encoded = RoundtripReencoder
            .instruction(op)
            .map_err(|error| anyhow!("re-encoding an instruction: {error:?}"))?;
        function.instruction(&encoded);
        op_index += 1;
    }
    code.function(&function);
    Ok(())
}

/// Determine every basic block's start operator index and instruction count.
fn block_spans(body: &wasmparser::FunctionBody<'_>) -> Result<Vec<BlockSpan>> {
    let mut reader = body.get_operators_reader()?;
    let mut spans = Vec::new();
    let mut start = 0usize;
    let mut instructions = 0u64;
    let mut new_block = false;
    let mut index = 0usize;
    while !reader.eof() {
        let op = reader.read()?;
        if index > 0 && new_block {
            spans.push(BlockSpan {
                start,
                instructions,
            });
            start = index;
            instructions = 0;
            new_block = false;
        }
        if !is_structural(&op) {
            instructions = instructions.saturating_add(1);
        }
        if starts_new_block_after(&op) {
            new_block = true;
        }
        index += 1;
    }
    if !new_block || instructions > 0 {
        spans.push(BlockSpan {
            start,
            instructions,
        });
    }
    spans.retain(|span| span.instructions > 0);
    Ok(spans)
}

fn is_structural(op: &Operator<'_>) -> bool {
    matches!(
        op,
        Operator::Block { .. }
            | Operator::Loop { .. }
            | Operator::If { .. }
            | Operator::Try { .. }
            | Operator::TryTable { .. }
            | Operator::Else
            | Operator::End
            | Operator::Catch { .. }
            | Operator::CatchAll
            | Operator::Delegate { .. }
    )
}

fn starts_new_block_after(op: &Operator<'_>) -> bool {
    is_structural(op)
        || matches!(
            op,
            Operator::Br { .. }
                | Operator::BrTable { .. }
                | Operator::Return
                | Operator::Unreachable
                | Operator::Throw { .. }
                | Operator::ThrowRef
                | Operator::Rethrow { .. }
                | Operator::ReturnCall { .. }
                | Operator::ReturnCallIndirect { .. }
        )
}

fn parse_module(wasm: &[u8]) -> Result<Parsed> {
    let mut imported_globals = 0u32;
    let mut defined_globals = Vec::new();
    for payload in Parser::new(0).parse_all(wasm) {
        match payload.context("parsing the wasm module")? {
            Payload::ImportSection(section) => {
                for import in section.into_imports() {
                    if matches!(import.context("reading an import")?.ty, TypeRef::Global(_)) {
                        imported_globals += 1;
                    }
                }
            }
            Payload::GlobalSection(section) => {
                for global in section {
                    let global = global.context("reading a global")?;
                    let mut reencoder = RoundtripReencoder;
                    let ty = reencoder
                        .global_type(global.ty)
                        .context("re-encoding a global type")?;
                    let init = reencoder
                        .const_expr(global.init_expr)
                        .context("re-encoding a global initializer")?;
                    defined_globals.push((ty, init));
                }
            }
            _ => {}
        }
    }
    Ok(Parsed {
        imported_globals,
        defined_globals,
    })
}

fn collect_bodies(wasm: &[u8]) -> Result<Vec<wasmparser::FunctionBody<'_>>> {
    let mut bodies = Vec::new();
    for payload in Parser::new(0).parse_all(wasm) {
        if let Payload::CodeSectionEntry(body) = payload.context("parsing the wasm module")? {
            bodies.push(body);
        }
    }
    Ok(bodies)
}

fn scan_sections(wasm: &[u8]) -> Result<Vec<RawSourceSection>> {
    if wasm.len() < 8 || &wasm[..4] != b"\0asm" {
        bail!("not a wasm module");
    }
    let mut offset = 8usize;
    let mut sections = Vec::new();
    while offset < wasm.len() {
        let id = wasm[offset];
        offset += 1;
        let (length, next) = read_leb_u32(wasm, offset)?;
        let end = next
            .checked_add(usize::try_from(length).context("section length")?)
            .context("section length overflow")?;
        if end > wasm.len() {
            bail!("section {id} runs past the end of the module");
        }
        sections.push(RawSourceSection {
            id,
            data: wasm[next..end].to_vec(),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn module() -> Vec<u8> {
        wat::parse_str(
            r#"
            (module
              (memory (export "memory") 1)
              (global $g (mut i32) (i32.const 0))
              (func (export "run") (param i64) (result i64)
                (local i32)
                i64.const 1
                local.get 0
                i64.add
                block
                  i32.const 3
                  local.set 1
                end
                local.get 1
                i64.extend_i32_u
                i64.add)
              (func (export "out_ptr") (result i32) i32.const 0)
              (func (export "out_len") (result i32) i32.const 0))
            "#,
        )
        .expect("valid wat")
    }

    #[test]
    fn instrumented_module_exports_and_runs_the_counter() {
        let bytes = module();
        let instrumented = instrument(&bytes).expect("instrument");
        assert!(instrumented.blocks > 0);
        assert!(instrumented.static_instructions > 0);
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, &instrumented.bytes).expect("compile");
        let mut store = wasmtime::Store::new(&engine, ());
        let instance = wasmtime::Instance::new(&mut store, &module, &[]).expect("instantiate");
        let run = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .expect("run");
        assert_eq!(run.call(&mut store, 2).expect("call"), 6);
        let counter = instance
            .get_global(&mut store, COUNTER_EXPORT)
            .expect("counter")
            .get(&mut store);
        assert!(matches!(counter, wasmtime::Val::I64(value) if value > 0));
    }

    #[test]
    fn inserts_a_counter_when_the_module_has_no_global_section() {
        let bytes = wat::parse_str(
            r#"
            (module
              (memory (export "memory") 1)
              (func (export "run") (param i64) (result i64)
                local.get 0
                i64.const 1
                i64.add)
              (func (export "out_ptr") (result i32) i32.const 0)
              (func (export "out_len") (result i32) i32.const 0))
            "#,
        )
        .expect("valid wat");
        let instrumented = instrument(&bytes).expect("instrument");
        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::new(&engine, &instrumented.bytes).expect("compile");
        let mut store = wasmtime::Store::new(&engine, ());
        let instance = wasmtime::Instance::new(&mut store, &module, &[]).expect("instantiate");
        let run = instance
            .get_typed_func::<u64, u64>(&mut store, "run")
            .expect("run");
        assert_eq!(run.call(&mut store, 2).expect("call"), 3);
        let counter = instance
            .get_global(&mut store, COUNTER_EXPORT)
            .expect("counter")
            .get(&mut store);
        assert!(matches!(counter, wasmtime::Val::I64(value) if value > 0));
    }

    #[test]
    fn stripped_or_malformed_modules_are_rejected() {
        assert!(instrument(b"not wasm").is_err());
        let empty = wat::parse_str("(module)").expect("wat");
        assert!(instrument(&empty).is_err());
    }
}

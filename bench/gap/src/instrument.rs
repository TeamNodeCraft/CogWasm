//! Rewrite a wasm module so every basic block increments an exported counter.
//!
//! The rewrite is intentionally conservative: the original instruction stream
//! is re-encoded unchanged, and before each basic-block entry we insert
//! `global.get $c; i64.const 1; i64.add; global.set $c`. That sequence has no
//! net stack effect, so block types and branch targets are untouched.

use crate::required_option;
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use wasm_encoder::reencode::{Reencode, RoundtripReencoder, utils};
use wasm_encoder::{
    CodeSection, ConstExpr, ExportKind, ExportSection, GlobalSection, GlobalType, Instruction,
    Module, RawSection, ValType,
};
use wasmparser::{FunctionBody, Name, Operator, Parser, Payload, TypeRef};

// ---------------------------------------------------------------------------
// Metadata written next to the instrumented module
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub source_wasm: String,
    pub instrumented_wasm: String,
    pub source_bytes: u64,
    pub functions: Vec<FunctionMeta>,
    pub blocks: Vec<BlockMeta>,
    pub stack_pointer_global: Option<u32>,
    pub imported_globals: u32,
    pub original_globals: u32,
    pub original_sections: Vec<SectionMeta>,
    pub data_segments: usize,
    pub data_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionMeta {
    pub id: u8,
    pub name: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMeta {
    pub index: u32,
    pub name: String,
    pub demangled: String,
    pub imported: bool,
    pub type_index: u32,
    pub body_bytes: u64,
    pub static_instructions: u64,
    pub blocks: Vec<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockMeta {
    pub id: u32,
    pub func: u32,
    /// Number of non-structural original instructions in this block.
    pub instructions: u32,
    pub bytes: u64,
    pub categories: BTreeMap<String, u32>,
    pub stack_loads: u32,
    pub stack_stores: u32,
    pub heap_loads: u32,
    pub heap_stores: u32,
    pub calls: Vec<CallSite>,
    pub bounds_check_pairs: u32,
    pub in_loop: bool,
    pub loop_header: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallSite {
    pub target: u32,
    pub name: String,
    pub demangled: String,
    pub kind: String,
}

// ---------------------------------------------------------------------------
// Instrumentation
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameKind {
    Block,
    Loop,
    If,
    Try,
    Function,
}

struct Frame {
    id: usize,
    kind: FrameKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cat {
    LocalGet,
    LocalSet,
    LocalTee,
    LoadStack,
    LoadHeap,
    StoreStack,
    StoreHeap,
    CallDirect,
    CallIndirect,
    Br,
    BrIf,
    BrTable,
    Return,
    Block,
    Loop,
    If,
    Else,
    End,
    IntArith,
    IntCompare,
    FloatArith,
    FloatCompare,
    Conversion,
    Const,
    GlobalGet,
    GlobalSet,
    Select,
    Drop,
    Memory,
    BoundsCheck,
    Other,
}

impl Cat {
    fn as_str(self) -> &'static str {
        match self {
            Cat::LocalGet => "local.get",
            Cat::LocalSet => "local.set",
            Cat::LocalTee => "local.tee",
            Cat::LoadStack => "load.stack",
            Cat::LoadHeap => "load.heap",
            Cat::StoreStack => "store.stack",
            Cat::StoreHeap => "store.heap",
            Cat::CallDirect => "call.direct",
            Cat::CallIndirect => "call.indirect",
            Cat::Br => "br",
            Cat::BrIf => "br_if",
            Cat::BrTable => "br_table",
            Cat::Return => "return",
            Cat::Block => "block",
            Cat::Loop => "loop",
            Cat::If => "if",
            Cat::Else => "else",
            Cat::End => "end",
            Cat::IntArith => "int.arith",
            Cat::IntCompare => "int.compare",
            Cat::FloatArith => "float.arith",
            Cat::FloatCompare => "float.compare",
            Cat::Conversion => "conversion",
            Cat::Const => "const",
            Cat::GlobalGet => "global.get",
            Cat::GlobalSet => "global.set",
            Cat::Select => "select",
            Cat::Drop => "drop",
            Cat::Memory => "memory",
            Cat::BoundsCheck => "bounds.check",
            Cat::Other => "other",
        }
    }

    fn structural(self) -> bool {
        matches!(
            self,
            Cat::Block | Cat::Loop | Cat::If | Cat::Else | Cat::End
        )
    }
}

struct InsnMeta {
    cat: Cat,
    is_int_compare: bool,
    branch_targets: Vec<usize>,
    if_frame: Option<usize>,
    call: Option<CallSite>,
    panics: bool,
}

struct BlockBuilder {
    meta: BlockMeta,
    insns: Vec<InsnMeta>,
}

#[derive(Clone)]
struct FunctionInfo {
    index: u32,
    name: String,
    demangled: String,
    imported: bool,
    type_index: u32,
    body_bytes: u64,
}

struct Instrumenter<'a> {
    bytes: &'a [u8],
    funcs: Vec<FunctionInfo>,
    func_types: Vec<u32>,
    type_params: Vec<usize>,
    type_results: Vec<usize>,
    imported_globals: u32,
    defined_globals: Vec<(GlobalType, ConstExpr)>,
    stack_pointer_global: Option<u32>,
    original_globals: u32,
    stack_locals: BTreeSet<u32>,
    blocks: Vec<BlockBuilder>,
    frames: Vec<Frame>,
    frame_targets: HashMap<usize, Option<u32>>,
    frame_else_targets: HashMap<usize, Option<u32>>,
    next_frame_id: usize,
    loop_depth: u32,
    current_func: u32,
    next_defined: u32,
    current_block: Option<u32>,
    pending_increment: bool,
    address_stack: Vec<bool>,
    local_stack: HashMap<u32, bool>,
}

pub fn run(rest: &[String]) -> Result<()> {
    let wasm_path = required_option(rest, "--wasm")?;
    let out = required_option(rest, "--out")?;
    let meta_path = required_option(rest, "--meta")?;
    let bytes = std::fs::read(&wasm_path).with_context(|| format!("reading {wasm_path}"))?;
    let mut instrumenter = Instrumenter::new(&bytes)?;
    let output_bytes = instrumenter.instrument()?;
    std::fs::write(&out, &output_bytes)?;
    let meta = instrumenter.meta(&wasm_path, &out, &output_bytes)?;
    std::fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
    eprintln!(
        "instrumented {}: {} -> {} bytes, {} functions, {} blocks",
        wasm_path,
        meta.source_bytes,
        output_bytes.len(),
        meta.functions.len(),
        meta.blocks.len()
    );
    Ok(())
}

impl<'a> Instrumenter<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self> {
        let parsed = parse_module(bytes)?;
        let stack_pointer_global = find_stack_pointer(
            &parsed.defined_globals,
            &parsed.defined_global_inits,
            parsed.imported_globals,
        );
        Ok(Self {
            bytes,
            funcs: parsed.funcs,
            func_types: parsed.func_types,
            type_params: parsed.type_params,
            type_results: parsed.type_results,
            imported_globals: parsed.imported_globals,
            defined_globals: parsed.defined_globals,
            stack_pointer_global,
            original_globals: parsed.original_globals,
            stack_locals: BTreeSet::new(),
            blocks: Vec::new(),
            frames: Vec::new(),
            frame_targets: HashMap::new(),
            frame_else_targets: HashMap::new(),
            next_frame_id: 0,
            loop_depth: 0,
            current_func: 0,
            next_defined: parsed.imported_funcs,
            current_block: None,
            pending_increment: false,
            address_stack: Vec::new(),
            local_stack: HashMap::new(),
        })
    }

    fn instrument(&mut self) -> Result<Vec<u8>> {
        // First pass: find locals that may hold a shadow-stack address. The
        // scan is flow-insensitive (a local is tainted if any assignment could
        // come from `__stack_pointer`), which is the conservative direction for
        // classifying a load/store as stack traffic.
        self.find_stack_locals()?;

        let sections = scan_sections(self.bytes)?;
        let mut code = CodeSection::new();
        let mut exported_globals = 0u32;
        let mut function_bodies = Vec::new();
        for payload in Parser::new(0).parse_all(self.bytes) {
            if let Payload::CodeSectionEntry(body) = payload? {
                function_bodies.push(body);
            }
        }
        let defined_count = self.funcs.iter().filter(|f| !f.imported).count();
        if function_bodies.len() != defined_count {
            bail!(
                "code section has {} bodies but the function section has {} defined functions",
                function_bodies.len(),
                defined_count
            );
        }
        for (body_index, body) in function_bodies.iter().enumerate() {
            self.instrument_body(body, &mut code)?;
            exported_globals += 1;
            let _ = body_index;
        }
        let _ = exported_globals;

        let mut module = Module::new();
        let mut inserted_globals = false;
        let mut replaced_code = false;
        for section in &sections {
            match section.id {
                6 => {
                    let globals = self.encode_globals();
                    module.section(&globals);
                    inserted_globals = true;
                }
                7 => {
                    if !inserted_globals {
                        let globals = self.encode_globals();
                        module.section(&globals);
                        inserted_globals = true;
                    }
                    let exports = self.encode_exports()?;
                    module.section(&exports);
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
            bail!("module has no export section to place the new globals before");
        }
        if !replaced_code {
            bail!("module has no code section");
        }
        Ok(module.finish())
    }

    fn encode_globals(&self) -> GlobalSection {
        let mut globals = GlobalSection::new();
        for (ty, init) in &self.defined_globals {
            globals.global(*ty, init);
        }
        for _block in &self.blocks {
            globals.global(
                GlobalType {
                    val_type: ValType::I64,
                    mutable: true,
                    shared: false,
                },
                &ConstExpr::i64_const(0),
            );
        }
        globals
    }

    #[expect(
        clippy::excessive_nesting,
        reason = "copying the export table is a single short match"
    )]
    fn encode_exports(&self) -> Result<ExportSection> {
        let mut exports = ExportSection::new();
        for payload in Parser::new(0).parse_all(self.bytes) {
            if let Payload::ExportSection(section) = payload? {
                for export in section {
                    let export = export?;
                    let kind = match export.kind {
                        wasmparser::ExternalKind::Func | wasmparser::ExternalKind::FuncExact => {
                            ExportKind::Func
                        }
                        wasmparser::ExternalKind::Table => ExportKind::Table,
                        wasmparser::ExternalKind::Memory => ExportKind::Memory,
                        wasmparser::ExternalKind::Global => ExportKind::Global,
                        wasmparser::ExternalKind::Tag => ExportKind::Tag,
                    };
                    exports.export(export.name, kind, export.index);
                }
            }
        }
        for block in &self.blocks {
            let index = self.original_globals + block.meta.id;
            exports.export(
                &format!("__gap_b{}", block.meta.id),
                ExportKind::Global,
                index,
            );
        }
        Ok(exports)
    }

    // -- first pass ---------------------------------------------------------

    fn find_stack_locals(&mut self) -> Result<()> {
        let bodies = collect_bodies(self.bytes)?;
        let mut stack_locals = BTreeSet::new();
        for _pass in 0..4 {
            let before = stack_locals.len();
            for (defined_index, body) in bodies.iter().enumerate() {
                let imported = u32::try_from(self.funcs.iter().filter(|f| f.imported).count())?;
                let func_index = imported + u32::try_from(defined_index)?;
                self.simulate_body(body, func_index, &mut stack_locals)?;
            }
            if stack_locals.len() == before {
                break;
            }
        }
        self.stack_locals = stack_locals;
        Ok(())
    }

    fn simulate_body(
        &self,
        body: &FunctionBody<'_>,
        func_index: u32,
        stack_locals: &mut BTreeSet<u32>,
    ) -> Result<()> {
        let mut reader = body.get_operators_reader()?;
        let mut locals: HashMap<u32, bool> = HashMap::new();
        let param_count = self.type_params[self.func_types[func_index as usize] as usize];
        for local in 0..u32::try_from(param_count)? {
            locals.insert(local, false);
        }
        let mut local_count = 0u32;
        for pair in body.get_locals_reader()? {
            let (count, _) = pair?;
            local_count += count;
        }
        for local in 0..local_count {
            locals.insert(u32::try_from(param_count)? + local, false);
        }
        let mut stack: Vec<bool> = Vec::new();
        while !reader.eof() {
            let op = reader.read()?;
            if is_block_start(&op) {
                stack.clear();
                for local in stack_locals.iter() {
                    locals.insert(*local, true);
                }
                continue;
            }
            if matches!(op, Operator::Else | Operator::End) || is_unconditional_end(&op) {
                stack.clear();
                continue;
            }
            if let Operator::Call { function_index } = op {
                let type_index = self.func_types[function_index as usize] as usize;
                let pops = self.type_params[type_index];
                let pushes = self.type_results[type_index];
                for _ in 0..pops {
                    stack.pop();
                }
                stack.resize(stack.len() + pushes, false);
                continue;
            }
            if let Operator::CallIndirect { type_index, .. } = op {
                let pops = self.type_params[type_index as usize] + 1;
                let pushes = self.type_results[type_index as usize];
                for _ in 0..pops {
                    stack.pop();
                }
                stack.resize(stack.len() + pushes, false);
                continue;
            }
            match &op {
                Operator::GlobalGet { global_index } => {
                    stack.push(Some(*global_index) == self.stack_pointer_global);
                }
                Operator::LocalGet { local_index } => {
                    stack.push(locals.get(local_index).copied().unwrap_or(false));
                }
                Operator::LocalSet { local_index } | Operator::LocalTee { local_index } => {
                    let value = stack.pop().unwrap_or(false);
                    let previous = locals.insert(*local_index, value).unwrap_or(false);
                    if value && !previous {
                        stack_locals.insert(*local_index);
                    }
                    if matches!(op, Operator::LocalTee { .. }) {
                        stack.push(value);
                    }
                }
                Operator::I32Const { .. }
                | Operator::I64Const { .. }
                | Operator::F32Const { .. }
                | Operator::F64Const { .. } => stack.push(false),
                _ => {
                    let (pops, pushes, preserve) =
                        stack_effect(&op, &self.type_params, &self.type_results);
                    let mut operands = Vec::new();
                    for _ in 0..pops {
                        operands.push(stack.pop().unwrap_or(false));
                    }
                    let value = preserve && operands.iter().any(|operand| *operand);
                    for _ in 0..pushes {
                        stack.push(value);
                    }
                }
            }
            if stack.len() > 4096 {
                stack.clear();
            }
        }
        Ok(())
    }

    // -- second pass --------------------------------------------------------

    #[expect(
        clippy::too_many_lines,
        clippy::excessive_nesting,
        clippy::collapsible_if,
        clippy::collapsible_else_if,
        reason = "one pass over the operator stream is clearer than a dozen \
                  state-machine helpers; the nesting is the wasm control stack"
    )]
    fn instrument_body(&mut self, body: &FunctionBody<'_>, code: &mut CodeSection) -> Result<()> {
        self.current_func = self.next_defined;
        self.next_defined += 1;
        self.frames.clear();
        self.frame_targets.clear();
        self.frame_else_targets.clear();
        self.loop_depth = 0;
        self.address_stack.clear();
        self.local_stack.clear();
        for local in &self.stack_locals {
            self.local_stack.insert(*local, true);
        }
        let params = self.type_params[self.func_types[self.current_func as usize] as usize];
        for local in 0..u32::try_from(params)? {
            self.local_stack.insert(local, false);
        }

        let start = self.new_block();
        let function_frame = self.new_frame_id();
        self.frames.push(Frame {
            id: function_frame,
            kind: FrameKind::Function,
        });
        self.current_block = Some(start);
        self.pending_increment = true;

        let mut reencoder = RoundtripReencoder;
        let mut function = utils::new_function_with_parsed_locals(&mut reencoder, body)?;
        let mut reader = body.get_operators_reader()?;
        while !reader.eof() {
            let op = reader.read()?;
            let is_last = reader.eof();
            if self.pending_increment {
                if let Some(block) = self.current_block {
                    self.emit_increment(&mut function, block);
                    self.pending_increment = false;
                }
            }
            let cat = self.classify(&op, &mut function)?;
            let mut insn = InsnMeta {
                cat,
                is_int_compare: is_int_compare(&op),
                branch_targets: Vec::new(),
                if_frame: None,
                panics: is_panic_operator(&op)
                    || call_target(&op, &self.funcs)
                        .as_ref()
                        .is_some_and(|call| is_panic_name(&call.demangled)),
                call: call_target(&op, &self.funcs),
            };
            if let Some(call) = &insn.call {
                if let Some(block) = self.current_block {
                    self.blocks[block as usize].meta.calls.push(call.clone());
                }
            }
            if let Some(depths) = branch_depths(&op) {
                for depth in depths {
                    if let Some(frame) = self.frame_at(depth) {
                        insn.branch_targets.push(frame.id);
                    }
                }
            }
            if is_unconditional_end(&op) && !is_last {
                if let Some(block) = self.current_block {
                    self.blocks[block as usize].insns.push(insn);
                }
                self.address_stack.clear();
                self.start_new_block();
            } else {
                if let Some(block) = self.current_block {
                    self.blocks[block as usize].insns.push(insn);
                }
            }
            match &op {
                Operator::Block { .. }
                | Operator::Loop { .. }
                | Operator::If { .. }
                | Operator::Try { .. }
                | Operator::TryTable { .. } => {
                    let kind = match &op {
                        Operator::Loop { .. } => FrameKind::Loop,
                        Operator::If { .. } => FrameKind::If,
                        Operator::Try { .. } | Operator::TryTable { .. } => FrameKind::Try,
                        _ => FrameKind::Block,
                    };
                    if kind == FrameKind::Loop {
                        self.loop_depth += 1;
                    }
                    let body_block = self.new_block();
                    if kind == FrameKind::Loop {
                        self.blocks[body_block as usize].meta.loop_header = true;
                    }
                    let frame_id = self.new_frame_id();
                    self.frames.push(Frame { id: frame_id, kind });
                    self.frame_targets.insert(frame_id, Some(body_block));
                    if kind == FrameKind::If {
                        if let Some(block) = self.current_block {
                            if let Some(insn) = self.blocks[block as usize].insns.last_mut() {
                                insn.if_frame = Some(frame_id);
                            }
                        }
                    }
                    self.current_block = Some(body_block);
                    self.pending_increment = true;
                    self.address_stack.clear();
                }
                Operator::Else => {
                    let else_block = self.new_block();
                    if let Some(frame) = self.frames.last() {
                        let frame_id = frame.id;
                        self.frame_else_targets.insert(frame_id, Some(else_block));
                    }
                    self.current_block = Some(else_block);
                    self.pending_increment = true;
                    self.address_stack.clear();
                }
                Operator::End => {
                    let final_end = self.frames.len() == 1;
                    if let Some(frame) = self.frames.last() {
                        let frame_id = frame.id;
                        let kind = frame.kind;
                        if kind == FrameKind::Loop {
                            self.loop_depth = self.loop_depth.saturating_sub(1);
                        }
                        self.frames.pop();
                        if final_end {
                            self.frame_targets.insert(frame_id, None);
                        } else {
                            let continuation = self.new_block();
                            // A branch to a `loop` label re-enters the header;
                            // every other label branches to the continuation.
                            if kind != FrameKind::Loop {
                                self.frame_targets.insert(frame_id, Some(continuation));
                            }
                            self.frame_targets
                                .entry(frame_id)
                                .or_insert(Some(continuation));
                            self.current_block = Some(continuation);
                            self.pending_increment = true;
                            self.address_stack.clear();
                        }
                    }
                }
                _ => {}
            }
        }
        // Resolve branch targets and mark bounds checks for the function.
        self.finish_function();
        code.function(&function);
        Ok(())
    }

    fn new_frame_id(&mut self) -> usize {
        let id = self.next_frame_id;
        self.next_frame_id += 1;
        id
    }

    fn new_block(&mut self) -> u32 {
        let id = u32::try_from(self.blocks.len()).expect("block count fits in u32");
        let mut meta = BlockMeta {
            id,
            func: self.current_func,
            in_loop: self.loop_depth > 0,
            ..BlockMeta::default()
        };
        meta.id = id;
        self.blocks.push(BlockBuilder {
            meta,
            insns: Vec::new(),
        });
        id
    }

    fn start_new_block(&mut self) {
        let block = self.new_block();
        self.current_block = Some(block);
        self.pending_increment = true;
        self.address_stack.clear();
    }

    fn frame_at(&self, depth: u32) -> Option<&Frame> {
        let index = self
            .frames
            .len()
            .checked_sub(usize::try_from(depth).ok()? + 1)?;
        self.frames.get(index)
    }

    fn emit_increment(&self, function: &mut wasm_encoder::Function, block: u32) {
        let global = self.original_globals + block;
        function.instruction(&Instruction::GlobalGet(global));
        function.instruction(&Instruction::I64Const(1));
        function.instruction(&Instruction::I64Add);
        function.instruction(&Instruction::GlobalSet(global));
    }

    fn classify(
        &mut self,
        op: &Operator<'_>,
        function: &mut wasm_encoder::Function,
    ) -> Result<Cat> {
        let (cat, stack_address) = self.categorize(op);
        let category = match (cat, stack_address) {
            (Cat::LoadHeap, Some(true)) => Cat::LoadStack,
            (Cat::StoreHeap, Some(true)) => Cat::StoreStack,
            (other, _) => other,
        };
        let op_owned = op.clone();
        let encoded = RoundtripReencoder
            .instruction(op_owned)
            .map_err(|error| anyhow::anyhow!("re-encoding instruction: {error:?}"))?;
        function.instruction(&encoded);
        if let Some(block) = self.current_block {
            let block = &mut self.blocks[block as usize];
            if category.structural() {
                // Structural markers are not counted as executed work.
            } else {
                block.meta.instructions += 1;
                *block
                    .meta
                    .categories
                    .entry(category.as_str().to_owned())
                    .or_insert(0) += 1;
                if category == Cat::LoadStack {
                    block.meta.stack_loads += 1;
                } else if category == Cat::StoreStack {
                    block.meta.stack_stores += 1;
                } else if category == Cat::LoadHeap {
                    block.meta.heap_loads += 1;
                } else if category == Cat::StoreHeap {
                    block.meta.heap_stores += 1;
                }
            }
        }
        Ok(category)
    }

    #[expect(
        clippy::too_many_lines,
        reason = "a flat opcode table is the readable form; splitting it \
                  would hide which operators share a category"
    )]
    fn categorize(&mut self, op: &Operator<'_>) -> (Cat, Option<bool>) {
        let stack_pointer = self.stack_pointer_global;
        let mut stack_address = None;
        let cat = match op {
            Operator::LocalGet { local_index } => {
                self.address_stack
                    .push(self.local_stack.get(local_index).copied().unwrap_or(false));
                Cat::LocalGet
            }
            Operator::LocalSet { local_index } => {
                let value = self.address_stack.pop().unwrap_or(false);
                self.local_stack.insert(*local_index, value);
                Cat::LocalSet
            }
            Operator::LocalTee { local_index } => {
                let value = self.address_stack.last().copied().unwrap_or(false);
                self.local_stack.insert(*local_index, value);
                Cat::LocalTee
            }
            Operator::GlobalGet { global_index } => {
                self.address_stack
                    .push(Some(*global_index) == stack_pointer);
                Cat::GlobalGet
            }
            Operator::GlobalSet { .. } => {
                self.address_stack.pop();
                Cat::GlobalSet
            }
            Operator::I32Load { .. }
            | Operator::I64Load { .. }
            | Operator::F32Load { .. }
            | Operator::F64Load { .. }
            | Operator::I32Load8S { .. }
            | Operator::I32Load8U { .. }
            | Operator::I32Load16S { .. }
            | Operator::I32Load16U { .. }
            | Operator::I64Load8S { .. }
            | Operator::I64Load8U { .. }
            | Operator::I64Load16S { .. }
            | Operator::I64Load16U { .. }
            | Operator::I64Load32S { .. }
            | Operator::I64Load32U { .. } => {
                let address = self.address_stack.pop().unwrap_or(false);
                self.address_stack.push(false);
                stack_address = Some(address);
                Cat::LoadHeap
            }
            Operator::I32Store { .. }
            | Operator::I64Store { .. }
            | Operator::F32Store { .. }
            | Operator::F64Store { .. }
            | Operator::I32Store8 { .. }
            | Operator::I32Store16 { .. }
            | Operator::I64Store8 { .. }
            | Operator::I64Store16 { .. }
            | Operator::I64Store32 { .. } => {
                self.address_stack.pop();
                let address = self.address_stack.pop().unwrap_or(false);
                stack_address = Some(address);
                Cat::StoreHeap
            }
            Operator::Call { .. } => {
                let call = call_target(op, &self.funcs);
                let type_index = call
                    .as_ref()
                    .and_then(|call| self.func_types.get(call.target as usize))
                    .copied()
                    .unwrap_or(0);
                self.apply_call(type_index);
                Cat::CallDirect
            }
            Operator::CallIndirect { type_index, .. } => {
                self.address_stack.pop();
                self.apply_call(*type_index);
                Cat::CallIndirect
            }
            Operator::Br { .. } => {
                self.address_stack.clear();
                Cat::Br
            }
            Operator::BrIf { .. } => {
                self.address_stack.pop();
                Cat::BrIf
            }
            Operator::BrTable { .. } => {
                self.address_stack.clear();
                Cat::BrTable
            }
            Operator::Return => {
                self.address_stack.clear();
                Cat::Return
            }
            Operator::Block { .. } => Cat::Block,
            Operator::Loop { .. } => Cat::Loop,
            Operator::If { .. } => {
                self.address_stack.pop();
                Cat::If
            }
            Operator::Else => Cat::Else,
            Operator::End => Cat::End,
            Operator::I32Add
            | Operator::I32Sub
            | Operator::I32Mul
            | Operator::I32DivS
            | Operator::I32DivU
            | Operator::I32RemS
            | Operator::I32RemU
            | Operator::I32And
            | Operator::I32Or
            | Operator::I32Xor
            | Operator::I32Shl
            | Operator::I32ShrS
            | Operator::I32ShrU
            | Operator::I32Rotl
            | Operator::I32Rotr
            | Operator::I64Add
            | Operator::I64Sub
            | Operator::I64Mul
            | Operator::I64DivS
            | Operator::I64DivU
            | Operator::I64RemS
            | Operator::I64RemU
            | Operator::I64And
            | Operator::I64Or
            | Operator::I64Xor
            | Operator::I64Shl
            | Operator::I64ShrS
            | Operator::I64ShrU
            | Operator::I64Rotl
            | Operator::I64Rotr
            | Operator::I32Clz
            | Operator::I32Ctz
            | Operator::I32Popcnt
            | Operator::I64Clz
            | Operator::I64Ctz
            | Operator::I64Popcnt => {
                let a = self.address_stack.pop().unwrap_or(false);
                let b = self.address_stack.pop().unwrap_or(false);
                self.address_stack.push(a || b);
                Cat::IntArith
            }
            Operator::I32Eqz
            | Operator::I32Eq
            | Operator::I32Ne
            | Operator::I32LtS
            | Operator::I32LtU
            | Operator::I32GtS
            | Operator::I32GtU
            | Operator::I32LeS
            | Operator::I32LeU
            | Operator::I32GeS
            | Operator::I32GeU
            | Operator::I64Eqz
            | Operator::I64Eq
            | Operator::I64Ne
            | Operator::I64LtS
            | Operator::I64LtU
            | Operator::I64GtS
            | Operator::I64GtU
            | Operator::I64LeS
            | Operator::I64LeU
            | Operator::I64GeS
            | Operator::I64GeU => {
                self.address_stack.pop();
                self.address_stack.pop();
                self.address_stack.push(false);
                Cat::IntCompare
            }
            Operator::F32Abs
            | Operator::F32Neg
            | Operator::F32Ceil
            | Operator::F32Floor
            | Operator::F32Trunc
            | Operator::F32Nearest
            | Operator::F32Sqrt
            | Operator::F32Add
            | Operator::F32Sub
            | Operator::F32Mul
            | Operator::F32Div
            | Operator::F32Min
            | Operator::F32Max
            | Operator::F32Copysign
            | Operator::F64Abs
            | Operator::F64Neg
            | Operator::F64Ceil
            | Operator::F64Floor
            | Operator::F64Trunc
            | Operator::F64Nearest
            | Operator::F64Sqrt
            | Operator::F64Add
            | Operator::F64Sub
            | Operator::F64Mul
            | Operator::F64Div
            | Operator::F64Min
            | Operator::F64Max
            | Operator::F64Copysign => {
                self.address_stack.pop();
                self.address_stack.pop();
                self.address_stack.push(false);
                Cat::FloatArith
            }
            Operator::F32Eq
            | Operator::F32Ne
            | Operator::F32Lt
            | Operator::F32Gt
            | Operator::F32Le
            | Operator::F32Ge
            | Operator::F64Eq
            | Operator::F64Ne
            | Operator::F64Lt
            | Operator::F64Gt
            | Operator::F64Le
            | Operator::F64Ge => {
                self.address_stack.pop();
                self.address_stack.pop();
                self.address_stack.push(false);
                Cat::FloatCompare
            }
            Operator::I32Const { .. }
            | Operator::I64Const { .. }
            | Operator::F32Const { .. }
            | Operator::F64Const { .. } => {
                self.address_stack.push(false);
                Cat::Const
            }
            Operator::Select | Operator::TypedSelect { .. } | Operator::TypedSelectMulti { .. } => {
                self.address_stack.pop();
                let a = self.address_stack.pop().unwrap_or(false);
                let b = self.address_stack.pop().unwrap_or(false);
                self.address_stack.push(a || b);
                Cat::Select
            }
            Operator::Drop => {
                self.address_stack.pop();
                Cat::Drop
            }
            Operator::MemorySize { .. } => {
                self.address_stack.push(false);
                Cat::Memory
            }
            Operator::MemoryGrow { .. } => {
                self.address_stack.pop();
                self.address_stack.push(false);
                Cat::Memory
            }
            Operator::MemoryCopy { .. }
            | Operator::MemoryFill { .. }
            | Operator::MemoryInit { .. }
            | Operator::DataDrop { .. }
            | Operator::MemoryDiscard { .. } => {
                self.address_stack.clear();
                Cat::Memory
            }
            Operator::I32WrapI64
            | Operator::I64ExtendI32S
            | Operator::I64ExtendI32U
            | Operator::I32TruncF32S
            | Operator::I32TruncF32U
            | Operator::I32TruncF64S
            | Operator::I32TruncF64U
            | Operator::I64TruncF32S
            | Operator::I64TruncF32U
            | Operator::I64TruncF64S
            | Operator::I64TruncF64U
            | Operator::I32TruncSatF32S
            | Operator::I32TruncSatF32U
            | Operator::I32TruncSatF64S
            | Operator::I32TruncSatF64U
            | Operator::I64TruncSatF32S
            | Operator::I64TruncSatF32U
            | Operator::I64TruncSatF64S
            | Operator::I64TruncSatF64U
            | Operator::F32ConvertI32S
            | Operator::F32ConvertI32U
            | Operator::F32ConvertI64S
            | Operator::F32ConvertI64U
            | Operator::F64ConvertI32S
            | Operator::F64ConvertI32U
            | Operator::F64ConvertI64S
            | Operator::F64ConvertI64U
            | Operator::F32DemoteF64
            | Operator::F64PromoteF32
            | Operator::I32ReinterpretF32
            | Operator::I64ReinterpretF64
            | Operator::F32ReinterpretI32
            | Operator::F64ReinterpretI64
            | Operator::I32Extend8S
            | Operator::I32Extend16S
            | Operator::I64Extend8S
            | Operator::I64Extend16S
            | Operator::I64Extend32S => {
                let value = self.address_stack.pop().unwrap_or(false);
                self.address_stack.push(value);
                Cat::Conversion
            }
            _ => {
                self.fallback_stack_effect(op);
                Cat::Other
            }
        };
        if self.address_stack.len() > 4096 {
            self.address_stack.clear();
        }
        (cat, stack_address)
    }

    fn apply_call(&mut self, type_index: u32) {
        let params = self
            .type_params
            .get(type_index as usize)
            .copied()
            .unwrap_or(0);
        let results = self
            .type_results
            .get(type_index as usize)
            .copied()
            .unwrap_or(0);
        for _ in 0..params {
            self.address_stack.pop();
        }
        for _ in 0..results {
            self.address_stack.push(false);
        }
    }

    fn fallback_stack_effect(&mut self, op: &Operator<'_>) {
        // Unknown operators: keep the simulation conservative. Resetting the
        // operand stack can only make a memory access look like heap traffic,
        // never the reverse.
        if is_block_start(op) || is_block_end(op) {
            self.address_stack.clear();
        }
    }

    #[expect(
        clippy::excessive_nesting,
        reason = "bounds-check detection walks blocks and control frames together"
    )]
    fn finish_function(&mut self) {
        let function_blocks: Vec<u32> = self
            .blocks
            .iter()
            .filter(|block| block.meta.func == self.current_func)
            .map(|block| block.meta.id)
            .collect();
        let mut panic_blocks = BTreeSet::new();
        for block_id in &function_blocks {
            let block = &self.blocks[*block_id as usize];
            if block.insns.iter().any(|insn| insn.panics) {
                panic_blocks.insert(*block_id);
            }
        }
        // Propagate through branches a few times: a block whose branch target
        // is a panic block is itself a panic path for the bounds-check shape.
        for _ in 0..4 {
            for block_id in &function_blocks {
                let block = &self.blocks[*block_id as usize];
                let reaches = block.insns.iter().any(|insn| {
                    insn.branch_targets.iter().any(|frame_id| {
                        self.frame_targets
                            .get(frame_id)
                            .copied()
                            .flatten()
                            .is_some_and(|target_block| panic_blocks.contains(&target_block))
                    })
                });
                if reaches {
                    panic_blocks.insert(*block_id);
                }
            }
        }
        let mut changed = false;
        for block_id in &function_blocks {
            let block = &mut self.blocks[*block_id as usize];
            let mut index = 0;
            while index + 1 < block.insns.len() {
                if block.insns[index].is_int_compare
                    && block.insns[index + 1].cat == Cat::If
                    && block.insns[index + 1].if_frame.is_some_and(|frame_id| {
                        let else_target = self.frame_else_targets.get(&frame_id).copied().flatten();
                        let end_target = self.frame_targets.get(&frame_id).copied().flatten();
                        else_target
                            .or(end_target)
                            .is_some_and(|target| panic_blocks.contains(&target))
                    })
                {
                    block.insns[index].cat = Cat::BoundsCheck;
                    block.insns[index + 1].cat = Cat::BoundsCheck;
                    changed = true;
                    index += 2;
                    continue;
                }
                if block.insns[index].is_int_compare
                    && !block.insns[index + 1].branch_targets.is_empty()
                    && block.insns[index + 1].cat == Cat::BrIf
                {
                    let target_hits =
                        block.insns[index + 1]
                            .branch_targets
                            .iter()
                            .any(|frame_id| {
                                self.frame_targets
                                    .get(frame_id)
                                    .copied()
                                    .flatten()
                                    .is_some_and(|target| panic_blocks.contains(&target))
                            });
                    if target_hits {
                        block.insns[index].cat = Cat::BoundsCheck;
                        block.insns[index + 1].cat = Cat::BoundsCheck;
                        changed = true;
                        index += 2;
                        continue;
                    }
                }
                index += 1;
            }
            if changed {
                recount(block);
            }
        }
    }

    fn meta(&self, source: &str, instrumented: &str, _instrumented_bytes: &[u8]) -> Result<Meta> {
        let mut functions = Vec::new();
        let imported = u32::try_from(self.funcs.iter().filter(|f| f.imported).count())?;
        for func in &self.funcs {
            let blocks: Vec<u32> = self
                .blocks
                .iter()
                .filter(|block| block.meta.func == func.index)
                .map(|block| block.meta.id)
                .collect();
            let static_instructions = blocks
                .iter()
                .map(|id| u64::from(self.blocks[*id as usize].meta.instructions))
                .sum();
            functions.push(FunctionMeta {
                index: func.index,
                name: func.name.clone(),
                demangled: func.demangled.clone(),
                imported: func.imported,
                type_index: func.type_index,
                body_bytes: func.body_bytes,
                static_instructions,
                blocks,
            });
        }
        let _ = imported;
        let mut sections = Vec::new();
        for section in scan_sections(self.bytes)? {
            sections.push(SectionMeta {
                id: section.id,
                name: section_name(section.id).to_owned(),
                bytes: u64::try_from(section.data.len()).unwrap_or(u64::MAX),
            });
        }
        let mut data_segments = 0usize;
        let mut data_bytes = 0u64;
        for payload in Parser::new(0).parse_all(self.bytes) {
            if let Payload::DataSection(section) = payload? {
                data_segments = section.count() as usize;
                for datum in section {
                    let datum = datum?;
                    data_bytes += u64::try_from(datum.data.len()).unwrap_or(0);
                }
            }
        }
        Ok(Meta {
            source_wasm: source.to_owned(),
            instrumented_wasm: instrumented.to_owned(),
            source_bytes: u64::try_from(self.bytes.len()).unwrap_or(u64::MAX),
            functions,
            blocks: self.blocks.iter().map(|block| block.meta.clone()).collect(),
            stack_pointer_global: self.stack_pointer_global,
            imported_globals: self.imported_globals,
            original_globals: self.original_globals,
            original_sections: sections,
            data_segments,
            data_bytes,
        })
    }
}

// ---------------------------------------------------------------------------
// Parsing helpers
// ---------------------------------------------------------------------------

struct SourceSection {
    id: u8,
    data: Vec<u8>,
}

fn scan_sections(bytes: &[u8]) -> Result<Vec<SourceSection>> {
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
        if end > bytes.len() {
            bail!("section {id} runs past the end of the module");
        }
        sections.push(SourceSection {
            id,
            data: bytes[offset..end].to_vec(),
        });
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

struct ParsedModule {
    imported_funcs: u32,
    funcs: Vec<FunctionInfo>,
    func_types: Vec<u32>,
    type_params: Vec<usize>,
    type_results: Vec<usize>,
    imported_globals: u32,
    defined_globals: Vec<(GlobalType, ConstExpr)>,
    defined_global_inits: Vec<Option<i64>>,
    original_globals: u32,
}

#[expect(
    clippy::too_many_lines,
    clippy::excessive_nesting,
    clippy::collapsible_if,
    reason = "the payload match mirrors the wasm section order"
)]
fn parse_module(bytes: &[u8]) -> Result<ParsedModule> {
    let mut names = HashMap::new();
    let mut type_params = Vec::new();
    let mut type_results = Vec::new();
    let mut funcs = Vec::new();
    let mut func_types = Vec::new();
    let mut imported_globals = 0u32;
    let mut defined_globals = Vec::new();
    let mut defined_global_inits = Vec::new();
    let mut defined_types = Vec::new();
    let mut imported_func_names = Vec::new();

    // Name section first so imported and defined functions can be named.
    for payload in Parser::new(0).parse_all(bytes) {
        if let Payload::CustomSection(section) = payload? {
            if let wasmparser::KnownCustom::Name(reader) = section.as_known() {
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
    }

    for payload in Parser::new(0).parse_all(bytes) {
        match payload? {
            Payload::TypeSection(section) => {
                for group in section {
                    let group = group?;
                    for ty in group.into_types() {
                        if let wasmparser::CompositeInnerType::Func(func) = ty.composite_type.inner
                        {
                            type_params.push(func.params().len());
                            type_results.push(func.results().len());
                        } else {
                            type_params.push(0);
                            type_results.push(0);
                        }
                    }
                }
            }
            Payload::ImportSection(section) => {
                for import in section.into_imports() {
                    let import = import?;
                    match import.ty {
                        TypeRef::Func(type_index) => {
                            let index = u32::try_from(funcs.len())?;
                            let name = names.get(&index).cloned().unwrap_or_else(|| {
                                format!("import_{}:{}", import.module, import.name)
                            });
                            imported_func_names.push(name.clone());
                            funcs.push(FunctionInfo {
                                index,
                                name: name.clone(),
                                demangled: demangle(&name),
                                imported: true,
                                type_index,
                                body_bytes: 0,
                            });
                            func_types.push(type_index);
                        }
                        TypeRef::Global(_) => imported_globals += 1,
                        _ => {}
                    }
                }
            }
            Payload::FunctionSection(section) => {
                for type_index in section {
                    defined_types.push(type_index?);
                }
            }
            Payload::GlobalSection(section) => {
                for global in section {
                    let global = global?;
                    let mut reencoder = RoundtripReencoder;
                    let ty = reencoder.global_type(global.ty)?;
                    let mut init_value = None;
                    let mut init_reader = global.init_expr.get_operators_reader();
                    if let Ok(Operator::I32Const { value }) = init_reader.read() {
                        init_value = Some(i64::from(value));
                    }
                    let init = reencoder.const_expr(global.init_expr)?;
                    defined_globals.push((ty, init));
                    defined_global_inits.push(init_value);
                }
            }
            Payload::CodeSectionEntry(body) => {
                let defined_index = u32::try_from(funcs.iter().filter(|f| !f.imported).count())?;
                let index = u32::try_from(imported_func_names.len())? + defined_index;
                let type_index = defined_types
                    .get(defined_index as usize)
                    .copied()
                    .unwrap_or(0);
                let name = names
                    .get(&index)
                    .cloned()
                    .unwrap_or_else(|| format!("func_{index}"));
                funcs.push(FunctionInfo {
                    index,
                    name: name.clone(),
                    demangled: demangle(&name),
                    imported: false,
                    type_index,
                    body_bytes: body.range().end - body.range().start,
                });
                func_types.push(type_index);
            }
            _ => {}
        }
    }
    funcs.sort_by_key(|func| func.index);
    let original_globals =
        imported_globals + u32::try_from(defined_globals.len()).expect("module globals fit u32");
    Ok(ParsedModule {
        imported_funcs: u32::try_from(imported_func_names.len()).expect("module functions fit u32"),
        funcs,
        func_types,
        type_params,
        type_results,
        imported_globals,
        defined_globals,
        defined_global_inits,
        original_globals,
    })
}

fn find_stack_pointer(
    globals: &[(GlobalType, ConstExpr)],
    inits: &[Option<i64>],
    imported_globals: u32,
) -> Option<u32> {
    let mut candidates = Vec::new();
    for (offset, (ty, _)) in globals.iter().enumerate() {
        if ty.val_type == ValType::I32 && ty.mutable && !ty.shared {
            let value = inits.get(offset).copied().flatten().unwrap_or(0);
            candidates.push((
                value,
                imported_globals + u32::try_from(offset).expect("module globals fit u32"),
            ));
        }
    }
    candidates.sort_by_key(|(value, index)| (*value, *index));
    candidates.last().map(|(_, index)| *index)
}

fn collect_bodies(bytes: &[u8]) -> Result<Vec<FunctionBody<'_>>> {
    let mut bodies = Vec::new();
    for payload in Parser::new(0).parse_all(bytes) {
        if let Payload::CodeSectionEntry(body) = payload? {
            bodies.push(body);
        }
    }
    Ok(bodies)
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

fn is_block_start(op: &Operator<'_>) -> bool {
    matches!(
        op,
        Operator::Block { .. }
            | Operator::Loop { .. }
            | Operator::If { .. }
            | Operator::Try { .. }
            | Operator::TryTable { .. }
    )
}

fn is_block_end(op: &Operator<'_>) -> bool {
    matches!(op, Operator::End)
}

fn is_unconditional_end(op: &Operator<'_>) -> bool {
    matches!(
        op,
        Operator::Br { .. }
            | Operator::BrTable { .. }
            | Operator::Return
            | Operator::Unreachable
            | Operator::Throw { .. }
            | Operator::Rethrow { .. }
            | Operator::ReturnCall { .. }
            | Operator::ReturnCallIndirect { .. }
    )
}

fn branch_depths(op: &Operator<'_>) -> Option<Vec<u32>> {
    match op {
        Operator::Br { relative_depth } | Operator::BrIf { relative_depth } => {
            Some(vec![*relative_depth])
        }
        Operator::BrTable { targets } => {
            let mut depths: Vec<u32> = targets.targets().filter_map(Result::ok).collect();
            depths.push(targets.default());
            Some(depths)
        }
        _ => None,
    }
}

fn is_int_compare(op: &Operator<'_>) -> bool {
    matches!(
        op,
        Operator::I32Eqz
            | Operator::I32Eq
            | Operator::I32Ne
            | Operator::I32LtS
            | Operator::I32LtU
            | Operator::I32GtS
            | Operator::I32GtU
            | Operator::I32LeS
            | Operator::I32LeU
            | Operator::I32GeS
            | Operator::I32GeU
            | Operator::I64Eqz
            | Operator::I64Eq
            | Operator::I64Ne
            | Operator::I64LtS
            | Operator::I64LtU
            | Operator::I64GtS
            | Operator::I64GtU
            | Operator::I64LeS
            | Operator::I64LeU
            | Operator::I64GeS
            | Operator::I64GeU
    )
}

fn call_target(op: &Operator<'_>, funcs: &[FunctionInfo]) -> Option<CallSite> {
    let (target, kind) = match op {
        Operator::Call { function_index } => (*function_index, "direct"),
        _ => return None,
    };
    let func = funcs.get(target as usize)?;
    Some(CallSite {
        target,
        name: func.name.clone(),
        demangled: func.demangled.clone(),
        kind: kind.to_owned(),
    })
}

fn is_panic_operator(op: &Operator<'_>) -> bool {
    matches!(
        op,
        Operator::Unreachable | Operator::Throw { .. } | Operator::Rethrow { .. }
    )
}

fn is_panic_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    [
        "panic",
        "unreachable",
        "unwrap_failed",
        "index_out_of_bounds",
        "slice_index",
        "capacity_overflow",
        "handle_alloc_error",
        "rust_begin_unwind",
        "bad_alloc",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn recount(block: &mut BlockBuilder) {
    block.meta.categories.clear();
    block.meta.instructions = 0;
    block.meta.stack_loads = 0;
    block.meta.stack_stores = 0;
    block.meta.heap_loads = 0;
    block.meta.heap_stores = 0;
    block.meta.bounds_check_pairs = 0;
    for insn in &block.insns {
        if insn.cat.structural() {
            continue;
        }
        block.meta.instructions += 1;
        *block
            .meta
            .categories
            .entry(insn.cat.as_str().to_owned())
            .or_insert(0) += 1;
        match insn.cat {
            Cat::LoadStack => block.meta.stack_loads += 1,
            Cat::StoreStack => block.meta.stack_stores += 1,
            Cat::LoadHeap => block.meta.heap_loads += 1,
            Cat::StoreHeap => block.meta.heap_stores += 1,
            Cat::BoundsCheck => block.meta.bounds_check_pairs += 1,
            _ => {}
        }
    }
}

// A coarse stack effect for the first-pass taint simulation. The exact counts
// matter only for keeping the operand stack aligned; unknown operators are
// treated as leaving a single non-stack value.
#[expect(
    clippy::match_same_arms,
    reason = "the opcode table groups operators by stack effect; repeating the \
              arm per operator is intentional documentation"
)]
fn stack_effect(
    op: &Operator<'_>,
    _type_params: &[usize],
    _type_results: &[usize],
) -> (usize, usize, bool) {
    use Operator::{
        BrIf, Call, CallIndirect, Drop, F32Abs, F32Add, F32Ceil, F32Const, F32ConvertI32S,
        F32ConvertI32U, F32ConvertI64S, F32ConvertI64U, F32Copysign, F32DemoteF64, F32Div, F32Eq,
        F32Floor, F32Ge, F32Gt, F32Le, F32Load, F32Lt, F32Max, F32Min, F32Mul, F32Ne, F32Nearest,
        F32Neg, F32ReinterpretI32, F32Sqrt, F32Store, F32Sub, F32Trunc, F64Abs, F64Add, F64Ceil,
        F64Const, F64ConvertI32S, F64ConvertI32U, F64ConvertI64S, F64ConvertI64U, F64Copysign,
        F64Div, F64Eq, F64Floor, F64Ge, F64Gt, F64Le, F64Load, F64Lt, F64Max, F64Min, F64Mul,
        F64Ne, F64Nearest, F64Neg, F64PromoteF32, F64ReinterpretI64, F64Sqrt, F64Store, F64Sub,
        F64Trunc, GlobalGet, GlobalSet, I32Add, I32And, I32Clz, I32Const, I32Ctz, I32DivS, I32DivU,
        I32Eq, I32Eqz, I32Extend8S, I32Extend16S, I32GeS, I32GeU, I32GtS, I32GtU, I32LeS, I32LeU,
        I32Load, I32Load8S, I32Load8U, I32Load16S, I32Load16U, I32LtS, I32LtU, I32Mul, I32Ne,
        I32Or, I32Popcnt, I32ReinterpretF32, I32RemS, I32RemU, I32Rotl, I32Rotr, I32Shl, I32ShrS,
        I32ShrU, I32Store, I32Store8, I32Store16, I32Sub, I32TruncF32S, I32TruncF32U, I32TruncF64S,
        I32TruncF64U, I32TruncSatF32S, I32TruncSatF32U, I32TruncSatF64S, I32TruncSatF64U,
        I32WrapI64, I32Xor, I64Add, I64And, I64Clz, I64Const, I64Ctz, I64DivS, I64DivU, I64Eq,
        I64Eqz, I64Extend8S, I64Extend16S, I64Extend32S, I64ExtendI32S, I64ExtendI32U, I64GeS,
        I64GeU, I64GtS, I64GtU, I64LeS, I64LeU, I64Load, I64Load8S, I64Load8U, I64Load16S,
        I64Load16U, I64Load32S, I64Load32U, I64LtS, I64LtU, I64Mul, I64Ne, I64Or, I64Popcnt,
        I64ReinterpretF64, I64RemS, I64RemU, I64Rotl, I64Rotr, I64Shl, I64ShrS, I64ShrU, I64Store,
        I64Store8, I64Store16, I64Store32, I64Sub, I64TruncF32S, I64TruncF32U, I64TruncF64S,
        I64TruncF64U, I64TruncSatF32S, I64TruncSatF32U, I64TruncSatF64S, I64TruncSatF64U, I64Xor,
        LocalGet, LocalSet, LocalTee, MemoryGrow, MemorySize, RefFunc, RefNull, Select,
        TypedSelect,
    };
    match op {
        LocalGet { .. }
        | GlobalGet { .. }
        | I32Const { .. }
        | I64Const { .. }
        | F32Const { .. }
        | F64Const { .. }
        | MemorySize { .. }
        | RefNull { .. }
        | RefFunc { .. } => (0, 1, false),
        LocalSet { .. } | GlobalSet { .. } | Drop | BrIf { .. } => (1, 0, false),
        LocalTee { .. } => (1, 1, true),
        I32Load { .. }
        | I64Load { .. }
        | F32Load { .. }
        | F64Load { .. }
        | I32Load8S { .. }
        | I32Load8U { .. }
        | I32Load16S { .. }
        | I32Load16U { .. }
        | I64Load8S { .. }
        | I64Load8U { .. }
        | I64Load16S { .. }
        | I64Load16U { .. }
        | I64Load32S { .. }
        | I64Load32U { .. } => (1, 1, false),
        I32Store { .. }
        | I64Store { .. }
        | F32Store { .. }
        | F64Store { .. }
        | I32Store8 { .. }
        | I32Store16 { .. }
        | I64Store8 { .. }
        | I64Store16 { .. }
        | I64Store32 { .. } => (2, 0, false),
        Call { .. } | CallIndirect { .. } => (0, 0, false),
        I32Add | I32Sub | I32Mul | I32DivS | I32DivU | I32RemS | I32RemU | I32And | I32Or
        | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr | I64Add | I64Sub | I64Mul
        | I64DivS | I64DivU | I64RemS | I64RemU | I64And | I64Or | I64Xor | I64Shl | I64ShrS
        | I64ShrU | I64Rotl | I64Rotr => (2, 1, true),
        I32Clz | I32Ctz | I32Popcnt | I64Clz | I64Ctz | I64Popcnt => (1, 1, true),
        I32Eqz | I64Eqz => (1, 1, false),
        I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU
        | I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU
        | F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge | F64Eq | F64Ne | F64Lt | F64Gt | F64Le
        | F64Ge => (2, 1, false),
        F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest | F32Sqrt | F64Abs
        | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest | F64Sqrt => (1, 1, false),
        F32Add | F32Sub | F32Mul | F32Div | F32Min | F32Max | F32Copysign | F64Add | F64Sub
        | F64Mul | F64Div | F64Min | F64Max | F64Copysign => (2, 1, false),
        Select | TypedSelect { .. } => (3, 1, true),
        MemoryGrow { .. } => (1, 1, false),
        I32WrapI64 | I64ExtendI32S | I64ExtendI32U | I32TruncF32S | I32TruncF32U | I32TruncF64S
        | I32TruncF64U | I64TruncF32S | I64TruncF32U | I64TruncF64S | I64TruncF64U
        | I32TruncSatF32S | I32TruncSatF32U | I32TruncSatF64S | I32TruncSatF64U
        | I64TruncSatF32S | I64TruncSatF32U | I64TruncSatF64S | I64TruncSatF64U
        | F32ConvertI32S | F32ConvertI32U | F32ConvertI64S | F32ConvertI64U | F64ConvertI32S
        | F64ConvertI32U | F64ConvertI64S | F64ConvertI64U | F32DemoteF64 | F64PromoteF32
        | I32ReinterpretF32 | I64ReinterpretF64 | F32ReinterpretI32 | F64ReinterpretI64
        | I32Extend8S | I32Extend16S | I64Extend8S | I64Extend16S | I64Extend32S => (1, 1, false),
        _ => (0, 0, false),
    }
}

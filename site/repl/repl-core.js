// The in-browser compiler behind the site's REPL: rustc.wasm and wild.wasm on
// an in-memory WASI filesystem, with no server and no native code.
//
// rustc.wasm is single-threaded WASI preview 1, so one synchronous WASI shim
// (vendor/browser_wasi_shim) runs it inside a Web Worker. rustc asks its host
// to run the linker through the `playground.spawn` import; here that runs
// wild.wasm to completion on the same filesystem. Every compile gets a fresh
// rustc instance from the already-compiled module.
//
// This module is plain ES and runs unchanged in a worker (repl-worker.js) and
// in Node (site/repl/tests/smoke.mjs, run by CI).

import {
  WASI,
  File,
  Directory,
  OpenFile,
  ConsoleStdout,
  PreopenDirectory,
} from "./vendor/browser_wasi_shim/index.js";

// The maths a no_std program lacks. core already has abs, min, max, clamp,
// signum and copysign on f32; everything here comes from the page's Math as a
// wasm import ("math" module) and is offered both as a free function,
// floor(x), and as a method, x.floor(), the way std spells it. One table
// drives the Rust prelude below and PROGRAM_IMPORTS.
const MATH = [
  // [Rust name, JS Math function, arity]
  ["sin", "sin", 1], ["cos", "cos", 1], ["tan", "tan", 1],
  ["asin", "asin", 1], ["acos", "acos", 1], ["atan", "atan", 1],
  ["sinh", "sinh", 1], ["cosh", "cosh", 1], ["tanh", "tanh", 1],
  ["sqrt", "sqrt", 1], ["cbrt", "cbrt", 1], ["exp", "exp", 1],
  ["ln", "log", 1], ["log2", "log2", 1], ["log10", "log10", 1],
  ["floor", "floor", 1], ["ceil", "ceil", 1], ["trunc", "trunc", 1],
  ["atan2", "atan2", 2], ["powf", "pow", 2], ["hypot", "hypot", 2],
];

function preludeSource() {
  // Two-argument functions take (y, x) for atan2 and (x, y) otherwise, as std does.
  const params = (arity) => (arity === 1 ? "x: f32" : "x: f32, y: f32");
  const args = (arity) => (arity === 1 ? "x" : "x, y");
  const imports = MATH.map(([name, , arity]) =>
    `            #[link_name = "${name}"]\n            pub fn ${name}(${params(arity)}) -> f32;`).join("\n");
  const free = MATH.map(([name, , arity]) =>
    `    pub fn ${name}(${params(arity)}) -> f32 { unsafe { host::${name}(${args(arity)}) } }`).join("\n");
  const decl = MATH.map(([name, , arity]) =>
    `        fn ${name}(self${arity === 2 ? ", y: f32" : ""}) -> f32;`).join("\n");
  const impl = MATH.map(([name, , arity]) =>
    `        fn ${name}(self${arity === 2 ? ", y: f32" : ""}) -> f32 { ${name}(self${arity === 2 ? ", y" : ""}) }`).join("\n");
  return `// Glob-imported, so a function of the visitor's own with the same name wins.
#[allow(unused_imports)]
use __prelude::*;
#[allow(dead_code)]
mod __prelude {
    #[allow(unused_imports)]
    pub use core::f32::consts::{E, PI, TAU};
    mod host {
        #[link(wasm_import_module = "math")]
        unsafe extern "C" {
${imports}
        }
    }
${free}
    /// Rust's rounding: halfway cases away from zero.
    pub fn round(x: f32) -> f32 { if x < 0.0 { -floor(0.5 - x) } else { floor(x + 0.5) } }
    pub fn fract(x: f32) -> f32 { x - trunc(x) }
    pub fn powi(x: f32, n: i32) -> f32 { powf(x, n as f32) }
    pub fn abs(x: f32) -> f32 { x.abs() }
    pub fn min(x: f32, y: f32) -> f32 { x.min(y) }
    pub fn max(x: f32, y: f32) -> f32 { x.max(y) }
    /// Not core's f32::clamp: its assert formats floats into a panic message,
    /// which links about 110 KB of float formatting into every program.
    pub fn clamp(x: f32, lo: f32, hi: f32) -> f32 { if x < lo { lo } else if x > hi { hi } else { x } }
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a) * t }
    /// The std f32 methods that no_std core lacks.
    pub trait FloatMath {
${decl}
        fn round(self) -> f32;
        fn fract(self) -> f32;
        fn powi(self, n: i32) -> f32;
    }
    impl FloatMath for f32 {
${impl}
        fn round(self) -> f32 { round(self) }
        fn fract(self) -> f32 { fract(self) }
        fn powi(self, n: i32) -> f32 { powi(self, n) }
    }
}`;
}

// Appended after the visitor's code, so rustc's line numbers in errors match
// the editor. The crate is \`no_std\` (passed as -Zcrate-attr), the maths
// prelude above stands in for std's f32 functions, and \`frame\` calls the
// visitor's \`particle\` for every particle and returns the positions.
export const HARNESS = `

// ---- appended by the page ------------------------------------------------
#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
${preludeSource()}
const __MAX: usize = 4096;
static mut __POSITIONS: [f32; 2 * __MAX] = [0.0; 2 * __MAX];
#[unsafe(no_mangle)]
pub extern "C" fn frame(n: u32, t: f32) -> *const f32 {
    let n = (n as usize).min(__MAX);
    let positions = unsafe { &mut *core::ptr::addr_of_mut!(__POSITIONS) };
    for i in 0..n {
        let (x, y) = particle(i as f32, n as f32, t);
        positions[2 * i] = x;
        positions[2 * i + 1] = y;
    }
    positions.as_ptr()
}
`;

// The flags the benchmarks and the supervisor use, plus no_std from outside.
export const RUSTC_ARGS = [
  "rustc",
  "/work/src/particles.rs",
  "--crate-name",
  "main",
  "--crate-type=cdylib",
  "--edition=2021",
  "--target=wasm32-unknown-unknown",
  "--sysroot",
  "/sysroot",
  "-Zcrate-attr=no_std",
  "-Copt-level=2",
  "-Cpanic=abort",
  "-Zcodegen-backend=cranelift",
  "-Clinker=wild",
  "-Clinker-flavor=wasm-ld",
  "--color=never",
  // Errors name the file as the editor does: src/particles.rs.
  "--remap-path-prefix=/work/=",
  "-o",
  "/work/out.wasm",
];

// Lines the visitor wrote, so errors can be cut at the harness.
export function userLineCount(source) {
  return source.split("\n").length;
}

function readCString(memory, pointer) {
  const bytes = new Uint8Array(memory.buffer);
  let end = pointer;
  while (bytes[end] !== 0) end += 1;
  return new TextDecoder().decode(bytes.subarray(pointer, end));
}

// Build a Directory tree from { "lib/rustlib/.../libcore.rmeta": Uint8Array }.
function directoryFrom(files) {
  const root = new Map();
  for (const [path, data] of Object.entries(files)) {
    const parts = path.split("/");
    let dir = root;
    for (const part of parts.slice(0, -1)) {
      if (!dir.has(part)) dir.set(part, new Map());
      dir = dir.get(part);
    }
    dir.set(parts.at(-1), new File(data, { readonly: true }));
  }
  const build = (map) =>
    new Directory(
      [...map].map(([name, entry]) => [name, entry instanceof Map ? build(entry) : entry]),
    );
  return build(root);
}

export class Compiler {
  // rustc, wild: WebAssembly.Module. sysroot: { relativePath: Uint8Array }.
  constructor({ rustc, wild, sysroot }) {
    this.rustc = rustc;
    this.wild = wild;
    this.sysroot = directoryFrom(sysroot);
  }

  // Compile the visitor's code. Returns { ok, wasm, stderr, compileMs, linkMs }.
  compile(userSource) {
    const encoder = new TextEncoder();
    const work = new Directory([
      ["src", new Directory([["particles.rs", new File(encoder.encode(userSource + HARNESS))]])],
    ]);
    const root = new PreopenDirectory("/", [
      ["sysroot", this.sysroot],
      ["work", work],
      ["tmp", new Directory([])],
    ]);
    const stderr = [];
    const sink = () => new ConsoleStdout((bytes) => stderr.push(new TextDecoder().decode(bytes)));
    const run = (module, args, imports = () => ({})) => {
      // The shim logs every call unless debug is explicitly off.
      const wasi = new WASI(args, ["TMPDIR=/tmp"], [new OpenFile(new File([])), sink(), sink(), root], {
        debug: false,
      });
      let instance;
      instance = new WebAssembly.Instance(module, {
        wasi_snapshot_preview1: wasi.wasiImport,
        ...imports(() => instance),
      });
      return wasi.start(instance);
    };

    let linkMs = 0;
    const started = performance.now();
    const code = run(this.rustc, RUSTC_ARGS, (instance) => ({
      playground: {
        // rustc's linker call: argv[0] is the linker name, the rest its args.
        spawn: (argvPointer, argc) => {
          const memory = instance().exports.memory;
          const view = new DataView(memory.buffer);
          const argv = [];
          for (let i = 0; i < argc; i += 1) {
            argv.push(readCString(memory, view.getUint32(argvPointer + 4 * i, true)));
          }
          const linkStarted = performance.now();
          const status = run(this.wild, ["wild", ...argv.slice(1)]);
          linkMs = performance.now() - linkStarted;
          return status;
        },
      },
      // The serving loop's imports; off here, as in a plain host.
      cogwasm: { next_request: () => 0, finish: () => {} },
    }));
    const totalMs = performance.now() - started;
    const output = work.contents.get("out.wasm");
    return {
      ok: code === 0 && output !== undefined,
      wasm: output ? output.data : null,
      stderr: stderr.join(""),
      compileMs: totalMs - linkMs,
      linkMs,
    };
  }
}

// What the compiled program imports: the page's maths, per the MATH table.
export const PROGRAM_IMPORTS = {
  math: Object.fromEntries(MATH.map(([name, js]) => [name, Math[js]])),
};

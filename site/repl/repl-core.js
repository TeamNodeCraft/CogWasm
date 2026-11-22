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

// Appended after the visitor's code, so rustc's line numbers in errors match
// the editor. The crate is `no_std` (passed as -Zcrate-attr), the page supplies
// sin/cos/sqrt as wasm imports, and `frame` calls the visitor's `particle` for
// every particle and returns a pointer to the positions.
export const HARNESS = `

// ---- appended by the page ------------------------------------------------
#[panic_handler]
fn __panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}
#[allow(unused_imports)]
use core::f32::consts::{PI, TAU};
#[link(wasm_import_module = "math")]
unsafe extern "C" {
    #[link_name = "sin"]
    fn __sin(x: f32) -> f32;
    #[link_name = "cos"]
    fn __cos(x: f32) -> f32;
    #[link_name = "sqrt"]
    fn __sqrt(x: f32) -> f32;
}
#[allow(dead_code)]
fn sin(x: f32) -> f32 { unsafe { __sin(x) } }
#[allow(dead_code)]
fn cos(x: f32) -> f32 { unsafe { __cos(x) } }
#[allow(dead_code)]
fn sqrt(x: f32) -> f32 { unsafe { __sqrt(x) } }
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

// What the compiled program imports: the page's maths.
export const PROGRAM_IMPORTS = {
  math: { sin: Math.sin, cos: Math.cos, sqrt: Math.sqrt },
};

// End-to-end check of the in-browser REPL's compiler, in Node (same V8 as
// Chrome): load the packaged assets, compile every example and a broken
// program, run the output, and report timings.
//
//   node site/repl/tests/smoke.mjs [assets-dir]   (default toolchain/work/repl-assets)
//
// CI runs it after ci/build-repl-assets.sh. Exits non-zero on any failure.

import { readFileSync } from "node:fs";
import { gunzipSync } from "node:zlib";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { Compiler, PROGRAM_IMPORTS, userLineCount } from "../repl-core.js";
import { EXAMPLES } from "../examples.js";

const here = path.dirname(fileURLToPath(import.meta.url));
const assets = process.argv[2] ?? path.resolve(here, "../../../toolchain/work/repl-assets");
const manifest = JSON.parse(readFileSync(path.join(assets, "manifest.json"), "utf8"));
const read = (entry) => gunzipSync(readFileSync(path.join(assets, entry.file)));

const loadStarted = performance.now();
const [rustc, wild] = await Promise.all([
  WebAssembly.compile(read(manifest.rustc)),
  WebAssembly.compile(read(manifest.wild)),
]);
const sysroot = Object.fromEntries(manifest.sysroot.map((entry) => [entry.path, read(entry)]));
const compiler = new Compiler({ rustc, wild, sysroot });
console.log(
  `loaded ${(manifest.download_bytes / 1e6).toFixed(1)} MB of assets in ${(performance.now() - loadStarted).toFixed(0)} ms`,
);

let failures = 0;
const fail = (message) => {
  failures += 1;
  console.error(`FAIL: ${message}`);
};

function check(name, source) {
  const result = compiler.compile(source);
  if (!result.ok) {
    fail(`${name} did not compile:\n${result.stderr}`);
    return;
  }
  const { exports } = new WebAssembly.Instance(new WebAssembly.Module(result.wasm), PROGRAM_IMPORTS);
  const n = 400;
  const pointer = exports.frame(n, 1.5);
  const positions = new Float32Array(exports.memory.buffer, pointer, 2 * n);
  const finite = positions.every(Number.isFinite);
  const spread = Math.max(...positions) - Math.min(...positions);
  if (!finite || spread <= 0) fail(`${name}: positions are not finite and spread (spread ${spread})`);
  console.log(
    `${name.padEnd(10)} compile ${result.compileMs.toFixed(0).padStart(4)} ms  link ${result.linkMs.toFixed(0).padStart(3)} ms  ${result.wasm.length} bytes  ok`,
  );
}

// Twice through, so the second pass shows the warmed-up compiler.
for (const pass of [1, 2]) {
  for (const example of EXAMPLES) check(`${example.name}${pass === 2 ? "*" : ""}`, example.source);
}

// A type error must fail with rustc's message, pointing at the visitor's line.
const broken = "fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {\n    (i, \"no\")\n}\n";
const result = compiler.compile(broken);
if (result.ok) fail("a program with a type error compiled");
else if (!/mismatched types/.test(result.stderr) || !/src\/particles\.rs:2:/.test(result.stderr)) {
  fail(`unexpected error output for the broken program:\n${result.stderr}`);
} else {
  console.log(`broken     rejected at src/particles.rs:2 (of ${userLineCount(broken)} visitor lines)  ok`);
}

if (failures > 0) {
  console.error(`${failures} failure(s)`);
  process.exit(1);
}
console.log("REPL smoke test passed");

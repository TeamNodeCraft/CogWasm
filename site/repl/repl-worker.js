// The REPL's compile worker. The page starts it only when a visitor asks to
// load the compiler; it downloads the packaged assets (see
// ci/build-repl-assets.sh), then compiles on request with repl-core.js.
//
// Messages in:  { type: "load", base }  then  { type: "compile", id, source }
// Messages out: { type: "progress", loaded, total }, { type: "ready", ms, commit },
//               { type: "result", id, ok, wasm, stderr, compileMs, linkMs },
//               { type: "error", message }

import { Compiler } from "./repl-core.js";

let compiler = null;

// Fetch one gzipped asset; count compressed bytes as they arrive and return
// a stream of the decompressed bytes.
async function fetchGzip(url, onBytes) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
  const counter = new TransformStream({
    transform(chunk, controller) {
      onBytes(chunk.byteLength);
      controller.enqueue(chunk);
    },
  });
  const counted = response.body.pipeThrough(counter);
  // A server that labels the file Content-Encoding: gzip has already had the
  // browser decompress it; GitHub Pages serves .gz files as plain bytes.
  const encoding = response.headers.get("Content-Encoding") || "";
  return /gzip/i.test(encoding) ? counted : counted.pipeThrough(new DecompressionStream("gzip"));
}

async function load(base) {
  const started = performance.now();
  const manifestResponse = await fetch(new URL("manifest.json", base));
  if (!manifestResponse.ok) throw new Error(`the compiler is not published here (HTTP ${manifestResponse.status})`);
  const manifest = await manifestResponse.json();

  let loaded = 0;
  let lastReport = 0;
  const total = manifest.download_bytes;
  const onBytes = (count) => {
    loaded += count;
    const now = performance.now();
    if (now - lastReport > 80 || loaded >= total) {
      lastReport = now;
      postMessage({ type: "progress", loaded, total });
    }
  };
  // Compile the modules while they stream in.
  const wasmModule = async (entry) =>
    WebAssembly.compileStreaming(
      new Response(await fetchGzip(new URL(entry.file, base), onBytes), {
        headers: { "Content-Type": "application/wasm" },
      }),
    );
  const sysrootFile = async (entry) => [
    entry.path,
    new Uint8Array(await new Response(await fetchGzip(new URL(entry.file, base), onBytes)).arrayBuffer()),
  ];
  const [rustc, wild, ...files] = await Promise.all([
    wasmModule(manifest.rustc),
    wasmModule(manifest.wild),
    ...manifest.sysroot.map(sysrootFile),
  ]);
  compiler = new Compiler({ rustc, wild, sysroot: Object.fromEntries(files) });
  postMessage({ type: "ready", ms: performance.now() - started, commit: manifest.commit });
}

onmessage = async ({ data }) => {
  try {
    if (data.type === "load") {
      await load(data.base);
    } else if (data.type === "compile") {
      const result = compiler.compile(data.source);
      const wasm = result.wasm ? result.wasm.slice().buffer : null;
      postMessage({ type: "result", id: data.id, ...result, wasm }, wasm ? [wasm] : []);
    }
  } catch (error) {
    postMessage({ type: "error", id: data.id, message: String(error && error.message ? error.message : error) });
  }
};

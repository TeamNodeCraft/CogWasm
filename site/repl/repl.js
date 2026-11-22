// The site's live REPL: an editor, a compile worker (repl-worker.js) that runs
// rustc.wasm and wild.wasm in this page, and a canvas driven by the compiled
// `frame` export. Each successful build is swapped into the running animation,
// which keeps its clock and trails; a failed build leaves the last good program
// running. Nothing is downloaded until the visitor presses "Load the compiler".

import { EXAMPLES } from "./examples.js";
import { PROGRAM_IMPORTS } from "./repl-core.js";

// --- Rust highlighting ------------------------------------------------------

const KEYWORDS = new Set(
  ("as break const continue else enum extern false fn for if impl in let loop match mod move mut pub " +
    "ref return self Self static struct trait true type unsafe use where while").split(" "),
);
const TYPES = new Set("f32 f64 i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize bool char str".split(" "));
const TOKEN = /(\/\/[^\n]*)|("(?:[^"\\]|\\.)*"?)|(\b\d[\d_]*(?:\.\d[\d_]*)?(?:[eE][+-]?\d+)?(?:f32|f64|u32|usize|i32)?\b)|([A-Za-z_]\w*)(\s*\()?|([^\w\s"/]+|\/)/g;

function escapeHtml(text) {
  return text.replace(/[&<>]/g, (ch) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;" })[ch]);
}

function highlight(source) {
  let html = "";
  let last = 0;
  for (const match of source.matchAll(TOKEN)) {
    html += escapeHtml(source.slice(last, match.index));
    last = match.index + match[0].length;
    const [text, comment, string, number, word, call] = match;
    if (comment) html += `<span class="tok-comment">${escapeHtml(comment)}</span>`;
    else if (string) html += `<span class="tok-string">${escapeHtml(string)}</span>`;
    else if (number) html += `<span class="tok-number">${number}</span>`;
    else if (word) {
      let cls = "";
      if (KEYWORDS.has(word)) cls = "tok-keyword";
      else if (TYPES.has(word) || /^[A-Z]/.test(word)) cls = /^[A-Z][A-Z0-9_]+$/.test(word) ? "tok-const" : "tok-type";
      else if (call) cls = "tok-fn";
      html += cls ? `<span class="${cls}">${word}</span>` : word;
      if (call) html += escapeHtml(call);
    } else html += escapeHtml(text);
  }
  return html + escapeHtml(source.slice(last)) + "\n";
}

// --- the REPL ----------------------------------------------------------------

function start(root) {
  const $ = (selector) => root.querySelector(selector);
  const grid = $(".repl-grid");
  const input = $(".repl-input");
  const code = $(".repl-highlight code");
  const highlightPre = $(".repl-highlight");
  const gutter = $(".repl-gutter");
  const status = $("#repl-status");
  const output = $("#repl-output");
  const canvas = $(".repl-canvas");
  const overlay = $(".repl-overlay");
  const loadButton = $(".repl-load");
  const progress = $(".repl-progress");
  const progressBar = $(".repl-progress-bar");
  const sizeLabel = $(".repl-size");
  const pauseButton = $(".repl-pause");
  const stat = (name) => root.querySelector(`[data-stat="${name}"]`);
  grid.hidden = false;

  const assetsBase = new URL("assets/", import.meta.url);
  const reducedMotion = window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  // Editor.
  function render() {
    code.innerHTML = highlight(input.value);
    const lines = input.value.split("\n").length;
    gutter.textContent = Array.from({ length: lines }, (_, i) => i + 1).join("\n");
  }
  function syncScroll() {
    highlightPre.scrollTop = gutter.scrollTop = input.scrollTop;
    highlightPre.scrollLeft = input.scrollLeft;
  }
  input.addEventListener("scroll", syncScroll);
  input.addEventListener("input", () => {
    render();
    scheduleBuild(350);
  });
  input.addEventListener("keydown", (event) => {
    if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
      event.preventDefault();
      scheduleBuild(0);
    } else if (event.key === "Tab" && !event.shiftKey && !event.altKey) {
      event.preventDefault();
      document.execCommand("insertText", false, "    "); // keeps undo history
    }
  });

  // Examples.
  const examples = $(".repl-examples");
  function choose(example, button) {
    input.value = example.source;
    for (const other of examples.querySelectorAll("button")) other.setAttribute("aria-pressed", String(other === button));
    render();
    syncScroll();
    scheduleBuild(0);
  }
  EXAMPLES.forEach((example, index) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "repl-chip";
    button.textContent = example.name;
    button.setAttribute("aria-pressed", String(index === 0));
    button.addEventListener("click", () => choose(example, button));
    examples.append(button);
  });
  input.value = EXAMPLES[0].source;
  render();

  // Show the real download size once the manifest is known.
  fetch(new URL("manifest.json", assetsBase))
    .then((response) => (response.ok ? response.json() : Promise.reject(response.status)))
    .then((manifest) => {
      sizeLabel.textContent = `${(manifest.download_bytes / 1e6).toFixed(0)} MB`;
    })
    .catch(() => {
      loadButton.disabled = true;
      loadButton.textContent = "Compiler not published yet";
      setStatus("unavailable", "idle");
    });

  function setStatus(text, kind) {
    status.textContent = text;
    status.dataset.kind = kind;
  }

  // Compile worker.
  let worker = null;
  let ready = false;
  let building = false;
  let pending = false;
  let timer = 0;
  let nextId = 0;
  let editStarted = 0;
  let edits = 0;

  loadButton.addEventListener("click", () => {
    if (worker) return;
    loadButton.disabled = true;
    loadButton.textContent = "Loading the compiler…";
    progress.hidden = false;
    setStatus("loading", "busy");
    worker = new Worker(new URL("repl-worker.js", import.meta.url), { type: "module" });
    worker.onmessage = onWorkerMessage;
    worker.onerror = (event) => showFailure(`The compile worker failed: ${event.message || "unknown error"}`);
    worker.postMessage({ type: "load", base: assetsBase.href });
  });

  function onWorkerMessage({ data }) {
    if (data.type === "progress") {
      progressBar.style.width = `${Math.min(100, (100 * data.loaded) / data.total).toFixed(1)}%`;
      loadButton.textContent = `Loading the compiler… ${(data.loaded / 1e6).toFixed(0)} of ${(data.total / 1e6).toFixed(0)} MB`;
    } else if (data.type === "ready") {
      ready = true;
      overlay.hidden = true;
      setStatus("compiling", "busy");
      scheduleBuild(0);
    } else if (data.type === "result") {
      onResult(data);
    } else if (data.type === "error") {
      if (!ready) showFailure(`Could not load the compiler: ${data.message}`);
      else {
        building = false;
        showOutput(`The compiler stopped: ${data.message}`, "error");
        setStatus("error", "error");
      }
    }
  }

  function showFailure(message) {
    loadButton.disabled = false;
    loadButton.textContent = "Try again";
    progress.hidden = true;
    worker && worker.terminate();
    worker = null;
    setStatus("not loaded", "idle");
    showOutput(message, "error");
  }

  function scheduleBuild(delay) {
    if (!ready) return;
    clearTimeout(timer);
    timer = setTimeout(build, delay);
  }

  function build() {
    if (building) {
      pending = true;
      return;
    }
    building = true;
    pending = false;
    editStarted = performance.now();
    setStatus("compiling", "busy");
    worker.postMessage({ type: "compile", id: ++nextId, source: input.value });
  }

  async function onResult(result) {
    building = false;
    if (result.ok) {
      try {
        const swapStarted = performance.now();
        const { instance } = await WebAssembly.instantiate(result.wasm, PROGRAM_IMPORTS);
        program = instance.exports;
        const swapMs = performance.now() - swapStarted;
        edits += 1;
        stat("compile").textContent = `${result.compileMs.toFixed(0)} ms`;
        stat("link").textContent = `${result.linkMs.toFixed(0)} ms`;
        stat("swap").textContent = `${swapMs.toFixed(1)} ms`;
        stat("edits").textContent = String(edits);
        const total = performance.now() - editStarted;
        setStatus(`live · ${total.toFixed(0)} ms`, "live");
        const warnings = result.stderr.trim();
        if (warnings) showOutput(warnings, "warning");
        else output.hidden = true;
        if (paused === null) setPaused(reducedMotion);
      } catch (error) {
        showOutput(`The new program could not start: ${error.message}`, "error");
        setStatus("error · still running the last build", "error");
      }
    } else {
      showOutput(result.stderr.trim() || "The build failed without a message.", "error");
      setStatus(program ? "error · still running the last build" : "error", "error");
    }
    if (pending) build();
  }

  function showOutput(text, kind) {
    output.textContent = text;
    output.dataset.kind = kind;
    output.hidden = false;
  }

  // Running program.
  const context = canvas.getContext("2d");
  const PARTICLES = 600;
  const TRAIL = 7;
  const trail = [];
  // Gear orange through WebAssembly purple and round again, one per particle.
  const COLORS = Array.from({ length: PARTICLES }, (_, i) => {
    const mix = i / PARTICLES;
    return `hsl(${26 + mix * 240}, 90%, ${58 + 10 * Math.sin(mix * 6.28)}%)`;
  });
  let program = null;
  let paused = null; // null until the first build: then follow reduced motion
  let clock = 0;
  let last = 0;
  let frames = 0;

  function setPaused(value) {
    paused = value;
    pauseButton.textContent = value ? "Play" : "Pause";
    pauseButton.setAttribute("aria-pressed", String(value));
  }
  pauseButton.addEventListener("click", () => setPaused(!paused));

  function resize() {
    const ratio = window.devicePixelRatio || 1;
    const { width, height } = canvas.getBoundingClientRect();
    canvas.width = Math.max(1, Math.round(width * ratio));
    canvas.height = Math.max(1, Math.round(height * ratio));
  }
  new ResizeObserver(resize).observe(canvas);

  function draw(now) {
    requestAnimationFrame(draw);
    const dt = last ? Math.min(0.05, (now - last) / 1000) : 0;
    last = now;
    if (!program || paused) return;
    clock += dt;
    frames += 1;
    if (frames % 10 === 0) stat("frames").textContent = frames.toLocaleString();

    let positions;
    try {
      const pointer = program.frame(PARTICLES, clock);
      positions = new Float32Array(program.memory.buffer, pointer, 2 * PARTICLES);
    } catch (error) {
      program = null;
      showOutput(`The program trapped: ${error.message}`, "error");
      setStatus("trapped", "error");
      return;
    }

    // The trails are the last few frames' positions, kept across swaps like
    // the clock. Redrawing them from a cleared canvas leaves no residue.
    trail.push(positions.slice());
    if (trail.length > TRAIL) trail.shift();
    const { width, height } = canvas;
    const scale = Math.min(width, height) * 0.46;
    const dot = Math.max(2, scale / 110);
    context.clearRect(0, 0, width, height);
    context.globalCompositeOperation = "lighter"; // overlapping particles glow
    trail.forEach((frame, age) => {
      context.globalAlpha = ((age + 1) / trail.length) ** 2;
      for (let i = 0; i < PARTICLES; i += 1) {
        const x = width / 2 + frame[2 * i] * scale;
        const y = height / 2 - frame[2 * i + 1] * scale;
        if (!Number.isFinite(x) || !Number.isFinite(y)) continue;
        context.fillStyle = COLORS[i];
        context.fillRect(x - dot / 2, y - dot / 2, dot, dot);
      }
    });
    context.globalAlpha = 1;
    context.globalCompositeOperation = "source-over";
  }
  requestAnimationFrame(draw);
}

// --- start -------------------------------------------------------------------
// Last, so every constant above is initialised before the page uses it.

const replRoot = document.getElementById("try-it");
const supported =
  typeof Worker === "function" &&
  typeof DecompressionStream === "function" &&
  typeof WebAssembly === "object" &&
  typeof WebAssembly.compileStreaming === "function";

if (replRoot && supported) {
  start(replRoot);
} else if (replRoot) {
  const note = document.createElement("p");
  note.className = "repl-noscript";
  note.textContent =
    "This browser lacks something the live compiler needs (module workers, DecompressionStream or streaming WebAssembly compilation).";
  replRoot.append(note);
}

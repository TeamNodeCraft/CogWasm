// Starting programs for the REPL. Each defines `particle(i, n, t) -> (x, y)`:
// particle i of n at time t (seconds), in roughly -1..1. The page appends
// `frame` and the maths imports (repl-core.js HARNESS). CI compiles every one
// of these (site/repl/tests/smoke.mjs).

export const EXAMPLES = [
  {
    name: "Orbit",
    source: `// Called for every particle, every frame.
// Edit it: this page recompiles it and swaps it in.
fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    let a = TAU * i / n + t * 0.4;
    let r = 0.75 + 0.12 * sin(5.0 * a + t * 2.0);
    (r * cos(a), r * sin(a))
}
`,
  },
  {
    name: "Lissajous",
    source: `// Two sine waves at different speeds trace a Lissajous figure.
fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    let s = TAU * i / n;
    (0.85 * sin(3.0 * s + t), 0.85 * sin(4.0 * s + t * 0.7))
}
`,
  },
  {
    name: "Spiral",
    source: `// A golden-angle spiral that breathes.
fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    let golden = PI * (3.0 - sqrt(5.0));
    let r = sqrt(i / n) * (0.85 + 0.1 * sin(t));
    let a = i * golden + t * 0.3;
    (r * cos(a), r * sin(a))
}
`,
  },
  {
    name: "Wave",
    source: `// A grid of particles riding a travelling wave.
fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    let side = sqrt(n);
    let col = i % side;
    let row = (i - col) / side;
    let x = col / side * 1.8 - 0.9;
    let y = row / side * 1.8 - 0.9;
    (x, y + 0.08 * sin(x * 6.0 + t * 3.0))
}
`,
  },
];

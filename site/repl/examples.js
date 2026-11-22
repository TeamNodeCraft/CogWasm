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
    source: `// A golden-angle spiral whose rings slowly twist.
fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    let golden = PI * (3.0 - sqrt(5.0));
    let f = i / n;

    // Ripples travel outward through the rings.
    let r = 0.86 * sqrt(f) + 0.025 * sin(30.0 * sqrt(f) - t * 1.2);

    // Inner rings turn against the outer ones, so the
    // spiral arms slowly wind up and unwind again.
    let twist = 1.4 * sin(t * 0.35) * (1.0 - f);
    let a = i * golden + t * 0.12 + twist;

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
  {
    name: "Wobbly",
    source: `fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    let side = sqrt(n);
    let col = i % side;
    let row = (i - col) / side;

    let x = col / side * 1.8 - 0.9;
    let y = row / side * 1.8 - 0.9;
    let r = sqrt(x * x + y * y);

    // Strong near the centre, fading toward the edges.
    let falloff = 1.0 / (1.0 + r * 3.0);
    let twist = 1.8 * falloff * sin(t * 1.4 - r * 8.0);

    let sx = x * cos(twist) - y * sin(twist);
    let sy = x * sin(twist) + y * cos(twist);

    // Two travelling waves interfere across the twisted field.
    let wave_a = sin(sx * 8.0 + sy * 3.0 - t * 2.5);
    let wave_b = sin(sy * 10.0 - sx * 4.0 + t * 1.8);
    let rings = sin(r * 24.0 - t * 4.0) * falloff;

    let out_x = sx + 0.035 * wave_b;
    let out_y = sy + 0.035 * wave_a + 0.035 * rings;

    (out_x, out_y)
}
`,
  },
];

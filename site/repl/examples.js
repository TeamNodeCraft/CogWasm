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
    name: "Logo",
    source: `// The CogWasm logo, traced in particles: the geometry of
// artwork/cog-logo/make_cog_logo.py. The cog turns once, then
// rests with its spoke back in the WebAssembly tag.
const TEETH: f32 = 10.0;
const R_TIP: f32 = 1.0;
const R_ROOT: f32 = 0.85;
const TOOTH_TIP: f32 = 0.44;   // tooth width at the tip, share of the pitch
const TOOTH_ROOT: f32 = 0.60;  // tooth width at the root
const R_HOLE: f32 = 0.69;
const HALF: f32 = 0.39;        // half size of the wasm square
const TAG_R: f32 = 0.14;       // the tag cut into its top edge
const SPOKE: f32 = 0.105;      // half width of the spoke (TAG_R - gap)

fn particle(i: f32, n: f32, t: f32) -> (f32, f32) {
    // One eased turn every 5 s, then a pause with the logo whole.
    let turn = floor(t / 5.0) + smooth(fract(t / 5.0) / 0.7);
    let spin = -turn * TAU;
    let s = i / n;
    if s < 0.5 {
        rotate(rim(s / 0.5), spin)
    } else if s < 0.68 {
        rotate(hole((s - 0.5) / 0.18), spin)
    } else if s < 0.76 {
        rotate(spoke((s - 0.68) / 0.08), spin)
    } else {
        mark((s - 0.76) / 0.24)
    }
}

fn smooth(x: f32) -> f32 {
    let x = clamp(x, 0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

fn rotate((x, y): (f32, f32), a: f32) -> (f32, f32) {
    (x * cos(a) - y * sin(a), x * sin(a) + y * cos(a))
}

fn polar(r: f32, a: f32) -> (f32, f32) {
    (r * cos(a), r * sin(a))
}

// The toothed rim, walked by length: flank up, tip, flank down, root.
fn rim(u: f32) -> (f32, f32) {
    let pitch = TAU / TEETH;
    let tip = TOOTH_TIP * pitch / 2.0;
    let root = TOOTH_ROOT * pitch / 2.0;
    let c = floor(u * TEETH) * pitch; // tooth centres, as the generator puts them
    let flank = hypot(R_TIP - R_ROOT, (root - tip) * R_TIP);
    let top = 2.0 * tip * R_TIP;
    let gap = (pitch - 2.0 * root) * R_ROOT;
    let mut d = fract(u * TEETH) * (2.0 * flank + top + gap);
    if d < flank {
        let f = d / flank;
        return polar(lerp(R_ROOT, R_TIP, f), c - root + f * (root - tip));
    }
    d -= flank;
    if d < top {
        return polar(R_TIP, c - tip + d / R_TIP);
    }
    d -= top;
    if d < flank {
        let f = d / flank;
        return polar(lerp(R_TIP, R_ROOT, f), c + tip + f * (root - tip));
    }
    polar(R_ROOT, c + root + (d - flank) / R_ROOT)
}

// The edge of the hole, open where the spoke leaves it (at the top).
fn hole(u: f32) -> (f32, f32) {
    let side = asin(SPOKE / R_HOLE);
    polar(R_HOLE, PI / 2.0 + side + u * (TAU - 2.0 * side))
}

// The spoke: down one side, round its tip, back up the other.
fn spoke(u: f32) -> (f32, f32) {
    let start = sqrt(R_HOLE * R_HOLE - SPOKE * SPOKE);
    let side = start - HALF;
    let arc = PI * SPOKE;
    let d = u * (2.0 * side + arc);
    if d < side {
        (SPOKE, start - d)
    } else if d < side + arc {
        let a = -(d - side) / SPOKE;
        (SPOKE * cos(a), HALF + SPOKE * sin(a))
    } else {
        (-SPOKE, HALF + (d - side - arc))
    }
}

// The WebAssembly mark, which stays put: a square with a
// semicircle tag bitten out of the middle of its top edge.
fn mark(u: f32) -> (f32, f32) {
    let edge = 2.0 * HALF;
    let part = HALF - TAG_R;
    let bite = PI * TAG_R;
    let mut d = u * (3.0 * edge + 2.0 * part + bite);
    if d < edge {
        return (-HALF, HALF - d);
    }
    d -= edge;
    if d < edge {
        return (-HALF + d, -HALF);
    }
    d -= edge;
    if d < edge {
        return (HALF, -HALF + d);
    }
    d -= edge;
    if d < part {
        return (HALF - d, HALF);
    }
    d -= part;
    if d < bite {
        let a = -d / TAG_R;
        return (TAG_R * cos(a), HALF + TAG_R * sin(a));
    }
    (-TAG_R - (d - bite), HALF)
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

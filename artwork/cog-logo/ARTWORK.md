# CogWasm artwork

<p align="center">
  <img src="logo-colour-dark.svg" width="180" alt="CogWasm app icon">
  &nbsp;
  <img src="logo-colour.svg" width="180" alt="CogWasm logo in colour">
</p>

The CogWasm mark is an orange gear holding the WebAssembly tag square: the
compiler (a machine, orange as Rust is often drawn) reaching into WebAssembly
(the purple square with its tag) through a single spoke. It says what the
project does: Rust's compiler, running inside and producing WebAssembly.

## Files

| File | Use |
| --- | --- |
| [`logo-colour-dark.svg`](logo-colour-dark.svg) | App icon: the mark on a dark squircle with a soft orange and purple glow |
| [`logo-colour.svg`](logo-colour.svg) | Full-colour mark on a transparent background |
| [`logo-silhouette-black.svg`](logo-silhouette-black.svg) | One-colour mark for light backgrounds |
| [`logo-silhouette-white.svg`](logo-silhouette-white.svg) | One-colour mark for dark backgrounds |
| [`make_cog_logo.py`](make_cog_logo.py) | The generator that draws all four |

All four are 512 × 512 SVGs and scale to any size.

## Design

- **Gear.** Ten teeth on a ring, softened corners, one spoke reaching inward.
  It stands for the compiler: Rust's compiler, as a working machine.
- **WebAssembly tag square.** A plain purple square with the semicircle tag
  cut into its top edge, after the WebAssembly logo, without its lettering.
- **One spoke into the tag.** The gear's only spoke drops into the tag with a
  small gap all round: the compiler connected to WebAssembly at one point,
  not wrapped around it.
- **Real 3D, drawn as flat SVG.** The shapes are built as geometry, extruded
  into a slab, turned and tilted, and projected in perspective. Every side wall
  that faces the viewer is shaded by its own angle to a light from the upper
  left; walls facing away are dropped, and the rest are drawn back to front.
  Top faces get a diagonal gradient, a soft sheen and a light bevel rim.
- **Upright tag.** The generator solves the in-plane turn that keeps the spoke
  and tag pointing straight up on screen, so the square reads upright like the
  WebAssembly mark at any tilt.
- **Silhouettes in two tones.** Top faces are solid ink; the extruded walls are
  the same ink at 40 %, so the depth still reads in one colour. A thin channel
  is cut around the gear so the spoke stays separate from the square.
- **Icon frame.** The app icon uses a superellipse squircle
  (|x|⁵ + |y|⁵ = 1) rather than a rounded rectangle, and the mark is scaled to
  fill it with an even margin clear of the curved corners.

### Colours

| Part | Top face | Lit wall | Shadowed wall |
| --- | --- | --- | --- |
| Gear | `#ffc36e` → `#f27412` | `#f08a2a` | `#6e2702` |
| Square | `#bc9dff` → `#6c2fec` | `#8a5cf6` | `#24096a` |
| Icon background | `#1d1a24` → `#0c0b10` | | |

## Regenerating

The SVGs are generated, not hand-edited. Change the parameters at the top of
`make_cog_logo.py` and run:

```bash
python3 make_cog_logo.py
```

### Optional WA lettering

```bash
python3 make_cog_logo.py --wa-inset
```

engraves the WebAssembly "WA" lettering into the square and writes the same
four files as `logo-*-wa-inset.svg`, leaving the plain marks alone. The
letters are the WebAssembly icon's own outlines (copied as path data from
`dist/icon/web-assembly-icon.svg`, CC0), at the same size and place on the
square. In colour they are cut `INSET` deep, with a floor and lit inner
walls; in the silhouettes they are punched out of the top face and filled at
the walls' 40 %. The README header uses the engraved app icon,
`logo-colour-dark-wa-inset.svg`.

Useful parameters: `TEETH`, `R_HOLE` (ring thickness), `PIECE_HALF` (square
size), `TAG_R` (tag size), `GAP` (clearance around the spoke), `DEPTH`, `YAW`,
`PITCH`, `ROLL`, `CAMERA` (perspective), `LIGHT`, `SQUIRCLE_N` (icon corner
shape) and the colours in `PAL`. The mark is fitted to each frame
automatically (`fit_view` margins in `main()`). The script needs only the
Python standard library.

## Copyright and permitted use

**Copyright © 2026 AlexMitchellMus. All rights reserved.**

The CogWasm logo, icon and artwork in this folder (the SVG files,
`make_cog_logo.py` and this document) belong to the CogWasm project. They are
**not** covered by this repository's MIT or Apache-2.0 licence, and no licence
to copy, modify or reuse them is granted, except as follows.

**You may**, without asking:

- show the unmodified logo or icon to refer to or link to the CogWasm
  project, for example in an article, a talk, a package listing or a link to
  this repository.

**You may not**, without written permission:

- use the logo, the icon or anything confusingly similar for another project,
  product, service or organisation;
- modify the artwork, or use `make_cog_logo.py` to make derived marks;
- use the artwork in a way that suggests the CogWasm project endorses you.

To ask for permission, open an issue on this repository.

### Attribution

The purple tag square follows the
[WebAssembly logo](https://github.com/carlosbaraza/web-assembly-logo) by
Carlos Baraza, chosen as the official logo in
[WebAssembly/design#980](https://github.com/WebAssembly/design/issues/980)
and offered by its designer under CC0. That shape stays free to use; the
copyright above covers this project's own work: the gear, the spoke, the
composition, the 3D rendering, the icon and the generator. The gear is an
original drawing; it is not the Rust logo.

"Rust", the Rust logo, "WebAssembly" and the WebAssembly logo belong to their
owners. This artwork does not imply endorsement by the Rust Project or the
WebAssembly community.

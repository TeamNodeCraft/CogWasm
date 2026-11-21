# Stage B / A.2 results: two realistic plugins, LLVM vs cg_clif

Date: 2026-11-18T07:55:16Z. Host: macOS-27.0.1-arm64-arm-64bit-Mach-O (arm64, 14 CPUs). Rust: `rustc 1.101.0-nightly (b6eeb93c9 2026-10-18)`. All artifacts were validated with `wasmparser 0.263` (all features) and executed by Wasmtime 50.0.2.

## Verdict

Both plugins **build, link, validate and run** under LLVM O2 and cg_clif O2. The requested MIR-inlining config (`-Zinline-mir -Zmir-opt-level=3`) does **not** build: compiler_builtins hits the f128 libcall-signature conflict already seen for Stage A's O0 sysroot. `-Zmir-opt-level=4` *does* build, so the MIR-inlining effect is measured at level 4 instead; its effect on these two programs is small. Details below.

## Configs

| config | backend | linker | std | RUSTFLAGS |
|---|---|---|---|---|
| llvm_o2 | llvm | rust-lld (default wasm-ld) | rustup prebuilt | `-Cpanic=abort` |
| clif_o2 | clif | Wild 0.11.0 | cg_clif -Zbuild-std | `-Zcodegen-backend=$STAGE_A/toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib -Cpanic=abort -Clinker=$STAGE_A/toolchain/target/wild/release/wild -Clinker-flavor=wasm-ld` |
| clif_o2_mir3 | clif | Wild 0.11.0 | cg_clif -Zbuild-std | `-Zcodegen-backend=$STAGE_A/toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib -Cpanic=abort -Zinline-mir -Zmir-opt-level=3 -Clinker=$STAGE_A/toolchain/target/wild/release/wild -Clinker-flavor=wasm-ld` |
| clif_o2_inline_only | clif | Wild 0.11.0 | cg_clif -Zbuild-std | `-Zcodegen-backend=$STAGE_A/toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib -Cpanic=abort -Zinline-mir -Clinker=$STAGE_A/toolchain/target/wild/release/wild -Clinker-flavor=wasm-ld` |
| clif_o2_mir4 | clif | Wild 0.11.0 | cg_clif -Zbuild-std | `-Zcodegen-backend=$STAGE_A/toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib -Cpanic=abort -Zinline-mir -Zmir-opt-level=4 -Clinker=$STAGE_A/toolchain/target/wild/release/wild -Clinker-flavor=wasm-ld` |
| clif_o2_noinline | clif | Wild 0.11.0 | cg_clif -Zbuild-std | `-Zcodegen-backend=$STAGE_A/toolchain/target/cg_clif/release/librustc_codegen_cranelift.dylib -Cpanic=abort -Zinline-mir=no -Zmir-opt-level=2 -Clinker=$STAGE_A/toolchain/target/wild/release/wild -Clinker-flavor=wasm-ld` |

## Build (cold = fresh target dir, warm = 5 × touch+build)

cg_clif builds `core`/`alloc`/`compiler_builtins` with `-Zbuild-std` in the fresh target dir, so its cold column includes the sysroot (~6.5–7 s); LLVM uses rustup's prebuilt std and its cold column is only the plugin. Warm is the realistic plugin rebuild. cg_clif links with Wild (which materialises the 1 MiB `.bss` heap as a data segment); LLVM links with the default rust-lld. `code B` is the sum of wasm function bodies, so it is the fair code-size comparison; `size B` includes Wild's 1 MiB data segment.

| config | program | cold s | warm s median [min–max] | size B | code B | data B | valid |
|---|---|---:|---:|---:|---:|---:|---|
| llvm_o2 | e_math | 3.358 | 0.100 [0.094–0.107] | 16773 | 13173 | 1156 | yes |
| llvm_o2 | f_geometry | 0.179 | 0.090 [0.089–0.100] | 7815 | 4638 | 680 | yes |
| clif_o2 | e_math | 7.038 | 0.131 [0.128–0.138] | 1110860 | 39709 | 1054877 | yes |
| clif_o2 | f_geometry | 6.645 | 0.129 [0.127–0.132] | 1082687 | 21679 | 1051446 | yes |
| clif_o2_inline_only | e_math | 6.784 | 0.134 [0.133–0.140] | 1110860 | 39709 | 1054877 | yes |
| clif_o2_inline_only | f_geometry | 6.558 | 0.126 [0.125–0.166] | 1082687 | 21679 | 1051446 | yes |
| clif_o2_mir4 | e_math | 6.937 | 0.136 [0.128–0.138] | 1110593 | 39630 | 1054763 | yes |
| clif_o2_mir4 | f_geometry | 6.814 | 0.133 [0.131–0.142] | 1082176 | 21472 | 1051332 | yes |

## Wasmtime load (median of 5, ms)

Fresh Engine per iteration; reused Engine = one Engine, new `Module` each iteration. `inst` is instantiate from the already-compiled module.

| config | program | fresh compile | fresh inst | fresh total | reused compile | reused inst | reused total |
|---|---|---:|---:|---:|---:|---:|---:|
| llvm_o2 | e_math | 6.771 [6.426–20.072] | 0.045 [0.041–0.079] | 6.813 [6.467–20.152] | 6.785 [6.496–6.867] | 0.049 [0.036–0.123] | 6.877 [6.532–6.981] |
| llvm_o2 | f_geometry | 4.078 [3.545–4.969] | 0.033 [0.023–0.060] | 4.111 [3.569–5.029] | 3.587 [3.348–3.848] | 0.027 [0.019–0.031] | 3.617 [3.376–3.873] |
| clif_o2 | e_math | 17.181 [16.735–17.846] | 0.117 [0.092–0.121] | 17.301 [16.827–17.966] | 16.316 [16.284–16.460] | 0.102 [0.096–0.105] | 16.419 [16.389–16.556] |
| clif_o2 | f_geometry | 10.145 [9.587–10.898] | 0.107 [0.089–0.150] | 10.252 [9.676–11.047] | 10.152 [9.702–10.502] | 0.103 [0.101–0.145] | 10.254 [9.806–10.647] |
| clif_o2_inline_only | e_math | 17.376 [17.008–18.436] | 0.106 [0.105–0.234] | 17.482 [17.135–18.670] | 16.695 [16.019–18.244] | 0.104 [0.091–0.114] | 16.804 [16.121–18.348] |
| clif_o2_inline_only | f_geometry | 9.560 [9.121–10.318] | 0.101 [0.093–0.131] | 9.669 [9.214–10.449] | 9.077 [8.899–10.308] | 0.089 [0.084–0.102] | 9.162 [8.988–10.409] |
| clif_o2_mir4 | e_math | 16.716 [16.481–18.156] | 0.103 [0.098–0.129] | 16.815 [16.584–18.285] | 16.671 [16.518–16.768] | 0.102 [0.096–0.115] | 16.767 [16.617–16.874] |
| clif_o2_mir4 | f_geometry | 9.433 [9.023–11.354] | 0.101 [0.090–0.135] | 9.544 [9.122–11.489] | 9.363 [9.320–10.114] | 0.097 [0.087–0.105] | 9.457 [9.417–10.219] |

## Run time (median of 5 warm calls)

(e) 100,000 f64 points: `y = sin(x)*0.5 + cos(x*0.25) + sqrt(|x|)`, reduce to mean. (f) 40,000 loop iterations × 5 imports = **200,000 host calls**.

| config | program | total ms median [min–max] | ns/point or ns/host-call | result check |
|---|---|---:|---:|---|
| llvm_o2 | e_math | 2.300 [2.266–2.378] | 22.996 ns/point | ok |
| llvm_o2 | f_geometry | 0.622 [0.608–0.630] | 3.112 ns/host-call | ok |
| clif_o2 | e_math | 8.779 [8.707–9.061] | 87.786 ns/point | ok |
| clif_o2 | f_geometry | 0.570 [0.548–0.593] | 2.851 ns/host-call | ok |
| clif_o2_inline_only | e_math | 8.546 [8.526–8.588] | 85.456 ns/point | ok |
| clif_o2_inline_only | f_geometry | 0.544 [0.541–0.595] | 2.719 ns/host-call | ok |
| clif_o2_mir4 | e_math | 8.618 [8.611–8.756] | 86.175 ns/point | ok |
| clif_o2_mir4 | f_geometry | 0.572 [0.537–0.707] | 2.860 ns/host-call | ok |

## cg_clif vs LLVM O2 ratios

`run` ratio is ns/point for (e) and ns/host-call for (f). Both size ratios are cg_clif/LLVM; `total size` is dominated by Wild's 1 MiB `.bss` data segment, so `code size` is the meaningful codegen comparison.

| program | cold | warm | module load | instantiate | run | total size | code size |
|---|---:|---:|---:|---:|---:|---:|---:|
| e_math | 2.10× | 1.30× | 2.54× | 2.62× | 3.82× | 66.23× | 3.01× |
| f_geometry | 37.09× | 1.44× | 2.49× | 3.22× | 0.92× | 138.54× | 4.67× |

## Interpretation

* **Compile+link.** Warm cg_clif is 1.30× LLVM for (e) and 1.44× for (f). Cold is not a fair backend comparison: cg_clif's fresh dir builds core/alloc/compiler_builtins (7.04 s / 6.65 s), while LLVM uses rustup's prebuilt std (3.36 s / 0.18 s, single runs and therefore noisy). The warm medians are the meaningful plugin-rebuild numbers and are well under the 0.41 s target.

* **Load.** cg_clif modules take 2.54× / 2.49× longer to compile in Wasmtime (17.2 vs 6.8 ms for (e), 10.1 vs 4.1 ms for (f)), consistent with the larger code sections. Instantiation itself is ~0.1 ms for both, so `Module::new` dominates load time.

* **Run.** The maths node is 3.82× slower under cg_clif (87.8 vs 23.0 ns/point; 8.78 vs 2.30 ms). The host-call geometry is 0.92× (cg_clif 2.85 vs LLVM 3.11 ns per imported call): the 200,000 Wasmtime host calls dominate, so backend codegen barely matters.

* **Size.** cg_clif code is 3.01× / 4.67× larger. Total artifact size cannot be compared directly because Wild writes the 1 MiB `.bss` heap as a data segment (+~1.05 MB); rust-lld represents the same zeroed heap in the memory minimum.

* **MIR inlining.** The requested `-Zinline-mir -Zmir-opt-level=3` fails to build. `-Zmir-opt-level=4` succeeds but reduces code by only 79 B (maths) / 207 B (geometry) and changes run time by <2%; `-Zinline-mir` alone at MIR opt level 2 is byte-identical to the default. There is no meaningful MIR-inlining win for these programs on cg_clif.

* **Strictness.** Every cg_clif artifact passes `CG_CLIF_WASM_STRICT=1` with zero trap-stub warnings, including core/alloc/libm.

## MIR inlining

`clif_o2` is the default (MIR inlining off). `inline_only` adds `-Zinline-mir` at the default release MIR opt level (2). `mir4` adds `-Zinline-mir -Zmir-opt-level=4`. The explicit `-Zinline-mir=no` control fails in compiler_builtins (see failures), so the default `clif_o2` (no flag) is the meaningful MIR-inlining-disabled baseline. `inline_only` artifacts are byte-identical to `clif_o2` (same SHA-256), so its ±3–5% ratios are run-to-run noise. Ratios are variant/clif_o2; <1 means the variant is faster/smaller.

| variant | program | warm ratio | run ratio | module-load ratio | code B delta | size B delta |
|---|---|---:|---:|---:|---:|---:|
| clif_o2_inline_only | e_math | 1.03× | 0.97× | 1.01× | +0 | +0 |
| clif_o2_inline_only | f_geometry | 0.97× | 0.95× | 0.94× | +0 | +0 |
| clif_o2_mir4 | e_math | 1.04× | 0.98× | 0.97× | -79 | -267 |
| clif_o2_mir4 | f_geometry | 1.03× | 1.00× | 0.93× | -207 | -511 |

## Strict-check and failures

| config | program | strict build | warnings |
|---|---|---|---:|
| clif_o2 | e_math | ok | 0 |
| clif_o2 | f_geometry | ok | 0 |
| clif_o2_inline_only | e_math | ok | 0 |
| clif_o2_inline_only | f_geometry | ok | 0 |
| clif_o2_mir4 | e_math | ok | 0 |
| clif_o2_mir4 | f_geometry | ok | 0 |

Failed configs / builds. `clif_o2_mir3` is the requested level-3 MIR-inlining config; `clif_o2_noinline` is an explicit disabled control. Both fail while building compiler_builtins on libcall ABI signatures (f128 and i128 respectively). The default `clif_o2` (no inline flag) builds and is used as the disabled baseline.

### `clif_o2_mir3` (cold build, e_math)

```text
Compiling compiler_builtins v0.1.160 (/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/rustup/toolchains/nightly-2026-10-19-aarch64-apple-darwin/lib/rustlib/src/rust/library/compiler-builtins/compiler-builtins)
   Compiling core v0.0.0 (/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/rustup/toolchains/nightly-2026-10-19-aarch64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling libm v0.2.16
   Compiling alloc v0.0.0 (/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/rustup/toolchains/nightly-2026-10-19-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc)
error: attempt to declare `__floatsitf` with signature Signature { params: [AbiParam { value_type: types::I32, purpose: Normal, extension: None }], returns: [AbiParam { value_type: types::F128, purpose: Normal, extension: None }], call_conv: SystemV }, but it was already declared with signature Signature { params: [AbiParam { value_type: types::I32, purpose: StructReturn, extension: None }, AbiParam { value_type: types::I32, purpose: Normal, extension: None }], returns: [], call_conv: SystemV }

error: could not compile `compiler_builtins` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

### `clif_o2_noinline` (cold build, e_math)

```text
Compiling compiler_builtins v0.1.160 (/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/rustup/toolchains/nightly-2026-10-19-aarch64-apple-darwin/lib/rustlib/src/rust/library/compiler-builtins/compiler-builtins)
   Compiling core v0.0.0 (/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/rustup/toolchains/nightly-2026-10-19-aarch64-apple-darwin/lib/rustlib/src/rust/library/core)
   Compiling libm v0.2.16
   Compiling alloc v0.0.0 (/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/rustup/toolchains/nightly-2026-10-19-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc)
error: attempt to declare `__umodti3` with signature Signature { params: [AbiParam { value_type: types::I128, purpose: Normal, extension: None }, AbiParam { value_type: types::I128, purpose: Normal, extension: None }], returns: [AbiParam { value_type: types::I128, purpose: Normal, extension: None }], call_conv: SystemV }, but it was already declared with signature Signature { params: [AbiParam { value_type: types::I32, purpose: StructReturn, extension: None }, AbiParam { value_type: types::I128, purpose: Normal, extension: None }, AbiParam { value_type: types::I128, purpose: Normal, extension: None }], returns: [], call_conv: SystemV }

error: could not compile `compiler_builtins` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

## Reproduce

```sh
stage-b/a2/run.sh
```

Raw runs and medians: `/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/stage-b/results/a2.json`. Harness: `/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/target/a2/harness/release/a2-harness` (Wasmtime 50 + wasmparser 0.263). All build outputs are under `/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/experiments/cranelift-wasm/toolchain/target/a2` (gitignored); no project target dirs are written.

## Protocol and caveats

* Cold: one build in a newly-created target dir. For cg_clif configs (`-Zbuild-std`) this also builds core/alloc/compiler_builtins/libm; for LLVM it is plugin-only against rustup's prebuilt std.
* Warm: prewarmed target dir, `touch src/lib.rs`, 5 builds; median [min–max]. Each timed run recompiled the plugin crate.
* Load: 5 iterations per mode. Fresh Engine per iteration; reused mode keeps one Engine and creates a new Module each iteration. `Module::new` compiles; instantiate is timed separately.
* Run: one warm-up call, then 5 timed calls. (e) n=100,000 f64 points. (f) n=40,000 iterations × 5 imports = 200,000 host calls; the host implements all five `nodecraft` imports with `Linker::func_wrap`.
* Linkers: cg_clif uses Wild 0.11.0 (`-Clinker=... -Clinker-flavor=wasm-ld`); LLVM uses rust-lld (rustc's default wasm linker). Within each family the linker is constant.
* Wild materialises the 1 MiB `.bss` heap as a data segment (~1.05 MB); the `code B` column and the code-size ratio avoid that confound.
* `CG_CLIF_WASM_STRICT=1` builds are separate clean builds per cg_clif config; warnings and errors are captured.
* Every artifact validated with `wasmparser::WasmFeatures::all()` and produced the same mean/result as the host reference to within 1e-9 relative.
* The host was shared with other stage-B build/test jobs during measurement; medians are reported, but single-run cold numbers and occasional load outliers (e.g. the first fresh-engine compile) are sensitive to machine load.


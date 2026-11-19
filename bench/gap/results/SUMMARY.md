# Gap attribution summary (darwin-arm64-local)

History: `/Users/alexandermitchell/Documents/github/rust-wasm-compiler/rust_wasm/bench/history/2026-11-19-darwin-arm64-92f976a.json` — commit `92f976a9bfdb`

| program | exec ours/O2 | instr ours | instr O2 | instr ratio | top cause (extra) |
| --- | ---: | ---: | ---: | ---: | --- |
| a_add | 0.52× | 1.2k | 264 | 4.61× | Software operations +388 |
| b_alloc | 2.42× | 1.64M | 347.9k | 4.71× | Values in memory / register moves +1.07M |
| c_plugin | 0.61× | 4.8k | 336 | 14.40× | Values in memory / register moves +2.5k |
| smoke | 3.82× | 928.4k | 80.5k | 11.54× | Software operations +512.9k |
| int_loops | 1.03× | 247.8k | 123.3k | 2.01× | Other +57.8k |
| float_sum | 1.32× | 251.6k | 71.1k | 3.54× | Values in memory / register moves +129.6k |
| expr | 4.25× | 2.97M | 669.0k | 4.44× | Values in memory / register moves +1.31M |
| json | 3.65× | 2.13M | 483.3k | 4.40× | Values in memory / register moves +892.5k |
| collections | 9.16× | 8.25M | 1.06M | 7.78× | Software operations +3.19M |
| scene | 1.67× | 443.5k | 113.8k | 3.90× | Values in memory / register moves +197.3k |
| nbody | 5.07× | 6.20G | 1.80G | 3.45× | Values in memory / register moves +3.55G |
| mandelbrot | 1.06× | 611.31M | 424.73M | 1.44× | Other +112.71M |
| sha256 | 12.33× | 363.02M | 42.34M | 8.57× | Software operations +290.48M |
| sort | 12.22× | 1.26G | 150.66M | 8.39× | Software operations +936.14M |
| matmul | 1.56× | 380.36M | 346.91M | 1.10× | Values in memory / register moves +31.30M |
| strings | 9.20× | 120.21M | 22.17M | 5.42× | Values in memory / register moves +49.68M |
| math_e | 2.71× | 774.68M | 350.64M | 2.21× | Software operations +400.47M |
| hostcall_f | 1.00× | 10.88M | 7.04M | 1.55× | Values in memory / register moves +1.76M |
| libm_driver | 3.25× | 302.70M | 123.01M | 2.46× | Software operations +170.36M |
| miniz_driver | 2.91× | 220.05M | 94.17M | 2.34× | Values in memory / register moves +72.26M |
| generated_stress | 1.10× | 691.4k | 351.1k | 1.97× | Other +150.2k |

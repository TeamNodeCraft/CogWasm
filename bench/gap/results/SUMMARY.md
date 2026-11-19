# Gap attribution summary (darwin-arm64-local)

History: `bench/history/2026-11-19-darwin-arm64-b3d59d0.json` — commit `b3d59d0c697a`

| program | exec ours/O2 | instr ours | instr O2 | instr ratio | top cause (extra) |
| --- | ---: | ---: | ---: | ---: | --- |
| a_add | 0.49× | 804 | 264 | 3.05× | Values in memory / register moves +372 |
| b_alloc | 2.44× | 1.63M | 347.9k | 4.70× | Values in memory / register moves +1.07M |
| c_plugin | 0.66× | 4.1k | 336 | 12.27× | Values in memory / register moves +2.5k |
| smoke | 2.10× | 406.0k | 80.5k | 5.04× | Values in memory / register moves +198.8k |
| int_loops | 0.98× | 213.9k | 123.3k | 1.73× | Other +57.0k |
| float_sum | 1.80× | 243.2k | 71.1k | 3.42× | Values in memory / register moves +129.6k |
| expr | 4.09× | 2.70M | 669.0k | 4.04× | Values in memory / register moves +1.31M |
| json | 3.17× | 1.87M | 483.3k | 3.87× | Values in memory / register moves +893.6k |
| collections | 6.06× | 5.06M | 1.06M | 4.77× | Values in memory / register moves +2.39M |
| scene | 1.81× | 415.7k | 113.8k | 3.65× | Values in memory / register moves +197.3k |
| nbody | 5.10× | 6.20G | 1.80G | 3.45× | Values in memory / register moves +3.55G |
| mandelbrot | 1.08× | 611.29M | 424.73M | 1.44× | Other +112.71M |
| sha256 | 1.50× | 70.42M | 42.34M | 1.66× | Values in memory / register moves +17.30M |
| sort | 3.51× | 321.21M | 150.66M | 2.13× | Values in memory / register moves +106.84M |
| matmul | 1.57× | 380.35M | 346.91M | 1.10× | Values in memory / register moves +31.30M |
| strings | 8.50× | 107.24M | 22.17M | 4.84× | Values in memory / register moves +49.65M |
| math_e | 2.73× | 774.67M | 350.64M | 2.21× | Software operations +400.47M |
| hostcall_f | 0.98× | 10.88M | 7.04M | 1.55× | Values in memory / register moves +1.76M |
| libm_driver | 3.20× | 302.69M | 123.01M | 2.46× | Software operations +170.36M |
| miniz_driver | 2.82× | 216.10M | 94.17M | 2.29× | Values in memory / register moves +72.26M |
| generated_stress | 1.09× | 671.5k | 351.1k | 1.91× | Other +141.7k |

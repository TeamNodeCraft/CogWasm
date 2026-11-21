| config | program | cold s | warm s | bytes | validation |
|---|---|---:|---:|---:|---|
| clif-lld-o2 | a_add | 6.366 | 0.127 | 236 | valid: 236 bytes |
| clif-lld-o2 | b_alloc | 7.056 | 0.153 | 89739 | valid: 89739 bytes |
| clif-lld-o2 | c_plugin | 6.855 | 0.145 | 34502 | valid: 34502 bytes |
| clif-lld-o2-prebuilt | a_add | 0.166 | 0.075 | 236 | valid: 236 bytes |
| clif-lld-o2-prebuilt | b_alloc | 1.070 | 0.100 | 60741 | valid: 60741 bytes |
| clif-lld-o2-prebuilt | c_plugin | 0.184 | 0.089 | 18225 | valid: 18225 bytes |
| clif-wild-o2 | a_add | 6.814 | 0.118 | 267 | valid: 267 bytes |
| clif-wild-o2 | b_alloc | 7.171 | 0.141 | 1139729 | valid: 1139729 bytes |
| clif-wild-o2 | c_plugin | 6.738 | 0.132 | 1083560 | valid: 1083560 bytes |
| llvm-lld-o2 | a_add | 0.168 | 0.074 | 306 | valid: 306 bytes |
| llvm-lld-o2 | b_alloc | 0.837 | 0.117 | 18920 | valid: 18920 bytes |
| llvm-lld-o2 | c_plugin | 0.195 | 0.100 | 7274 | valid: 7274 bytes |
| llvm-wild-o2 | a_add | 0.153 | 0.061 | 265 | valid: 265 bytes |
| llvm-wild-o2 | b_alloc | 0.816 | 0.103 | 1067615 | valid: 1067615 bytes |
| llvm-wild-o2 | c_plugin | 0.180 | 0.088 | 1055837 | valid: 1055837 bytes |
| clif-lld-o0 | a_add | 0.170 | 0.074 | 304 | valid: 304 bytes |
| clif-lld-o0 | b_alloc | 1.344 | 0.100 | 80026 | valid: 80026 bytes |
| clif-lld-o0 | c_plugin | 0.179 | 0.086 | 25299 | valid: 25299 bytes |
| llvm-lld-o0 | a_add | 0.166 | 0.073 | 307 | valid: 307 bytes |
| llvm-lld-o0 | b_alloc | 0.746 | 0.100 | 60221 | valid: 60221 bytes |
| llvm-lld-o0 | c_plugin | 0.169 | 0.091 | 16442 | valid: 16442 bytes |

| config | b_alloc ns/iter (n=2,000,000) |
|---|---:|
| clif-lld-o2 | 98.19 |
| clif-lld-o2-prebuilt | 101.01 |
| clif-wild-o2 | 98.62 |
| llvm-lld-o2 | 25.28 |
| llvm-wild-o2 | 24.93 |
| clif-lld-o0 | 238.87 |
| llvm-lld-o0 | 206.51 |

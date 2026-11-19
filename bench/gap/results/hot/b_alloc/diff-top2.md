# <b_alloc::Scale as b_alloc::Op>::apply

- ours: 56,580 executed instructions (3.5% of the program)
- LLVM -O2: 45,264 executed instructions (13.0%)

## Ours

```wat
(func $_RNvXCs6R0IlkGltaz_7b_allocNtB2_5ScaleNtB2_2Op5apply (;40;) (type 12) (param i32 f64) (result f64)
    local.get 1
    local.get 0
    f64.load align=1
    f64.mul
    return
    unreachable
  )
```

## LLVM -O2

```wat
(func $_RNvXCsf2RBcje3qbD_7b_allocNtB2_5ScaleNtB2_2Op5apply (;4;) (type 0) (param i32 f64) (result f64)
    local.get 1
    local.get 0
    f64.load
    f64.mul
  )
```

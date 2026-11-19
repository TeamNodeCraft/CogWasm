# generated_stress::combine::<generated_stress::S0>

- ours: 29,696 executed instructions (4.3% of the program)
- LLVM -O2: 19,968 executed instructions (5.7%)

## Ours

```wat
(func $_RINvCs2LeA2mZavAK_16generated_stress7combineNtB2_2S0EB2_ (;0;) (type 0) (param i32 i64) (result i64)
    (local i64 i64 i64 i64 i64 i64 i64 i64)
    local.get 0
    call $_RNvXs_Cs2LeA2mZavAK_16generated_stressNtB4_2S0NtB4_2T02v0
    local.set 2
    local.get 0
    call $_RNvXs0_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T12v1
    local.set 3
    local.get 0
    call $_RNvXs1_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T22v2
    local.set 4
    local.get 0
    call $_RNvXs2_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T32v3
    local.set 5
    local.get 0
    call $_RNvXs3_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T42v4
    local.set 6
    local.get 0
    call $_RNvXs4_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T52v5
    local.set 7
    local.get 0
    call $_RNvXs5_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T62v6
    local.set 8
    local.get 0
    call $_RNvXs6_Cs2LeA2mZavAK_16generated_stressNtB5_2S0NtB5_2T72v7
    local.set 9
    local.get 1
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 3
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 4
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 5
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 6
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 7
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 8
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 9
    i64.add
    return
    unreachable
  )
```

## LLVM -O2

```wat
(func $_RINvCsicJf4rFWGrm_16generated_stress7combineNtB2_2S0EB2_ (;0;) (type 3) (param i32 i64) (result i64)
    (local i64)
    local.get 0
    i64.load
    i64.const 3
    i64.mul
    i64.const 1
    i64.add
    local.tee 2
    local.get 1
    i64.const -7046029254386353131
    i64.mul
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
    i64.const -7046029254386353131
    i64.mul
    local.get 2
    i64.add
  )
```

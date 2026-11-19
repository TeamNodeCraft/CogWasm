# hostcall_f::body vs LLVM's inlined `run`

LLVM -O2 inlined the whole program into `run`; this is the hot function that survives as a separate body in our module.

## Ours (10,880,372 instructions, 100.0%)

```wat
(func $_RNvCskgam9jYqc1y_10hostcall_f4body (;8;) (type 3) (param i64) (result i64)
    (local i32 f64 i32 f64 i32 f64 f64 f64 i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 9
    global.set $__stack_pointer
    i32.const 1065112
    local.set 1
    local.get 0
    local.get 0
    local.set 10
    i64.const -1085102592571150095
    local.set 11
    local.get 10
    i64.const 4294967295
    i64.and
    local.get 11
    i64.const 32
    i64.shr_u
    i64.mul
    local.set 12
    local.get 10
    i64.const 32
    i64.shr_u
    local.get 11
    i64.const 4294967295
    i64.and
    i64.mul
    local.set 13
    local.get 10
    i64.const 32
    i64.shr_u
    local.get 11
    i64.const 32
    i64.shr_u
    i64.mul
    local.get 12
    i64.const 32
    i64.shr_u
    i64.add
    local.get 13
    i64.const 32
    i64.shr_u
    i64.add
    local.get 10
    i64.const 4294967295
    i64.and
    local.get 11
    i64.const 4294967295
    i64.and
    i64.mul
    i64.const 32
    i64.shr_u
    local.get 12
    i64.const 4294967295
    i64.and
    i64.add
    local.get 13
    i64.const 4294967295
    i64.and
    i64.add
    i64.const 32
    i64.shr_u
    i64.add
    i64.const 4
    i64.shr_u
    i64.const 17
    i64.mul
    i64.sub
    f64.convert_i64_u
    f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
    f64.mul
    local.set 2
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    local.set 4
    local.set 3
    loop ;; label = @1
      local.get 3
      i32.const 40000
      i32.lt_u
      if ;; label = @2
        local.get 3
        i32.const 7
        i32.and
        local.tee 5
        i32.const 16
        i32.lt_u
        if ;; label = @3
          local.get 1
          local.get 5
          i32.const 3
          i32.shl
          i32.add
          f64.load align=1
          local.set 6
          local.get 5
          i32.const 8
          i32.add
          i32.const 16
          i32.lt_u
          if ;; label = @4
            local.get 1
            local.get 5
            i32.const 8
            i32.add
            i32.const 3
            i32.shl
            i32.add
            f64.load align=1
            local.set 7
            local.get 6
            local.get 7
            f64.const 0x1p-1 (;=0.5;)
            f64.const 0x1p-2 (;=0.25;)
            call $dot2
            local.tee 8
            local.get 3
            f64.convert_i32_u
            f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
            f64.mul
            local.get 2
            f64.add
            f64.add
            local.get 6
            call $length2
            local.tee 8
            f64.const 0x1.0624dd2f1a9fcp-10 (;=0.001;)
            f64.mul
            call $clamp01
            local.tee 8
            f64.const 0x1p+1 (;=2;)
            call $scale
            local.tee 8
            local.get 7
            f64.const 0x1p-1 (;=0.5;)
            call $mix
            local.set 8
            local.get 3
            i32.const 1
            i32.add
            local.get 4
            local.get 8
            f64.add
            local.set 4
            local.set 3
            br 3 (;@1;)
          else
            local.get 5
            i32.const 8
            i32.add
            i32.const 16
            i32.const 1065240
            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
            unreachable
          end
        else
          local.get 5
          i32.const 16
          i32.const 1065372
          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
          unreachable
        end
      else
        local.get 9
        local.tee 3
        local.get 4
        f64.store align=1
        local.get 3
        i32.const 8
        call $_RNvCskgam9jYqc1y_10hostcall_f10emit_bytes
        local.get 4
        i64.reinterpret_f64
        local.get 9
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
    end
    unreachable
  )
```

## LLVM -O2 `run` (7,040,280 instructions, 100.0%)

```wat
(func $run (;9;) (type 9) (param i64) (result i64)
    (local i32 f64 i32 f64 f64 i32 f64 f64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i64.const 17
    i64.rem_u
    f64.convert_i64_u
    f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
    f64.mul
    local.set 2
    i32.const 0
    local.set 3
    f64.const 0x0p+0 (;=0;)
    local.set 4
    f64.const 0x0p+0 (;=0;)
    local.set 5
    loop ;; label = @1
      local.get 4
      local.get 2
      local.get 5
      f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
      f64.mul
      f64.add
      local.get 3
      i32.const 3
      i32.shl
      i32.const 56
      i32.and
      local.tee 6
      f64.load offset=1048848
      local.tee 7
      local.get 6
      f64.load offset=1048912
      local.tee 8
      f64.const 0x1p-1 (;=0.5;)
      f64.const 0x1p-2 (;=0.25;)
      call $_RNvCsjXLsirhmX2U_10hostcall_f4dot2
      f64.add
      local.get 7
      call $_RNvCsjXLsirhmX2U_10hostcall_f7length2
      f64.const 0x1.0624dd2f1a9fcp-10 (;=0.001;)
      f64.mul
      call $_RNvCsjXLsirhmX2U_10hostcall_f7clamp01
      f64.const 0x1p+1 (;=2;)
      call $_RNvCsjXLsirhmX2U_10hostcall_f5scale
      local.get 8
      f64.const 0x1p-1 (;=0.5;)
      call $_RNvCsjXLsirhmX2U_10hostcall_f3mix
      f64.add
      local.set 4
      local.get 5
      f64.const 0x1p+0 (;=1;)
      f64.add
      local.set 5
      local.get 3
      i32.const 1
      i32.add
      local.tee 3
      i32.const 40000
      i32.ne
      br_if 0 (;@1;)
    end
    local.get 1
    local.get 4
    f64.store offset=8
    block ;; label = @1
      i32.const 0
      i32.const 16384
      i32.const 0
      i32.load offset=1065560
      local.tee 3
      i32.sub
      local.tee 6
      local.get 6
      i32.const 16384
      i32.gt_u
      select
      local.tee 6
      i32.const 8
      local.get 6
      i32.const 8
      i32.lt_u
      select
      local.tee 9
      local.get 3
      i32.add
      local.tee 6
      local.get 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 6
      i32.const 16385
      i32.ge_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 9
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049176
        i32.add
        local.get 1
        i32.const 8
        i32.add
        local.get 9
        memory.copy
      end
      i32.const 0
      local.get 6
      i32.store offset=1065560
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 4
      i64.reinterpret_f64
      return
    end
    local.get 3
    local.get 6
    i32.const 16384
    i32.const 1048828
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )
```

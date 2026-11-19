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

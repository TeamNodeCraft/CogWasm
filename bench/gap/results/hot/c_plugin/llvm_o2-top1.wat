(func $run (;5;) (type 6) (param i64) (result i64)
    (local i32 f64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i64.const 97
    i64.rem_u
    f64.convert_i64_u
    f64.const 0x1p-3 (;=0.125;)
    f64.mul
    local.tee 2
    f64.const 0x1p+0 (;=1;)
    f64.add
    call $_RNvCsjr8rPXcfihl_8c_plugin9host_bias
    f64.const 0x1p+1 (;=2;)
    local.get 2
    f64.sub
    call $_RNvCsjr8rPXcfihl_8c_plugin9host_bias
    f64.add
    local.get 2
    f64.const 0x1.8p+1 (;=3;)
    f64.add
    call $_RNvCsjr8rPXcfihl_8c_plugin9host_bias
    f64.add
    f64.const 0x1p+2 (;=4;)
    local.get 2
    f64.sub
    call $_RNvCsjr8rPXcfihl_8c_plugin9host_bias
    f64.add
    f64.const 0x1p-2 (;=0.25;)
    f64.mul
    local.tee 2
    f64.store offset=8
    block ;; label = @1
      i32.const 0
      i32.const 16384
      i32.const 0
      i32.load offset=1065428
      local.tee 3
      i32.sub
      local.tee 4
      local.get 4
      i32.const 16384
      i32.gt_u
      select
      local.tee 4
      i32.const 8
      local.get 4
      i32.const 8
      i32.lt_u
      select
      local.tee 5
      local.get 3
      i32.add
      local.tee 4
      local.get 3
      i32.lt_u
      br_if 0 (;@1;)
      local.get 4
      i32.const 16385
      i32.ge_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049044
        i32.add
        local.get 1
        i32.const 8
        i32.add
        local.get 5
        memory.copy
      end
      i32.const 0
      local.get 4
      i32.store offset=1065428
      local.get 1
      i32.const 16
      i32.add
      global.set $__stack_pointer
      local.get 2
      i64.reinterpret_f64
      return
    end
    local.get 3
    local.get 4
    i32.const 16384
    i32.const 1048828
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )

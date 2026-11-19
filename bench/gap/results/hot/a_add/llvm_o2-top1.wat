(func $run (;4;) (type 5) (param i64) (result i64)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i32.wrap_i64
    local.get 0
    i64.const 32
    i64.shr_u
    i32.wrap_i64
    i32.add
    i32.const 20
    i32.add
    local.tee 2
    i32.store offset=12
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
      i32.const 4
      local.get 4
      i32.const 4
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
        i32.const 12
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
      i64.extend_i32_u
      return
    end
    local.get 3
    local.get 4
    i32.const 16384
    i32.const 1048828
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )

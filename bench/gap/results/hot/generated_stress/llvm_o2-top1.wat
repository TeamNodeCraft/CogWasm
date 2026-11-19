(func $run (;1036;) (type 0) (param i64) (result i64)
    (local i32 i64 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i64.const 0
    local.set 2
    i32.const 1048844
    local.set 3
    i32.const 0
    local.set 4
    loop ;; label = @1
      local.get 0
      local.get 3
      i32.load
      call_indirect (type 0)
      local.get 2
      i64.add
      local.set 2
      block ;; label = @2
        local.get 4
        i32.const 63
        i32.and
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i64.store offset=8
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.const 16384
            i32.const 0
            i32.load offset=1069524
            local.tee 5
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
            local.tee 7
            local.get 5
            i32.add
            local.tee 6
            local.get 5
            i32.lt_u
            br_if 0 (;@4;)
            local.get 6
            i32.const 16385
            i32.lt_u
            br_if 1 (;@3;)
          end
          local.get 5
          local.get 6
          i32.const 16384
          i32.const 1048828
          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
          unreachable
        end
        block ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 5
          i32.const 1053140
          i32.add
          local.get 1
          i32.const 8
          i32.add
          local.get 7
          memory.copy
        end
        i32.const 0
        local.get 6
        i32.store offset=1069524
      end
      local.get 3
      i32.const 4
      i32.add
      local.set 3
      local.get 4
      i32.const 1
      i32.add
      local.tee 4
      i32.const 1024
      i32.ne
      br_if 0 (;@1;)
    end
    local.get 1
    local.get 2
    i64.store offset=8
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1069524
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
        br_if 0 (;@2;)
        local.get 4
        i32.const 16385
        i32.lt_u
        br_if 1 (;@1;)
      end
      local.get 3
      local.get 4
      i32.const 16384
      i32.const 1048828
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    block ;; label = @1
      local.get 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      i32.const 1053140
      i32.add
      local.get 1
      i32.const 8
      i32.add
      local.get 5
      memory.copy
    end
    i32.const 0
    local.get 4
    i32.store offset=1069524
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 2
  )

(func $_RNvCsgBEE3CyPQPa_9int_loops10emit_bytes (;2;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 8
    global.set $__stack_pointer
    local.get 8
    local.tee 2
    local.get 0
    i32.store align=1
    local.get 8
    i32.const 4
    i32.add
    local.tee 0
    local.get 1
    i32.store align=1
    i32.const 1048724
    local.set 1
    local.get 8
    i32.const 12
    i32.add
    local.get 1
    i32.store align=1
    local.get 8
    i32.const 8
    i32.add
    local.tee 3
    local.get 1
    i32.store align=1
    local.get 1
    i32.load offset=16384 align=1
    local.set 4
    local.get 0
    i32.load align=1
    local.set 0
    block ;; label = @1
      local.get 8
      i32.const 16
      i32.add
      local.tee 5
      local.get 1
      i32.store align=1
      local.get 8
      i32.const 20
      i32.add
      i32.const 16384
      i32.store align=1
      local.get 0
      local.tee 9
      i32.const 0
      i32.const 16384
      local.get 4
      i32.sub
      local.get 4
      i32.const 16384
      i32.gt_u
      select
      local.tee 10
      local.get 9
      local.get 10
      i32.lt_u
      select
      local.set 1
      local.get 4
      local.get 1
      i32.add
      local.tee 6
      local.get 4
      i32.lt_u
      if ;; label = @2
        br 1 (;@1;)
      else
        local.get 6
        i32.const 16384
        i32.le_u
        if ;; label = @3
          local.get 8
          i32.const 24
          i32.add
          local.tee 7
          local.get 5
          i32.load align=1
          local.get 4
          i32.add
          i32.store align=1
          local.get 8
          i32.const 28
          i32.add
          local.get 1
          i32.store align=1
          local.get 7
          i32.load align=1
          local.set 7
          local.get 1
          local.get 0
          i32.le_u
          if ;; label = @4
            local.get 8
            i32.const 32
            i32.add
            local.tee 0
            local.get 2
            i32.load align=1
            i32.store align=1
            local.get 8
            i32.const 36
            i32.add
            local.get 1
            i32.store align=1
            local.get 7
            local.get 1
            local.get 0
            i32.load align=1
            local.get 1
            i32.const 1048692
            call $_RINvNtCsk7Mwp4gGzxp_4core5slice20copy_from_slice_implhECsgBEE3CyPQPa_9int_loops
            local.get 3
            i32.load align=1
            local.get 6
            i32.store offset=16384 align=1
            local.get 8
            i32.const 48
            i32.add
            global.set $__stack_pointer
            return
          else
            i32.const 0
            local.get 1
            local.get 0
            i32.const 1048576
            call $_RNvNtNtCsk7Mwp4gGzxp_4core5slice5index16slice_index_fail
            unreachable
          end
        else
          br 2 (;@1;)
        end
      end
    end
    local.get 4
    local.get 6
    i32.const 16384
    i32.const 1048708
    call $_RNvNtNtCsk7Mwp4gGzxp_4core5slice5index16slice_index_fail
    unreachable
    unreachable
  )

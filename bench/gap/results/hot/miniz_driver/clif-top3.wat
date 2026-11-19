(func $_RNvNtNtCs408MSACshni_11miniz_oxide7deflate4core14record_literal (;78;) (type 7) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 2
    i32.const 255
    i32.and
    local.set 2
    local.get 1
    local.get 1
    i32.load offset=12 align=1
    i32.const 1
    i32.add
    i32.store offset=12 align=1
    local.get 1
    i32.load offset=4 align=1
    i32.const 65535
    i32.and
    local.tee 3
    i32.const 65536
    i32.lt_u
    if ;; label = @1
      local.get 1
      i32.load align=1
      local.set 4
      local.get 5
      i32.const 0
      i32.add
      local.get 4
      i32.store align=1
      local.get 4
      local.get 3
      i32.add
      local.get 2
      i32.store8
      local.get 1
      local.get 1
      i32.load offset=4 align=1
      i32.const 1
      i32.add
      i32.store offset=4 align=1
      local.get 1
      i32.load offset=8 align=1
      i32.const 65535
      i32.and
      local.tee 4
      i32.const 65536
      i32.lt_u
      if ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load align=1
          local.set 3
          local.get 5
          i32.const 8
          i32.add
          local.get 3
          i32.store align=1
          local.get 5
          i32.const 4
          i32.add
          local.get 3
          i32.store align=1
          local.get 3
          local.get 4
          i32.add
          local.tee 4
          local.get 4
          i32.load8_u
          i32.const 255
          i32.and
          i32.const 1
          i32.const 7
          i32.and
          i32.shr_u
          i32.store8
          local.get 1
          i32.load offset=16 align=1
          i32.const 1
          i32.sub
          local.set 4
          local.get 1
          local.get 4
          i32.store offset=16 align=1
          local.get 4
          if ;; label = @4
            br 1 (;@3;)
          else
            local.get 1
            i32.const 8
            i32.store offset=16 align=1
            local.get 1
            local.get 1
            i32.load offset=4 align=1
            i32.store offset=8 align=1
            local.get 1
            local.get 1
            i32.load offset=4 align=1
            i32.const 1
            i32.add
            i32.store offset=4 align=1
            br 1 (;@3;)
          end
        end
        local.get 2
        local.tee 2
        i32.const 288
        i32.lt_u
        if ;; label = @3
          local.get 0
          local.get 2
          i32.const 1
          i32.shl
          i32.add
          local.tee 2
          local.get 2
          i32.load16_u align=1
          i32.const 65535
          i32.and
          i32.const 1
          i32.add
          i32.const 65535
          i32.and
          i32.store16 align=1
          local.get 5
          i32.const 16
          i32.add
          global.set $__stack_pointer
          return
        else
          local.get 2
          i32.const 288
          i32.const 1072516
          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
          unreachable
        end
      else
        local.get 4
        i32.const 65536
        i32.const 1072648
        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
        unreachable
      end
    else
      local.get 3
      i32.const 65536
      i32.const 1072664
      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
      unreachable
    end
    unreachable
  )

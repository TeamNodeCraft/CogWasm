(func $_RINvNtCsk7Mwp4gGzxp_4core10intrinsics20select_unpredictableINtNtNtB4_3mem12maybe_uninit11MaybeUninitOyEECsbVGWwvyaa9p_5smoke (;0;) (type 1) (param i32 i32 i32) (result i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 0
    i32.const 255
    i32.and
    local.set 0
    local.get 4
    i32.const 4
    i32.add
    local.tee 3
    local.get 1
    i32.store align=1
    local.get 4
    i32.const 8
    i32.add
    local.get 2
    i32.store align=1
    local.get 0
    if ;; label = @1
      local.get 3
      i32.load align=1
      local.set 3
      local.get 4
      local.get 3
      i32.store align=1
      local.get 3
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    else
      local.get 4
      local.get 2
      i32.store align=1
      local.get 2
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
  )

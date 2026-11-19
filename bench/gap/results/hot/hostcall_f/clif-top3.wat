(func $_RINvNtCsk7Mwp4gGzxp_4core5slice20copy_from_slice_implhECskgam9jYqc1y_10hostcall_f (;5;) (type 0) (param i32 i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 6
    global.set $__stack_pointer
    local.get 6
    local.tee 5
    local.get 0
    i32.store align=1
    local.get 6
    i32.const 4
    i32.add
    local.tee 0
    local.get 1
    i32.store align=1
    local.get 6
    i32.const 8
    i32.add
    local.tee 1
    local.get 2
    i32.store align=1
    local.get 6
    i32.const 12
    i32.add
    local.get 3
    i32.store align=1
    local.get 5
    i32.load align=1
    local.set 5
    local.get 0
    i32.load align=1
    local.set 0
    local.get 1
    i32.load align=1
    local.set 1
    local.get 0
    local.get 3
    i32.ne
    if ;; label = @1
      local.get 0
      local.get 3
      local.get 4
      call $_RNvNvNtCsk7Mwp4gGzxp_4core5slice20copy_from_slice_impl17len_mismatch_fail
      unreachable
    else
      local.get 5
      local.get 1
      local.get 0
      memory.copy
      local.get 6
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
  )

(func $_RNvMNtCsk7Mwp4gGzxp_4core5sliceSy14swap_uncheckedCsknsHYGPguHZ_4sort (;52;) (type 5) (param i32 i32 i32 i32 i32)
    (local i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.tee 4
    local.get 0
    i32.store align=1
    local.get 5
    i32.const 4
    i32.add
    local.get 1
    i32.store align=1
    local.get 4
    i32.load align=1
    local.tee 4
    local.get 2
    i32.const 3
    i32.shl
    i32.add
    local.set 2
    local.get 5
    i32.const 8
    i32.add
    local.tee 1
    local.get 2
    i64.load
    i64.store
    local.get 4
    local.get 3
    i32.const 3
    i32.shl
    i32.add
    local.set 4
    local.get 2
    local.get 4
    i64.load
    i64.store
    local.get 4
    local.get 1
    i64.load
    i64.store
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

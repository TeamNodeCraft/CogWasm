(func $_RNCINvNtNtNtNtCsk7Mwp4gGzxp_4core5slice4sort8unstable9quicksort34partition_lomuto_branchless_cyclicyNvYyNtNtBc_3cmp10PartialOrd2ltE0CsknsHYGPguHZ_4sort (;39;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 0
    i32.load align=1
    local.set 2
    local.get 1
    i32.load offset=8 align=1
    local.set 3
    local.get 5
    i32.const 0
    i32.add
    local.get 3
    i32.store align=1
    local.get 2
    local.get 3
    local.get 0
    i32.load offset=4 align=1
    call $_RNvYNvYyNtNtCsk7Mwp4gGzxp_4core3cmp10PartialOrd2ltINtNtNtBa_3ops8function5FnMutTRyB1g_EE8call_mutCsknsHYGPguHZ_4sort
    i32.const 255
    i32.and
    local.set 3
    local.get 0
    i32.load offset=8 align=1
    i32.load align=1
    local.set 0
    local.get 1
    i32.load offset=12 align=1
    local.set 2
    local.get 1
    i32.load align=1
    local.set 4
    local.get 0
    local.get 2
    i32.const 3
    i32.shl
    i32.add
    local.set 2
    local.get 4
    local.get 2
    i32.const 8
    call $memmove
    drop
    local.get 1
    i32.load offset=8 align=1
    local.set 4
    local.get 2
    local.get 4
    i32.const 8
    call $memcpy
    drop
    local.get 1
    local.get 1
    i32.load offset=8 align=1
    i32.store align=1
    local.get 1
    local.get 1
    i32.load offset=12 align=1
    local.get 3
    i32.add
    i32.store offset=12 align=1
    local.get 1
    local.get 1
    i32.load offset=8 align=1
    i32.const 8
    i32.add
    i32.store offset=8 align=1
    local.get 5
    i32.const 16
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

(func $_RINvNtNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6shared9smallsort12swap_if_lessyNvYyNtNtBa_3cmp10PartialOrd2ltECsbVGWwvyaa9p_5smoke (;19;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    local.tee 4
    local.get 0
    local.get 1
    i32.const 3
    i32.shl
    i32.add
    i32.store align=1
    local.get 0
    local.get 2
    i32.const 3
    i32.shl
    i32.add
    local.set 2
    local.get 5
    i32.const 4
    i32.add
    local.tee 0
    local.get 2
    i32.store align=1
    local.get 3
    local.get 2
    local.get 4
    i32.load align=1
    call $_RNvYNvYyNtNtCsk7Mwp4gGzxp_4core3cmp10PartialOrd2ltINtNtNtBa_3ops8function5FnMutTRyB1g_EE8call_mutCsbVGWwvyaa9p_5smoke
    i32.const 255
    i32.and
    local.tee 3
    local.get 0
    i32.load align=1
    local.get 4
    i32.load align=1
    call $_RINvNtCsk7Mwp4gGzxp_4core4hint20select_unpredictableOyECsbVGWwvyaa9p_5smoke
    local.set 2
    local.get 3
    local.get 4
    i32.load align=1
    local.get 0
    i32.load align=1
    call $_RINvNtCsk7Mwp4gGzxp_4core4hint20select_unpredictableOyECsbVGWwvyaa9p_5smoke
    local.set 3
    local.get 5
    i32.const 8
    i32.add
    local.tee 1
    local.get 3
    i64.load align=1
    i64.store align=1
    local.get 4
    i32.load align=1
    local.get 2
    i64.load
    i64.store
    local.get 0
    i32.load align=1
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

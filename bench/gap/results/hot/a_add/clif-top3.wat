(func $_RNvCs1lxurIFlWxT_5a_add4body (;2;) (type 2) (param i64) (result i64)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 0
    i32.wrap_i64
    local.get 0
    i32.const 32
    i64.extend_i32_u
    i64.shr_u
    i32.wrap_i64
    i32.const 20
    i32.add
    i32.add
    local.set 1
    local.get 3
    local.tee 2
    local.get 1
    i32.store align=1
    local.get 2
    i32.const 4
    call $_RNvCs1lxurIFlWxT_5a_add10emit_bytes
    local.get 1
    i64.extend_i32_u
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

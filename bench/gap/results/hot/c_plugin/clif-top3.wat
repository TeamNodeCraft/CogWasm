(func $_RNvCskuAql209qb2_8c_plugin4body (;6;) (type 5) (param i64) (result i64)
    (local i64 f64 i32 i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 0
    local.set 5
    i64.const 5895351198814392785
    local.set 6
    local.get 5
    i64.const 4294967295
    i64.and
    local.get 6
    i64.const 32
    i64.shr_u
    i64.mul
    local.set 7
    local.get 5
    i64.const 32
    i64.shr_u
    local.get 6
    i64.const 4294967295
    i64.and
    i64.mul
    local.set 8
    local.get 5
    i64.const 32
    i64.shr_u
    local.get 6
    i64.const 32
    i64.shr_u
    i64.mul
    local.get 7
    i64.const 32
    i64.shr_u
    i64.add
    local.get 8
    i64.const 32
    i64.shr_u
    i64.add
    local.get 5
    i64.const 4294967295
    i64.and
    local.get 6
    i64.const 4294967295
    i64.and
    i64.mul
    i64.const 32
    i64.shr_u
    local.get 7
    i64.const 4294967295
    i64.and
    i64.add
    local.get 8
    i64.const 4294967295
    i64.and
    i64.add
    i64.const 32
    i64.shr_u
    i64.add
    local.set 1
    local.get 0
    local.get 1
    local.get 0
    local.get 1
    i64.sub
    i64.const 1
    i64.shr_u
    i64.add
    i64.const 6
    i64.shr_u
    i64.const 97
    i64.mul
    i64.sub
    f64.convert_i64_u
    f64.const 0x1p-3 (;=0.125;)
    f64.mul
    local.set 2
    local.get 4
    local.tee 3
    f64.const 0x1p+0 (;=1;)
    local.get 2
    f64.add
    f64.store align=1
    local.get 3
    i32.const 8
    i32.add
    f64.const 0x1p+1 (;=2;)
    local.get 2
    f64.sub
    f64.store align=1
    local.get 3
    i32.const 16
    i32.add
    f64.const 0x1.8p+1 (;=3;)
    local.get 2
    f64.add
    f64.store align=1
    local.get 3
    i32.const 24
    i32.add
    f64.const 0x1p+2 (;=4;)
    local.get 2
    f64.sub
    f64.store align=1
    local.get 3
    i32.const 4
    call $_RNvCskuAql209qb2_8c_plugin4eval
    local.set 2
    local.get 4
    i32.const 32
    i32.add
    local.tee 3
    local.get 2
    f64.store align=1
    local.get 3
    i32.const 8
    call $_RNvCskuAql209qb2_8c_plugin10emit_bytes
    local.get 2
    i64.reinterpret_f64
    local.get 4
    i32.const 48
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

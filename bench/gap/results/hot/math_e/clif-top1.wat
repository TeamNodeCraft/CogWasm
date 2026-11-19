(func $_RNvXsy_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyNtB5_4HInt9widen_mul (;58;) (type 22) (param i32 i64 i64)
    (local i64 i64 i64 i64)
    local.get 0
    local.get 1
    local.get 2
    i64.mul
    i64.store align=1
    local.get 0
    local.get 1
    local.set 3
    local.get 2
    local.set 4
    local.get 3
    i64.const 4294967295
    i64.and
    local.get 4
    i64.const 32
    i64.shr_u
    i64.mul
    local.set 5
    local.get 3
    i64.const 32
    i64.shr_u
    local.get 4
    i64.const 4294967295
    i64.and
    i64.mul
    local.set 6
    local.get 3
    i64.const 32
    i64.shr_u
    local.get 4
    i64.const 32
    i64.shr_u
    i64.mul
    local.get 5
    i64.const 32
    i64.shr_u
    i64.add
    local.get 6
    i64.const 32
    i64.shr_u
    i64.add
    local.get 3
    i64.const 4294967295
    i64.and
    local.get 4
    i64.const 4294967295
    i64.and
    i64.mul
    i64.const 32
    i64.shr_u
    local.get 5
    i64.const 4294967295
    i64.and
    i64.add
    local.get 6
    i64.const 4294967295
    i64.and
    i64.add
    i64.const 32
    i64.shr_u
    i64.add
    local.get 1
    i64.const 0
    i64.mul
    i64.add
    i64.const 0
    local.get 2
    i64.mul
    i64.add
    i64.store offset=8 align=1
    return
    unreachable
  )

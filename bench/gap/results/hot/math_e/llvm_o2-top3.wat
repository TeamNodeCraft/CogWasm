(func $_RNvNtNtCs3at74BFuvPN_4libm4math4sqrt4sqrt (;9;) (type 8) (param f64) (result f64)
    (local i32 i64 i32 f64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        local.get 0
        i64.reinterpret_f64
        local.tee 2
        i64.const 52
        i64.shr_u
        i32.wrap_i64
        local.tee 3
        i32.const -2047
        i32.add
        i32.const -2047
        i32.gt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 0
          f64.const 0x0p+0 (;=0;)
          f64.ne
          br_if 0 (;@3;)
          local.get 0
          local.set 4
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 2
          i64.const 9218868437227405312
          i64.ne
          br_if 0 (;@3;)
          local.get 0
          local.set 4
          br 2 (;@1;)
        end
        f64.const nan (;=NaN;)
        local.set 4
        local.get 2
        i64.const 9218868437227405312
        i64.gt_u
        br_if 1 (;@1;)
        local.get 0
        f64.const 0x1p+52 (;=4503599627370496;)
        f64.mul
        i64.reinterpret_f64
        local.tee 2
        i64.const 52
        i64.shr_u
        i32.wrap_i64
        i32.const 2047
        i32.and
        i32.const -52
        i32.add
        local.set 3
      end
      local.get 1
      i32.const 32
      i32.add
      i64.const 3221225472
      local.get 2
      i64.const 11
      i64.shl
      i64.const -9223372036854775808
      i64.or
      local.get 3
      i32.const 1
      i32.and
      i64.extend_i32_u
      i64.shr_u
      local.tee 5
      i64.const 32
      i64.shr_u
      local.get 2
      i64.const 46
      i64.shr_u
      i32.wrap_i64
      i32.const 127
      i32.and
      i32.const 1
      i32.shl
      i64.load16_u offset=1048844
      i64.const 16
      i64.shl
      local.tee 2
      i64.mul
      i64.const 32
      i64.shr_u
      local.tee 6
      local.get 2
      i64.mul
      i64.const 32
      i64.shr_u
      i64.sub
      i64.const 4294967295
      i64.and
      local.tee 7
      local.get 2
      i64.mul
      i64.const 31
      i64.shr_u
      i64.const 4294967294
      i64.and
      local.tee 2
      i64.const 3221225472
      local.get 2
      local.get 7
      local.get 6
      i64.mul
      i64.const 31
      i64.shr_u
      i64.const 4294967294
      i64.and
      i64.mul
      i64.const 32
      i64.shr_u
      i64.sub
      i64.const 4294967295
      i64.and
      i64.mul
      i64.const 1
      i64.shl
      i64.const -8589934592
      i64.and
      local.tee 2
      i64.const 0
      local.get 5
      i64.const 0
      call $__multi3
      local.get 1
      i32.const 16
      i32.add
      local.get 1
      i64.load offset=40
      local.tee 6
      i64.const 0
      local.get 2
      i64.const 0
      call $__multi3
      local.get 1
      local.get 6
      i64.const 0
      i64.const -4611686018427387904
      local.get 1
      i64.load offset=24
      i64.sub
      i64.const 0
      call $__multi3
      local.get 1
      i64.load offset=8
      i64.const 9
      i64.shr_u
      local.tee 2
      local.get 2
      i64.mul
      local.get 5
      i64.const 42
      i64.shl
      i64.sub
      local.get 2
      i64.add
      local.tee 5
      i64.const 63
      i64.shr_u
      local.get 2
      i64.add
      i64.const 4503599627370495
      i64.and
      local.get 3
      i32.const 1023
      i32.add
      i32.const 1
      i32.shr_u
      i64.extend_i32_u
      i64.const 52
      i64.shl
      i64.or
      local.tee 2
      f64.reinterpret_i64
      i64.const 0
      i64.const 4503599627370496
      local.get 5
      local.get 2
      i64.add
      i64.const 1
      i64.add
      local.tee 2
      i64.eqz
      select
      local.get 2
      local.get 5
      i64.xor
      i64.const -9223372036854775808
      i64.and
      i64.or
      f64.reinterpret_i64
      f64.add
      local.set 4
    end
    local.get 1
    i32.const 48
    i32.add
    global.set $__stack_pointer
    local.get 4
  )

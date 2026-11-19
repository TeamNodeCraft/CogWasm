(func $_RNvCskgam9jYqc1y_10hostcall_f4body (;8;) (type 3) (param i64) (result i64)
    (local i32 f64 i32 f64 i32 f64 f64 f64 i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 9
    global.set $__stack_pointer
    i32.const 1065112
    local.set 1
    local.get 0
    local.get 0
    local.set 10
    i64.const -1085102592571150095
    local.set 11
    local.get 10
    i64.const 4294967295
    i64.and
    local.get 11
    i64.const 32
    i64.shr_u
    i64.mul
    local.set 12
    local.get 10
    i64.const 32
    i64.shr_u
    local.get 11
    i64.const 4294967295
    i64.and
    i64.mul
    local.set 13
    local.get 10
    i64.const 32
    i64.shr_u
    local.get 11
    i64.const 32
    i64.shr_u
    i64.mul
    local.get 12
    i64.const 32
    i64.shr_u
    i64.add
    local.get 13
    i64.const 32
    i64.shr_u
    i64.add
    local.get 10
    i64.const 4294967295
    i64.and
    local.get 11
    i64.const 4294967295
    i64.and
    i64.mul
    i64.const 32
    i64.shr_u
    local.get 12
    i64.const 4294967295
    i64.and
    i64.add
    local.get 13
    i64.const 4294967295
    i64.and
    i64.add
    i64.const 32
    i64.shr_u
    i64.add
    i64.const 4
    i64.shr_u
    i64.const 17
    i64.mul
    i64.sub
    f64.convert_i64_u
    f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
    f64.mul
    local.set 2
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    local.set 4
    local.set 3
    loop ;; label = @1
      local.get 3
      i32.const 40000
      i32.lt_u
      if ;; label = @2
        local.get 3
        i32.const 7
        i32.and
        local.tee 5
        i32.const 16
        i32.lt_u
        if ;; label = @3
          local.get 1
          local.get 5
          i32.const 3
          i32.shl
          i32.add
          f64.load align=1
          local.set 6
          local.get 5
          i32.const 8
          i32.add
          i32.const 16
          i32.lt_u
          if ;; label = @4
            local.get 1
            local.get 5
            i32.const 8
            i32.add
            i32.const 3
            i32.shl
            i32.add
            f64.load align=1
            local.set 7
            local.get 6
            local.get 7
            f64.const 0x1p-1 (;=0.5;)
            f64.const 0x1p-2 (;=0.25;)
            call $dot2
            local.tee 8
            local.get 3
            f64.convert_i32_u
            f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
            f64.mul
            local.get 2
            f64.add
            f64.add
            local.get 6
            call $length2
            local.tee 8
            f64.const 0x1.0624dd2f1a9fcp-10 (;=0.001;)
            f64.mul
            call $clamp01
            local.tee 8
            f64.const 0x1p+1 (;=2;)
            call $scale
            local.tee 8
            local.get 7
            f64.const 0x1p-1 (;=0.5;)
            call $mix
            local.set 8
            local.get 3
            i32.const 1
            i32.add
            local.get 4
            local.get 8
            f64.add
            local.set 4
            local.set 3
            br 3 (;@1;)
          else
            local.get 5
            i32.const 8
            i32.add
            i32.const 16
            i32.const 1065240
            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
            unreachable
          end
        else
          local.get 5
          i32.const 16
          i32.const 1065372
          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
          unreachable
        end
      else
        local.get 9
        i32.const 0
        i32.add
        local.tee 3
        local.get 4
        f64.store align=1
        local.get 3
        i32.const 8
        call $_RNvCskgam9jYqc1y_10hostcall_f10emit_bytes
        local.get 4
        i64.reinterpret_f64
        local.get 9
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
    end
    unreachable
  )

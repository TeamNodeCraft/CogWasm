(func $_RNvNtNtCs3at74BFuvPN_4libm4math5log1p5log1p (;13;) (type 6) (param f64) (result f64)
    (local i32 i64 i32 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.set 1
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 0
                  i64.reinterpret_f64
                  local.tee 2
                  i64.const 4601133429810003967
                  i64.gt_s
                  br_if 0 (;@7;)
                  local.get 2
                  i64.const -4616189618054758401
                  i64.gt_u
                  br_if 3 (;@4;)
                  local.get 2
                  i64.const 32
                  i64.shr_u
                  i32.wrap_i64
                  local.tee 3
                  i32.const 1
                  i32.shl
                  i32.const 2034237440
                  i32.ge_u
                  br_if 1 (;@6;)
                  local.get 3
                  i32.const 2146435072
                  i32.and
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 0
                  return
                end
                local.get 2
                i64.const 9218868437227405311
                i64.le_u
                br_if 1 (;@5;)
                local.get 0
                return
              end
              f64.const 0x0p+0 (;=0;)
              local.set 4
              local.get 2
              i64.const -4624424114038243329
              i64.gt_u
              br_if 0 (;@5;)
              f64.const 0x0p+0 (;=0;)
              local.set 5
              br 4 (;@1;)
            end
            local.get 0
            f64.const 0x1p+0 (;=1;)
            f64.add
            local.tee 5
            i64.reinterpret_f64
            local.tee 2
            i64.const 32
            i64.shr_u
            i32.wrap_i64
            i32.const 614242
            i32.add
            local.tee 1
            i32.const 20
            i32.shr_u
            i32.const -1023
            i32.add
            local.set 3
            f64.const 0x0p+0 (;=0;)
            local.set 4
            block ;; label = @5
              local.get 1
              i32.const 1129316352
              i32.ge_u
              br_if 0 (;@5;)
              local.get 0
              local.get 5
              f64.sub
              f64.const 0x1p+0 (;=1;)
              f64.add
              local.get 0
              local.get 5
              f64.const -0x1p+0 (;=-1;)
              f64.add
              f64.sub
              local.get 1
              i32.const 1074790399
              i32.gt_u
              select
              local.get 5
              f64.div
              local.set 4
            end
            local.get 1
            i32.const 1048575
            i32.and
            i32.const 1072079006
            i32.add
            i64.extend_i32_u
            i64.const 32
            i64.shl
            local.get 2
            i64.const 4294967295
            i64.and
            i64.or
            f64.reinterpret_i64
            f64.const -0x1p+0 (;=-1;)
            f64.add
            local.set 0
            local.get 3
            f64.convert_i32_s
            local.set 5
            br 3 (;@1;)
          end
          local.get 0
          f64.const -0x1p+0 (;=-1;)
          f64.ne
          br_if 1 (;@2;)
          f64.const -inf (;=-inf;)
          return
        end
        local.get 1
        local.get 0
        f32.demote_f64
        f32.store offset=12
        local.get 1
        f32.load offset=12
        drop
        local.get 0
        return
      end
      local.get 0
      local.get 0
      f64.sub
      f64.const 0x0p+0 (;=0;)
      f64.div
      return
    end
    local.get 5
    f64.const 0x1.62e42feep-1 (;=0.6931471803691238;)
    f64.mul
    local.get 0
    local.get 4
    local.get 5
    f64.const 0x1.a39ef35793c76p-33 (;=0.00000000019082149292705877;)
    f64.mul
    f64.add
    local.get 0
    local.get 0
    f64.const 0x1p+1 (;=2;)
    f64.add
    f64.div
    local.tee 5
    local.get 0
    local.get 0
    f64.const 0x1p-1 (;=0.5;)
    f64.mul
    f64.mul
    local.tee 6
    local.get 5
    local.get 5
    f64.mul
    local.tee 4
    local.get 4
    f64.mul
    local.tee 5
    local.get 5
    local.get 5
    f64.const 0x1.39a09d078c69fp-3 (;=0.15313837699209373;)
    f64.mul
    f64.const 0x1.c71c51d8e78afp-3 (;=0.22222198432149784;)
    f64.add
    f64.mul
    f64.const 0x1.999999997fa04p-2 (;=0.3999999999940942;)
    f64.add
    f64.mul
    local.get 4
    local.get 5
    local.get 5
    local.get 5
    f64.const 0x1.2f112df3e5244p-3 (;=0.14798198605116586;)
    f64.mul
    f64.const 0x1.7466496cb03dep-3 (;=0.1818357216161805;)
    f64.add
    f64.mul
    f64.const 0x1.2492494229359p-2 (;=0.2857142874366239;)
    f64.add
    f64.mul
    f64.const 0x1.5555555555593p-1 (;=0.6666666666666735;)
    f64.add
    f64.mul
    f64.add
    f64.add
    f64.mul
    f64.add
    local.get 6
    f64.sub
    f64.add
    f64.add
  )

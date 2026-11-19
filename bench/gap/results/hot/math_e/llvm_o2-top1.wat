(func $_RNvNtNtCs3at74BFuvPN_4libm4math8rem_pio28rem_pio2 (;6;) (type 7) (param i32 f64)
    (local i32 i64 i32 i32 f64 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i64.reinterpret_f64
          local.tee 3
          i64.const 32
          i64.shr_u
          i32.wrap_i64
          local.tee 4
          i32.const 2147483647
          i32.and
          local.tee 5
          i32.const 1074752123
          i32.lt_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.const 1075594812
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 5
                  i32.const 1094263291
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 2146435071
                  i32.gt_u
                  br_if 1 (;@6;)
                  local.get 2
                  local.get 3
                  i64.const 4503599627370495
                  i64.and
                  i64.const 4710765210229538816
                  i64.or
                  f64.reinterpret_i64
                  local.tee 1
                  i32.trunc_sat_f64_s
                  f64.convert_i32_s
                  local.tee 6
                  f64.store
                  local.get 2
                  local.get 1
                  local.get 6
                  f64.sub
                  f64.const 0x1p+24 (;=16777216;)
                  f64.mul
                  local.tee 1
                  i32.trunc_sat_f64_s
                  local.tee 4
                  f64.convert_i32_s
                  local.tee 6
                  f64.store offset=8
                  local.get 2
                  local.get 1
                  local.get 6
                  f64.sub
                  f64.const 0x1p+24 (;=16777216;)
                  f64.mul
                  local.tee 1
                  f64.store offset=16
                  local.get 2
                  i64.const 0
                  i64.store offset=40
                  local.get 2
                  i64.const 0
                  i64.store offset=32
                  local.get 2
                  i64.const 0
                  i64.store offset=24
                  local.get 2
                  i32.const 2
                  i32.const 1
                  local.get 4
                  select
                  i32.const 3
                  local.get 1
                  f64.const 0x0p+0 (;=0;)
                  f64.eq
                  select
                  local.get 2
                  i32.const 24
                  i32.add
                  i32.const 3
                  local.get 5
                  i32.const 20
                  i32.shr_u
                  i32.const -1046
                  i32.add
                  i32.const 1
                  call $_RNvNtNtCs3at74BFuvPN_4libm4math14rem_pio2_large14rem_pio2_large
                  local.set 5
                  local.get 3
                  i64.const -1
                  i64.le_s
                  br_if 2 (;@5;)
                  local.get 0
                  local.get 5
                  i32.store offset=8
                  local.get 0
                  local.get 2
                  f64.load offset=32
                  f64.store offset=16
                  local.get 0
                  local.get 2
                  f64.load offset=24
                  f64.store
                  br 6 (;@1;)
                end
                block ;; label = @7
                  local.get 5
                  i32.const 20
                  i32.shr_u
                  local.tee 5
                  local.get 1
                  local.get 1
                  f64.const 0x1.45f306dc9c883p-1 (;=0.6366197723675814;)
                  f64.mul
                  f64.const 0x1.8p+52 (;=6755399441055744;)
                  f64.add
                  f64.const -0x1.8p+52 (;=-6755399441055744;)
                  f64.add
                  local.tee 6
                  f64.const -0x1.921fb544p+0 (;=-1.5707963267341256;)
                  f64.mul
                  f64.add
                  local.tee 1
                  local.get 6
                  f64.const 0x1.0b4611a626331p-34 (;=0.00000000006077100506506192;)
                  f64.mul
                  local.tee 7
                  f64.sub
                  local.tee 8
                  i64.reinterpret_f64
                  i64.const 52
                  i64.shr_u
                  i32.wrap_i64
                  i32.const 2047
                  i32.and
                  i32.sub
                  i32.const 17
                  i32.lt_s
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 5
                    local.get 1
                    local.get 6
                    f64.const 0x1.0b4611a6p-34 (;=0.00000000006077100506303966;)
                    f64.mul
                    local.tee 8
                    f64.sub
                    local.tee 9
                    local.get 6
                    f64.const 0x1.3198a2e037073p-69 (;=0.0000000000000000000020222662487959506;)
                    f64.mul
                    local.get 1
                    local.get 9
                    f64.sub
                    local.get 8
                    f64.sub
                    f64.sub
                    local.tee 7
                    f64.sub
                    local.tee 8
                    i64.reinterpret_f64
                    i64.const 52
                    i64.shr_u
                    i32.wrap_i64
                    i32.const 2047
                    i32.and
                    i32.sub
                    i32.const 50
                    i32.ge_s
                    br_if 0 (;@8;)
                    local.get 9
                    local.set 1
                    br 1 (;@7;)
                  end
                  local.get 9
                  local.get 6
                  f64.const 0x1.3198a2ep-69 (;=0.0000000000000000000020222662487111665;)
                  f64.mul
                  local.tee 8
                  f64.sub
                  local.tee 1
                  local.get 6
                  f64.const 0x1.b839a252049c1p-104 (;=0.000000000000000000000000000000084784276603689;)
                  f64.mul
                  local.get 9
                  local.get 1
                  f64.sub
                  local.get 8
                  f64.sub
                  f64.sub
                  local.tee 7
                  f64.sub
                  local.set 8
                end
                local.get 0
                local.get 8
                f64.store
                local.get 0
                local.get 6
                i32.trunc_sat_f64_s
                i32.store offset=8
                local.get 0
                local.get 1
                local.get 8
                f64.sub
                local.get 7
                f64.sub
                f64.store offset=16
                br 5 (;@1;)
              end
              local.get 0
              i32.const 0
              i32.store offset=8
              local.get 0
              local.get 1
              local.get 1
              f64.sub
              local.tee 1
              f64.store offset=16
              local.get 0
              local.get 1
              f64.store
              br 4 (;@1;)
            end
            local.get 0
            i32.const 0
            local.get 5
            i32.sub
            i32.store offset=8
            local.get 0
            local.get 2
            f64.load offset=32
            f64.neg
            f64.store offset=16
            local.get 0
            local.get 2
            f64.load offset=24
            f64.neg
            f64.store
            br 3 (;@1;)
          end
          block ;; label = @4
            local.get 5
            i32.const 1075183037
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 5
              i32.const 1075388923
              i32.ne
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 1
                local.get 1
                f64.const 0x1.45f306dc9c883p-1 (;=0.6366197723675814;)
                f64.mul
                f64.const 0x1.8p+52 (;=6755399441055744;)
                f64.add
                f64.const -0x1.8p+52 (;=-6755399441055744;)
                f64.add
                local.tee 6
                f64.const -0x1.921fb544p+0 (;=-1.5707963267341256;)
                f64.mul
                f64.add
                local.tee 1
                local.get 6
                f64.const 0x1.0b4611a626331p-34 (;=0.00000000006077100506506192;)
                f64.mul
                local.tee 7
                f64.sub
                local.tee 8
                i64.reinterpret_f64
                i64.const 9218868437227405312
                i64.and
                i64.const 4544132024016830463
                i64.gt_u
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 1
                  local.get 6
                  f64.const 0x1.0b4611a6p-34 (;=0.00000000006077100506303966;)
                  f64.mul
                  local.tee 8
                  f64.sub
                  local.tee 9
                  local.get 6
                  f64.const 0x1.3198a2e037073p-69 (;=0.0000000000000000000020222662487959506;)
                  f64.mul
                  local.get 1
                  local.get 9
                  f64.sub
                  local.get 8
                  f64.sub
                  f64.sub
                  local.tee 7
                  f64.sub
                  local.tee 8
                  i64.reinterpret_f64
                  i64.const 9151314442816847872
                  i64.and
                  i64.const 4395513236313604095
                  i64.le_u
                  br_if 0 (;@7;)
                  local.get 9
                  local.set 1
                  br 1 (;@6;)
                end
                local.get 9
                local.get 6
                f64.const 0x1.3198a2ep-69 (;=0.0000000000000000000020222662487111665;)
                f64.mul
                local.tee 8
                f64.sub
                local.tee 1
                local.get 6
                f64.const 0x1.b839a252049c1p-104 (;=0.000000000000000000000000000000084784276603689;)
                f64.mul
                local.get 9
                local.get 1
                f64.sub
                local.get 8
                f64.sub
                f64.sub
                local.tee 7
                f64.sub
                local.set 8
              end
              local.get 0
              local.get 8
              f64.store
              local.get 0
              local.get 6
              i32.trunc_sat_f64_s
              i32.store offset=8
              local.get 0
              local.get 1
              local.get 8
              f64.sub
              local.get 7
              f64.sub
              f64.store offset=16
              br 4 (;@1;)
            end
            block ;; label = @5
              local.get 3
              i64.const 0
              i64.lt_s
              br_if 0 (;@5;)
              local.get 0
              i32.const 4
              i32.store offset=8
              local.get 0
              local.get 1
              f64.const -0x1.921fb544p+2 (;=-6.2831853069365025;)
              f64.add
              local.tee 1
              f64.const -0x1.0b4611a626331p-32 (;=-0.0000000002430840202602477;)
              f64.add
              local.tee 6
              f64.store
              local.get 0
              local.get 1
              local.get 6
              f64.sub
              f64.const -0x1.0b4611a626331p-32 (;=-0.0000000002430840202602477;)
              f64.add
              f64.store offset=16
              br 4 (;@1;)
            end
            local.get 0
            i32.const -4
            i32.store offset=8
            local.get 0
            local.get 1
            f64.const 0x1.921fb544p+2 (;=6.2831853069365025;)
            f64.add
            local.tee 1
            f64.const 0x1.0b4611a626331p-32 (;=0.0000000002430840202602477;)
            f64.add
            local.tee 6
            f64.store
            local.get 0
            local.get 1
            local.get 6
            f64.sub
            f64.const 0x1.0b4611a626331p-32 (;=0.0000000002430840202602477;)
            f64.add
            f64.store offset=16
            br 3 (;@1;)
          end
          local.get 5
          i32.const 1074977148
          i32.eq
          br_if 1 (;@2;)
          block ;; label = @4
            local.get 3
            i64.const 0
            i64.lt_s
            br_if 0 (;@4;)
            local.get 0
            i32.const 3
            i32.store offset=8
            local.get 0
            local.get 1
            f64.const -0x1.2d97c7f3p+2 (;=-4.712388980202377;)
            f64.add
            local.tee 1
            f64.const -0x1.90e91a79394cap-33 (;=-0.00000000018231301519518578;)
            f64.add
            local.tee 6
            f64.store
            local.get 0
            local.get 1
            local.get 6
            f64.sub
            f64.const -0x1.90e91a79394cap-33 (;=-0.00000000018231301519518578;)
            f64.add
            f64.store offset=16
            br 3 (;@1;)
          end
          local.get 0
          i32.const -3
          i32.store offset=8
          local.get 0
          local.get 1
          f64.const 0x1.2d97c7f3p+2 (;=4.712388980202377;)
          f64.add
          local.tee 1
          f64.const 0x1.90e91a79394cap-33 (;=0.00000000018231301519518578;)
          f64.add
          local.tee 6
          f64.store
          local.get 0
          local.get 1
          local.get 6
          f64.sub
          f64.const 0x1.90e91a79394cap-33 (;=0.00000000018231301519518578;)
          f64.add
          f64.store offset=16
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 4
          i32.const 1048575
          i32.and
          i32.const 598523
          i32.eq
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.const 1073928573
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 3
              i64.const -1
              i64.le_s
              br_if 0 (;@5;)
              local.get 0
              i32.const 2
              i32.store offset=8
              local.get 0
              local.get 1
              f64.const -0x1.921fb544p+1 (;=-3.1415926534682512;)
              f64.add
              local.tee 1
              f64.const -0x1.0b4611a626331p-33 (;=-0.00000000012154201013012384;)
              f64.add
              local.tee 6
              f64.store
              local.get 0
              local.get 1
              local.get 6
              f64.sub
              f64.const -0x1.0b4611a626331p-33 (;=-0.00000000012154201013012384;)
              f64.add
              f64.store offset=16
              br 4 (;@1;)
            end
            local.get 0
            i32.const -2
            i32.store offset=8
            local.get 0
            local.get 1
            f64.const 0x1.921fb544p+1 (;=3.1415926534682512;)
            f64.add
            local.tee 1
            f64.const 0x1.0b4611a626331p-33 (;=0.00000000012154201013012384;)
            f64.add
            local.tee 6
            f64.store
            local.get 0
            local.get 1
            local.get 6
            f64.sub
            f64.const 0x1.0b4611a626331p-33 (;=0.00000000012154201013012384;)
            f64.add
            f64.store offset=16
            br 3 (;@1;)
          end
          block ;; label = @4
            local.get 3
            i64.const -1
            i64.gt_s
            br_if 0 (;@4;)
            local.get 0
            i32.const -1
            i32.store offset=8
            local.get 0
            local.get 1
            f64.const 0x1.921fb544p+0 (;=1.5707963267341256;)
            f64.add
            local.tee 1
            f64.const 0x1.0b4611a626331p-34 (;=0.00000000006077100506506192;)
            f64.add
            local.tee 6
            f64.store
            local.get 0
            local.get 1
            local.get 6
            f64.sub
            f64.const 0x1.0b4611a626331p-34 (;=0.00000000006077100506506192;)
            f64.add
            f64.store offset=16
            br 3 (;@1;)
          end
          local.get 0
          i32.const 1
          i32.store offset=8
          local.get 0
          local.get 1
          f64.const -0x1.921fb544p+0 (;=-1.5707963267341256;)
          f64.add
          local.tee 1
          f64.const -0x1.0b4611a626331p-34 (;=-0.00000000006077100506506192;)
          f64.add
          local.tee 6
          f64.store
          local.get 0
          local.get 1
          local.get 6
          f64.sub
          f64.const -0x1.0b4611a626331p-34 (;=-0.00000000006077100506506192;)
          f64.add
          f64.store offset=16
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 5
          i32.const 20
          i32.shr_u
          local.tee 5
          local.get 1
          local.get 1
          f64.const 0x1.45f306dc9c883p-1 (;=0.6366197723675814;)
          f64.mul
          f64.const 0x1.8p+52 (;=6755399441055744;)
          f64.add
          f64.const -0x1.8p+52 (;=-6755399441055744;)
          f64.add
          local.tee 6
          f64.const -0x1.921fb544p+0 (;=-1.5707963267341256;)
          f64.mul
          f64.add
          local.tee 1
          local.get 6
          f64.const 0x1.0b4611a626331p-34 (;=0.00000000006077100506506192;)
          f64.mul
          local.tee 7
          f64.sub
          local.tee 8
          i64.reinterpret_f64
          i64.const 52
          i64.shr_u
          i32.wrap_i64
          i32.const 2047
          i32.and
          i32.sub
          i32.const 17
          i32.lt_s
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            local.get 1
            local.get 6
            f64.const 0x1.0b4611a6p-34 (;=0.00000000006077100506303966;)
            f64.mul
            local.tee 8
            f64.sub
            local.tee 9
            local.get 6
            f64.const 0x1.3198a2e037073p-69 (;=0.0000000000000000000020222662487959506;)
            f64.mul
            local.get 1
            local.get 9
            f64.sub
            local.get 8
            f64.sub
            f64.sub
            local.tee 7
            f64.sub
            local.tee 8
            i64.reinterpret_f64
            i64.const 52
            i64.shr_u
            i32.wrap_i64
            i32.const 2047
            i32.and
            i32.sub
            i32.const 50
            i32.ge_s
            br_if 0 (;@4;)
            local.get 9
            local.set 1
            br 1 (;@3;)
          end
          local.get 9
          local.get 6
          f64.const 0x1.3198a2ep-69 (;=0.0000000000000000000020222662487111665;)
          f64.mul
          local.tee 8
          f64.sub
          local.tee 1
          local.get 6
          f64.const 0x1.b839a252049c1p-104 (;=0.000000000000000000000000000000084784276603689;)
          f64.mul
          local.get 9
          local.get 1
          f64.sub
          local.get 8
          f64.sub
          f64.sub
          local.tee 7
          f64.sub
          local.set 8
        end
        local.get 0
        local.get 8
        f64.store
        local.get 0
        local.get 6
        i32.trunc_sat_f64_s
        i32.store offset=8
        local.get 0
        local.get 1
        local.get 8
        f64.sub
        local.get 7
        f64.sub
        f64.store offset=16
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 1
        local.get 1
        f64.const 0x1.45f306dc9c883p-1 (;=0.6366197723675814;)
        f64.mul
        f64.const 0x1.8p+52 (;=6755399441055744;)
        f64.add
        f64.const -0x1.8p+52 (;=-6755399441055744;)
        f64.add
        local.tee 6
        f64.const -0x1.921fb544p+0 (;=-1.5707963267341256;)
        f64.mul
        f64.add
        local.tee 1
        local.get 6
        f64.const 0x1.0b4611a626331p-34 (;=0.00000000006077100506506192;)
        f64.mul
        local.tee 7
        f64.sub
        local.tee 8
        i64.reinterpret_f64
        i64.const 9218868437227405312
        i64.and
        i64.const 4544132024016830463
        i64.gt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 1
          local.get 6
          f64.const 0x1.0b4611a6p-34 (;=0.00000000006077100506303966;)
          f64.mul
          local.tee 8
          f64.sub
          local.tee 9
          local.get 6
          f64.const 0x1.3198a2e037073p-69 (;=0.0000000000000000000020222662487959506;)
          f64.mul
          local.get 1
          local.get 9
          f64.sub
          local.get 8
          f64.sub
          f64.sub
          local.tee 7
          f64.sub
          local.tee 8
          i64.reinterpret_f64
          i64.const 9151314442816847872
          i64.and
          i64.const 4395513236313604095
          i64.le_u
          br_if 0 (;@3;)
          local.get 9
          local.set 1
          br 1 (;@2;)
        end
        local.get 9
        local.get 6
        f64.const 0x1.3198a2ep-69 (;=0.0000000000000000000020222662487111665;)
        f64.mul
        local.tee 8
        f64.sub
        local.tee 1
        local.get 6
        f64.const 0x1.b839a252049c1p-104 (;=0.000000000000000000000000000000084784276603689;)
        f64.mul
        local.get 9
        local.get 1
        f64.sub
        local.get 8
        f64.sub
        f64.sub
        local.tee 7
        f64.sub
        local.set 8
      end
      local.get 0
      local.get 8
      f64.store
      local.get 0
      local.get 6
      i32.trunc_sat_f64_s
      i32.store offset=8
      local.get 0
      local.get 1
      local.get 8
      f64.sub
      local.get 7
      f64.sub
      f64.store offset=16
    end
    local.get 2
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )

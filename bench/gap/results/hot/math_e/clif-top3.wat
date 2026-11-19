(func $_RINvNtNtNtCseievaMsdffa_4libm4math7generic4sqrt10sqrt_rounddEB8_ (;39;) (type 12) (param i32 f64 i32)
    (local i64 i32 i32 f64 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 192
    i32.sub
    local.tee 9
    global.set $__stack_pointer
    local.get 2
    i32.const 255
    i32.and
    local.set 2
    local.get 9
    i32.const 0
    i32.add
    local.get 2
    i32.store8
    local.get 9
    i32.const 8
    i32.add
    local.tee 2
    i64.const 0
    i64.store align=1
    local.get 1
    call $"#func49 _RNvXs_NtNtNtCseievaMsdffa_4libm4math7support12float_traitsdNtB4_5Float7to_bits"
    local.set 3
    local.get 9
    i32.const 16
    i32.add
    local.tee 4
    local.get 3
    i64.store align=1
    local.get 3
    i32.const 52
    i64.extend_i32_u
    i64.shr_u
    i32.const 1066692
    call $_RNvXs2h_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyINtB6_8CastIntomE4cast
    local.set 5
    local.get 9
    i32.const 44
    i32.add
    local.get 5
    i32.store align=1
    local.get 9
    i32.const 40
    i32.add
    i32.const 0
    i32.store align=1
    local.get 5
    i32.const 1
    i32.sub
    i32.const 2046
    i32.ge_u
    local.set 5
    block ;; label = @1
      block ;; label = @2
        local.get 5
        if ;; label = @3
          local.get 4
          i64.load align=1
          local.set 3
          local.get 9
          i32.const 48
          i32.add
          local.tee 5
          local.get 3
          i32.const 1
          i64.extend_i32_u
          i64.shl
          i64.store align=1
          local.get 5
          local.get 2
          call $"#func54 _RNvXsn_NtNtCsk7Mwp4gGzxp_4core3cmp5implsyNtB7_9PartialEq2eq"
          i32.const 255
          i32.and
          local.set 5
          block ;; label = @4
            local.get 5
            if ;; label = @5
              local.get 0
              local.get 1
              f64.store align=1
              local.get 9
              i32.const 136
              i32.add
              i32.const 0
              i32.store8
              local.get 0
              i32.const 0
              i32.store8 offset=8
              br 1 (;@4;)
            else
              local.get 9
              i32.const 56
              i32.add
              local.tee 5
              i64.const 9218868437227405312
              i64.store align=1
              local.get 4
              local.get 5
              call $"#func54 _RNvXsn_NtNtCsk7Mwp4gGzxp_4core3cmp5implsyNtB7_9PartialEq2eq"
              i32.const 255
              i32.and
              local.tee 5
              if ;; label = @6
                local.get 0
                local.get 1
                f64.store align=1
                local.get 9
                i32.const 137
                i32.add
                i32.const 0
                i32.store8
                local.get 0
                i32.const 0
                i32.store8 offset=8
                br 2 (;@4;)
              else
                local.get 9
                i32.const 64
                i32.add
                local.tee 5
                i64.const 9218868437227405312
                i64.store align=1
                local.get 4
                local.get 5
                call $_RNvXs12_NtNtCsk7Mwp4gGzxp_4core3cmp5implsyNtB8_10PartialOrd2gt
                i32.const 255
                i32.and
                local.tee 5
                if ;; label = @7
                  local.get 0
                  f64.const nan (;=NaN;)
                  f64.store align=1
                  local.get 9
                  i32.const 138
                  i32.add
                  i32.const 1
                  i32.store8
                  local.get 0
                  i32.const 1
                  i32.store8 offset=8
                  br 3 (;@4;)
                else
                  i32.const 0
                  i32.const 1075
                  local.get 2
                  i64.load align=1
                  call $_RNvYdNtNtNtNtCseievaMsdffa_4libm4math7support12float_traits5Float10from_partsBb_
                  local.set 6
                  local.get 1
                  local.get 6
                  f64.mul
                  local.tee 6
                  call $"#func49 _RNvXs_NtNtNtCseievaMsdffa_4libm4math7support12float_traitsdNtB4_5Float7to_bits"
                  local.set 3
                  block ;; label = @8
                    local.get 4
                    local.get 3
                    i64.store align=1
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 9
                          i32.const 40
                          i32.add
                          i32.load align=1
                          br_table 1 (;@10;) 2 (;@9;) 0 (;@11;)
                        end
                        br 9 (;@1;)
                      end
                      local.get 6
                      call $_RNvYdNtNtNtNtCseievaMsdffa_4libm4math7support12float_traits5Float2exBb_
                      local.set 5
                      local.get 9
                      i32.const 44
                      i32.add
                      local.tee 8
                      local.get 5
                      i32.store align=1
                      local.get 8
                      local.get 5
                      i32.const 52
                      i32.sub
                      i32.store align=1
                      br 1 (;@8;)
                    end
                    i32.const 54525952
                    i32.const 1066676
                    call $"#func44 _RNvXs1V_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsmINtB6_8CastIntoyE4cast"
                    local.set 7
                    local.get 4
                    local.get 3
                    local.get 7
                    i64.sub
                    i64.store align=1
                    br 0 (;@8;)
                  end
                  br 5 (;@2;)
                end
              end
            end
          end
          local.get 9
          i32.const 192
          i32.add
          global.set $__stack_pointer
          return
        else
          br 1 (;@2;)
        end
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 9
              i32.const 40
              i32.add
              i32.load align=1
              br_table 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            br 3 (;@1;)
          end
          local.get 9
          i32.const 44
          i32.add
          i32.load align=1
          local.set 5
          local.get 4
          i64.load align=1
          local.set 7
          local.get 9
          i32.const 72
          i32.add
          local.tee 8
          local.get 7
          i64.const 4503599627370496
          i64.or
          i32.const 11
          i64.extend_i32_u
          i64.shl
          i64.store align=1
          block ;; label = @4
            local.get 5
            i32.const 1
            i32.and
            if ;; label = @5
              local.get 8
              i32.const 1
              i32.const 1066660
              call $_RNvXssP_NtNtCsk7Mwp4gGzxp_4core3ops3bityINtB6_9ShrAssignlE10shr_assign
              br 1 (;@4;)
            else
              br 1 (;@4;)
            end
          end
          local.get 8
          i64.load align=1
          local.set 3
          local.get 9
          i32.const 100
          i32.add
          local.get 5
          i32.const 1023
          i32.add
          i32.const 1
          i32.shr_u
          i32.store align=1
          local.get 9
          i32.const 96
          i32.add
          i32.const 0
          i32.store align=1
          br 1 (;@2;)
        end
        local.get 4
        i64.load align=1
        local.set 7
        local.get 9
        i32.const 80
        i32.add
        local.tee 8
        local.get 7
        i64.const 4503599627370496
        i64.and
        i64.store align=1
        local.get 8
        local.get 2
        call $"#func55 _RNvXsn_NtNtCsk7Mwp4gGzxp_4core3cmp5implsyNtB7_9PartialEq2ne"
        i32.const 255
        i32.and
        local.set 8
        local.get 4
        i64.load align=1
        local.set 7
        local.get 9
        i32.const 88
        i32.add
        local.tee 5
        local.get 7
        i32.const 1
        i64.extend_i32_u
        i64.shr_u
        i64.store align=1
        local.get 5
        i64.const 2303591209400008704
        i32.const 1066644
        call $"#func47 _RNvXs5w_NtNtCsk7Mwp4gGzxp_4core3ops5arithyNtB6_9AddAssign10add_assign"
        local.get 5
        i64.const 9218868437227405312
        call $"#func51 _RNvXslA_NtNtCsk7Mwp4gGzxp_4core3ops3bityNtB6_12BitAndAssign13bitand_assign"
        local.get 4
        i64.load align=1
        local.set 7
        block ;; label = @3
          local.get 8
          if ;; label = @4
            local.get 7
            i32.const 10
            i64.extend_i32_u
            i64.shl
            i64.const 9223372036854775807
            i64.and
            local.set 3
            br 1 (;@3;)
          else
            local.get 7
            i32.const 11
            i64.extend_i32_u
            i64.shl
            i64.const -9223372036854775808
            i64.or
            local.set 3
            br 1 (;@3;)
          end
        end
        local.get 9
        i32.const 104
        i32.add
        local.get 5
        i64.load align=1
        i64.store align=1
        local.get 9
        i32.const 96
        i32.add
        i32.const 1
        i32.store align=1
        br 0 (;@2;)
      end
      local.get 4
      i64.load align=1
      local.tee 7
      i32.const 46
      i64.extend_i32_u
      i64.shr_u
      i32.const 1066628
      call $_RNvXs2b_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyINtB6_8CastIntojE4cast
      local.tee 4
      i32.const 127
      i32.and
      i32.const 128
      i32.lt_u
      if ;; label = @2
        i32.const 1066160
        local.get 4
        i32.const 127
        i32.and
        i32.const 1
        i32.shl
        i32.add
        i32.load16_u align=1
        i32.const 65535
        i32.and
        i32.const 1066612
        call $_RNvXNtNtNtCseievaMsdffa_4libm4math7support10int_traitsmINtB2_8CastFromtE9cast_fromB8_
        local.set 4
        local.get 3
        i32.const 32
        i64.extend_i32_u
        i64.shr_u
        local.tee 7
        i32.const 1066596
        call $_RNvXs2h_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyINtB6_8CastIntomE4cast
        local.set 5
        local.get 9
        i32.const 176
        i32.add
        local.tee 8
        local.get 4
        i32.const 16
        i32.shl
        local.get 5
        i32.const 0
        i32.const 0
        call $_RINvNtNtNtCseievaMsdffa_4libm4math7generic4sqrt11goldschmidtdmEB8_
        local.get 8
        i32.load
        local.set 8
        local.get 7
        i32.const 1066580
        call $_RNvXs2h_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyINtB6_8CastIntomE4cast
        local.set 5
        local.get 9
        i32.const 176
        i32.add
        local.tee 4
        local.get 8
        local.get 5
        i32.const 2
        i32.const 0
        call $_RINvNtNtNtCseievaMsdffa_4libm4math7generic4sqrt11goldschmidtdmEB8_
        local.get 4
        i32.load
        local.set 4
        local.get 9
        i32.const 144
        i32.add
        local.get 4
        i64.extend_i32_u
        i32.const 32
        i64.extend_i32_u
        i64.shl
        local.get 3
        i32.const 2
        i32.const 1
        call $_RINvNtNtNtCseievaMsdffa_4libm4math7generic4sqrt11goldschmidtdyEB8_
        local.get 9
        i32.const 152
        i32.add
        i64.load align=1
        local.tee 7
        i32.const 9
        i64.extend_i32_u
        i64.shr_u
        local.set 7
        local.get 9
        i32.const 112
        i32.add
        local.tee 4
        local.get 7
        i64.store align=1
        local.get 7
        local.get 3
        i32.const 42
        i64.extend_i32_u
        i64.shl
        local.get 7
        local.get 7
        i64.mul
        i64.sub
        i64.sub
        local.set 7
        local.get 4
        local.get 7
        i32.const 63
        i64.extend_i32_u
        i64.shr_u
        i32.const 1066564
        call $"#func47 _RNvXs5w_NtNtCsk7Mwp4gGzxp_4core3ops5arithyNtB6_9AddAssign10add_assign"
        local.get 4
        i64.const 4503599627370495
        call $"#func51 _RNvXslA_NtNtCsk7Mwp4gGzxp_4core3ops3bityNtB6_12BitAndAssign13bitand_assign"
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 9
                i32.const 96
                i32.add
                i32.load align=1
                br_table 1 (;@5;) 2 (;@4;) 0 (;@6;)
              end
              br 4 (;@1;)
            end
            local.get 9
            i32.const 100
            i32.add
            i32.load align=1
            i32.const 1066548
            call $_RNvXNtNtNtCseievaMsdffa_4libm4math7support10int_traitsyINtB2_8CastFrommE9cast_fromB8_
            local.set 3
            local.get 4
            local.get 3
            i32.const 52
            i64.extend_i32_u
            i64.shl
            call $_RNvXsm0_NtNtCsk7Mwp4gGzxp_4core3ops3bityNtB6_11BitOrAssign12bitor_assign
            br 1 (;@3;)
          end
          local.get 4
          local.get 9
          i32.const 104
          i32.add
          i64.load align=1
          call $_RNvXsm0_NtNtCsk7Mwp4gGzxp_4core3ops3bityNtB6_11BitOrAssign12bitor_assign
          br 0 (;@3;)
        end
        local.get 4
        i64.load align=1
        call $"#func50 _RNvXs_NtNtNtCseievaMsdffa_4libm4math7support12float_traitsdNtB4_5Float9from_bits"
        local.set 6
        local.get 4
        i64.load align=1
        local.set 3
        local.get 9
        i32.const 120
        i32.add
        local.tee 4
        local.get 7
        local.get 3
        i64.add
        i64.const 1
        i64.add
        i64.store align=1
        local.get 4
        local.get 2
        call $"#func54 _RNvXsn_NtNtCsk7Mwp4gGzxp_4core3cmp5implsyNtB7_9PartialEq2eq"
        i32.const 255
        i32.and
        local.set 8
        block ;; label = @3
          local.get 8
          if ;; label = @4
            local.get 9
            i32.const 128
            i32.add
            local.get 2
            i64.load align=1
            i64.store align=1
            br 1 (;@3;)
          else
            local.get 9
            i32.const 128
            i32.add
            i64.const 4503599627370496
            i64.store align=1
            br 1 (;@3;)
          end
        end
        local.get 4
        i64.load align=1
        local.set 3
        local.get 9
        i32.const 128
        i32.add
        local.tee 4
        local.get 7
        local.get 3
        i64.xor
        i64.const -9223372036854775808
        i64.and
        call $_RNvXsm0_NtNtCsk7Mwp4gGzxp_4core3ops3bityNtB6_11BitOrAssign12bitor_assign
        local.get 4
        i64.load align=1
        call $"#func50 _RNvXs_NtNtNtCseievaMsdffa_4libm4math7support12float_traitsdNtB4_5Float9from_bits"
        local.set 1
        local.get 6
        local.get 1
        f64.add
        local.set 6
        local.get 0
        local.get 6
        f64.store align=1
        local.get 9
        i32.const 160
        i32.add
        i32.const 0
        i32.store8
        local.get 0
        i32.const 0
        i32.store8 offset=8
        local.get 9
        i32.const 192
        i32.add
        global.set $__stack_pointer
        return
      else
        local.get 4
        i32.const 127
        i32.and
        i32.const 128
        i32.const 1066416
        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
        unreachable
      end
    end
    unreachable
    unreachable
  )

(func $_RINvYINtNtNtCs8euEcNjxCEf_4core3str4iter5SplitNtB8_12IsWhitespaceENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvB12_4find5checkReQNtB8_10IsNotEmptyE0INtNtNtBa_3ops12control_flow11ControlFlowB2d_EECs9hsE6Fh6sWE_7strings (;2;) (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.load8_u offset=29
    local.set 2
    local.get 1
    i32.load offset=24
    local.set 3
    local.get 1
    i32.load offset=16
    local.set 4
    local.get 1
    i32.load offset=4
    local.set 5
    local.get 1
    i32.load8_u offset=28
    local.set 6
    local.get 1
    i32.load offset=20
    local.set 7
    local.get 1
    i32.load offset=8
    local.set 8
    local.get 1
    i32.load
    local.set 9
    block ;; label = @1
      loop ;; label = @2
        local.get 9
        local.set 10
        i32.const 0
        local.set 11
        block ;; label = @3
          local.get 2
          i32.const 1
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
        block ;; label = @3
          block ;; label = @4
            local.get 4
            local.get 7
            i32.eq
            br_if 0 (;@4;)
            loop ;; label = @5
              local.get 3
              local.set 12
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  local.tee 3
                  i32.load8_s
                  local.tee 2
                  i32.const -1
                  i32.le_s
                  br_if 0 (;@7;)
                  local.get 3
                  i32.const 1
                  i32.add
                  local.set 4
                  local.get 2
                  i32.const 255
                  i32.and
                  local.set 2
                  br 1 (;@6;)
                end
                local.get 3
                i32.load8_u offset=1
                i32.const 63
                i32.and
                local.set 4
                local.get 2
                i32.const 31
                i32.and
                local.set 9
                block ;; label = @7
                  local.get 2
                  i32.const -33
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 9
                  i32.const 6
                  i32.shl
                  local.get 4
                  i32.or
                  local.set 2
                  local.get 3
                  i32.const 2
                  i32.add
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 4
                i32.const 6
                i32.shl
                local.get 3
                i32.load8_u offset=2
                i32.const 63
                i32.and
                i32.or
                local.set 4
                block ;; label = @7
                  local.get 2
                  i32.const -16
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 9
                  i32.const 12
                  i32.shl
                  i32.or
                  local.set 2
                  local.get 3
                  i32.const 3
                  i32.add
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 4
                i32.const 6
                i32.shl
                local.get 3
                i32.load8_u offset=3
                i32.const 63
                i32.and
                i32.or
                local.get 9
                i32.const 18
                i32.shl
                i32.const 1835008
                i32.and
                i32.or
                local.set 2
                local.get 3
                i32.const 4
                i32.add
                local.set 4
              end
              local.get 4
              local.get 3
              i32.sub
              local.get 12
              i32.add
              local.set 3
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.const -9
                    i32.add
                    local.tee 9
                    i32.const 23
                    i32.gt_u
                    br_if 0 (;@8;)
                    i32.const 1
                    local.get 9
                    i32.shl
                    i32.const 8388639
                    i32.and
                    br_if 1 (;@7;)
                  end
                  local.get 2
                  i32.const 133
                  i32.lt_u
                  br_if 1 (;@6;)
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 2
                          i32.const 8
                          i32.shr_u
                          local.tee 9
                          i32.const -22
                          i32.add
                          br_table 3 (;@8;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 1 (;@10;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 5 (;@6;) 0 (;@11;) 2 (;@9;)
                        end
                        local.get 2
                        i32.const 12288
                        i32.ne
                        br_if 4 (;@6;)
                        br 3 (;@7;)
                      end
                      local.get 2
                      i32.const 255
                      i32.and
                      i32.load8_u offset=1049524
                      i32.const 2
                      i32.and
                      i32.eqz
                      br_if 3 (;@6;)
                      br 2 (;@7;)
                    end
                    local.get 9
                    br_if 2 (;@6;)
                    local.get 2
                    i32.const 255
                    i32.and
                    i32.load8_u offset=1049524
                    i32.const 1
                    i32.and
                    br_if 1 (;@7;)
                    br 2 (;@6;)
                  end
                  local.get 2
                  i32.const 5760
                  i32.ne
                  br_if 1 (;@6;)
                end
                local.get 1
                local.get 3
                i32.store offset=24
                local.get 1
                local.get 4
                i32.store offset=16
                local.get 1
                local.get 3
                i32.store
                i32.const 0
                local.set 2
                local.get 3
                local.set 9
                br 3 (;@3;)
              end
              local.get 4
              local.get 7
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 1
            local.get 3
            i32.store offset=24
            local.get 1
            local.get 4
            i32.store offset=16
          end
          i32.const 1
          local.set 2
          local.get 1
          i32.const 1
          i32.store8 offset=29
          block ;; label = @4
            local.get 6
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 10
            local.set 9
            local.get 5
            local.set 12
            br 1 (;@3;)
          end
          local.get 10
          local.set 9
          local.get 5
          local.set 12
          local.get 5
          local.get 10
          i32.eq
          br_if 2 (;@1;)
        end
        local.get 12
        local.get 10
        i32.sub
        local.tee 13
        i32.eqz
        br_if 0 (;@2;)
      end
      local.get 8
      local.get 10
      i32.add
      local.set 11
    end
    local.get 0
    local.get 13
    i32.store offset=4
    local.get 0
    local.get 11
    i32.store
  )

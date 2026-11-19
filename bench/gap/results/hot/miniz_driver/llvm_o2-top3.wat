(func $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7inflate4core15decompress_fast (;20;) (type 14) (param i32 i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 5
    i32.load8_u offset=16
    local.set 7
    local.get 5
    i32.load offset=12
    local.set 8
    local.get 5
    i32.load offset=8
    local.set 9
    local.get 5
    i32.load offset=4
    local.set 10
    local.get 5
    i32.load
    local.set 11
    i32.const 12
    local.set 12
    i32.const 0
    local.set 13
    block ;; label = @1
      local.get 3
      i32.load offset=12
      local.tee 14
      local.get 3
      i32.load offset=8
      local.tee 15
      i32.sub
      i32.const 259
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=4
      local.tee 16
      i32.const 14
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 5760
      i32.add
      local.set 17
      local.get 1
      i32.const 3712
      i32.add
      local.set 18
      local.get 1
      i32.const 2560
      i32.add
      local.set 19
      local.get 1
      i32.const 512
      i32.add
      local.set 20
      local.get 3
      i32.load
      local.set 21
      local.get 3
      i32.load offset=4
      local.set 22
      local.get 2
      i32.load
      local.set 23
      local.get 4
      i32.const 4
      i32.and
      i32.const 0
      i32.ne
      local.set 24
      loop ;; label = @2
        local.get 23
        local.set 8
        block ;; label = @3
          block ;; label = @4
            loop ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 10
                  i32.const 14
                  i32.le_u
                  br_if 0 (;@7;)
                  local.get 10
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 2
                local.get 16
                i32.const -2
                i32.add
                local.tee 16
                i32.store offset=4
                local.get 2
                local.get 8
                i32.const 2
                i32.add
                local.tee 23
                i32.store
                local.get 10
                i32.const 16
                i32.or
                local.set 4
                local.get 8
                i32.load16_u align=1
                local.get 10
                i32.shl
                local.get 11
                i32.or
                local.set 11
                local.get 23
                local.set 8
              end
              block ;; label = @6
                block ;; label = @7
                  local.get 20
                  local.get 11
                  i32.const 1023
                  i32.and
                  i32.const 1
                  i32.shl
                  i32.add
                  i32.load16_s
                  local.tee 1
                  i32.const -1
                  i32.gt_s
                  br_if 0 (;@7;)
                  i32.const 10
                  local.set 10
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 11
                      local.get 10
                      i32.shr_u
                      i32.const 1
                      i32.and
                      local.get 1
                      i32.const -1
                      i32.xor
                      i32.add
                      local.tee 1
                      i32.const 576
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 32767
                      local.set 1
                      local.get 10
                      i32.const 1
                      i32.add
                      i32.const 255
                      i32.and
                      local.set 13
                      br 3 (;@6;)
                    end
                    local.get 10
                    i32.const 1
                    i32.add
                    local.set 10
                    local.get 19
                    local.get 1
                    i32.const 1
                    i32.shl
                    i32.add
                    i32.load16_s
                    local.tee 1
                    i32.const 0
                    i32.lt_s
                    br_if 0 (;@8;)
                  end
                  local.get 10
                  i32.const 255
                  i32.and
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 1
                i32.const 9
                i32.shr_u
                local.set 13
              end
              local.get 4
              local.get 13
              i32.sub
              local.set 10
              local.get 11
              local.get 13
              i32.shr_u
              local.set 11
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.const 256
                    i32.and
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 10
                        i32.const 14
                        i32.le_u
                        br_if 0 (;@10;)
                        local.get 10
                        local.set 25
                        br 1 (;@9;)
                      end
                      local.get 2
                      local.get 16
                      i32.const -2
                      i32.add
                      local.tee 16
                      i32.store offset=4
                      local.get 2
                      local.get 8
                      i32.const 2
                      i32.add
                      local.tee 23
                      i32.store
                      local.get 10
                      i32.const 16
                      i32.or
                      local.set 25
                      local.get 8
                      i32.load16_u align=1
                      local.get 10
                      i32.shl
                      local.get 11
                      i32.or
                      local.set 11
                      local.get 23
                      local.set 8
                    end
                    block ;; label = @9
                      block ;; label = @10
                        local.get 20
                        local.get 11
                        i32.const 1023
                        i32.and
                        i32.const 1
                        i32.shl
                        i32.add
                        i32.load16_s
                        local.tee 13
                        i32.const -1
                        i32.gt_s
                        br_if 0 (;@10;)
                        i32.const 10
                        local.set 10
                        loop ;; label = @11
                          block ;; label = @12
                            local.get 11
                            local.get 10
                            i32.shr_u
                            i32.const 1
                            i32.and
                            local.get 13
                            i32.const -1
                            i32.xor
                            i32.add
                            local.tee 13
                            i32.const 576
                            i32.lt_u
                            br_if 0 (;@12;)
                            i32.const 32767
                            local.set 13
                            local.get 10
                            i32.const 1
                            i32.add
                            i32.const 255
                            i32.and
                            local.set 4
                            br 3 (;@9;)
                          end
                          local.get 10
                          i32.const 1
                          i32.add
                          local.set 10
                          local.get 19
                          local.get 13
                          i32.const 1
                          i32.shl
                          i32.add
                          i32.load16_s
                          local.tee 13
                          i32.const 0
                          i32.lt_s
                          br_if 0 (;@11;)
                        end
                        local.get 10
                        i32.const 255
                        i32.and
                        local.set 4
                        br 1 (;@9;)
                      end
                      local.get 13
                      i32.const 9
                      i32.shr_u
                      local.set 4
                    end
                    local.get 15
                    local.get 22
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 25
                    local.get 4
                    i32.sub
                    local.set 10
                    local.get 11
                    local.get 4
                    i32.shr_u
                    local.set 11
                    local.get 3
                    local.get 15
                    i32.const 1
                    i32.add
                    local.tee 4
                    i32.store offset=8
                    local.get 21
                    local.get 15
                    i32.add
                    local.get 1
                    i32.store8
                    local.get 13
                    i32.const 256
                    i32.and
                    i32.eqz
                    br_if 2 (;@6;)
                    local.get 4
                    local.set 15
                    local.get 13
                    local.set 1
                  end
                  i32.const 256
                  local.set 8
                  i32.const 0
                  local.set 13
                  local.get 1
                  i32.const 511
                  i32.and
                  local.tee 4
                  i32.const 256
                  i32.ne
                  br_if 4 (;@3;)
                  i32.const 20
                  local.set 12
                  br 6 (;@1;)
                end
                local.get 15
                local.get 22
                i32.const 1051924
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 4
              local.get 22
              i32.ge_u
              br_if 1 (;@4;)
              local.get 3
              local.get 15
              i32.const 2
              i32.add
              local.tee 15
              i32.store offset=8
              local.get 21
              local.get 4
              i32.add
              local.get 13
              i32.store8
              i32.const 0
              local.set 13
              block ;; label = @6
                local.get 14
                local.get 15
                i32.sub
                i32.const 259
                i32.ge_u
                br_if 0 (;@6;)
                local.get 1
                local.set 8
                br 5 (;@1;)
              end
              local.get 16
              i32.const 14
              i32.ge_u
              br_if 0 (;@5;)
            end
            local.get 1
            local.set 8
            br 3 (;@1;)
          end
          local.get 4
          local.get 22
          i32.const 1051924
          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
          unreachable
        end
        block ;; label = @3
          local.get 4
          i32.const 285
          i32.le_u
          br_if 0 (;@3;)
          i32.const 33
          local.set 12
          i32.const 255
          local.set 13
          local.get 4
          local.set 8
          br 2 (;@1;)
        end
        local.get 1
        i32.const -1
        i32.add
        i32.const 31
        i32.and
        local.tee 1
        i32.const 1
        i32.shl
        i32.const 1051988
        i32.add
        local.set 8
        block ;; label = @3
          block ;; label = @4
            local.get 10
            i32.const 14
            i32.le_u
            br_if 0 (;@4;)
            local.get 23
            local.set 25
            local.get 10
            local.set 4
            br 1 (;@3;)
          end
          local.get 2
          local.get 16
          i32.const -2
          i32.add
          local.tee 16
          i32.store offset=4
          local.get 2
          local.get 23
          i32.const 2
          i32.add
          local.tee 25
          i32.store
          local.get 10
          i32.const 16
          i32.or
          local.set 4
          local.get 23
          i32.load16_u align=1
          local.get 10
          i32.shl
          local.get 11
          i32.or
          local.set 11
        end
        local.get 1
        i32.load8_u offset=1051956
        local.set 7
        local.get 8
        i32.load16_u
        local.set 8
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const -28
            i32.add
            i32.const -20
            i32.ge_u
            br_if 0 (;@4;)
            local.get 25
            local.set 26
            br 1 (;@3;)
          end
          local.get 11
          local.get 7
          i32.shr_u
          local.set 10
          local.get 11
          i32.const -1
          local.get 7
          i32.shl
          i32.const -1
          i32.xor
          i32.and
          local.get 8
          i32.add
          local.set 8
          block ;; label = @4
            local.get 4
            local.get 7
            i32.sub
            local.tee 11
            i32.const 14
            i32.le_u
            br_if 0 (;@4;)
            local.get 25
            local.set 26
            local.get 11
            local.set 4
            local.get 10
            local.set 11
            br 1 (;@3;)
          end
          local.get 2
          local.get 16
          i32.const -2
          i32.add
          local.tee 16
          i32.store offset=4
          local.get 2
          local.get 25
          i32.const 2
          i32.add
          local.tee 26
          i32.store
          local.get 11
          i32.const 16
          i32.or
          local.set 4
          local.get 25
          i32.load16_u align=1
          local.get 11
          i32.shl
          local.get 10
          i32.or
          local.set 11
        end
        block ;; label = @3
          block ;; label = @4
            local.get 18
            local.get 11
            i32.const 1023
            i32.and
            i32.const 1
            i32.shl
            i32.add
            i32.load16_s
            local.tee 1
            i32.const -1
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 10
            local.set 10
            loop ;; label = @5
              block ;; label = @6
                local.get 11
                local.get 10
                i32.shr_u
                i32.const 1
                i32.and
                local.get 1
                i32.const -1
                i32.xor
                i32.add
                local.tee 1
                i32.const 576
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 32767
                local.set 1
                local.get 10
                i32.const 1
                i32.add
                i32.const 255
                i32.and
                local.set 23
                br 3 (;@3;)
              end
              local.get 10
              i32.const 1
              i32.add
              local.set 10
              local.get 17
              local.get 1
              i32.const 1
              i32.shl
              i32.add
              i32.load16_s
              local.tee 1
              i32.const 0
              i32.lt_s
              br_if 0 (;@5;)
            end
            local.get 10
            i32.const 255
            i32.and
            local.set 23
            br 1 (;@3;)
          end
          local.get 1
          i32.const 9
          i32.shr_u
          local.set 23
        end
        local.get 4
        local.get 23
        i32.sub
        local.set 10
        local.get 11
        local.get 23
        i32.shr_u
        local.set 11
        block ;; label = @3
          local.get 1
          i32.const 511
          i32.and
          local.tee 4
          i32.const 29
          i32.le_u
          br_if 0 (;@3;)
          i32.const 34
          local.set 12
          i32.const 255
          local.set 13
          br 2 (;@1;)
        end
        local.get 1
        i32.const 255
        i32.and
        local.tee 1
        i32.const 1
        i32.shr_u
        local.tee 23
        local.get 23
        i32.const 0
        i32.ne
        i32.sub
        local.set 7
        local.get 4
        i32.const 1
        i32.shl
        i32.load16_u offset=1051864
        local.set 9
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 4
            i32.ge_u
            br_if 0 (;@4;)
            local.get 26
            local.set 23
            br 1 (;@3;)
          end
          block ;; label = @4
            block ;; label = @5
              local.get 10
              i32.const 15
              i32.lt_u
              br_if 0 (;@5;)
              local.get 26
              local.set 23
              local.get 10
              local.set 25
              local.get 11
              local.set 1
              br 1 (;@4;)
            end
            local.get 2
            local.get 16
            i32.const -2
            i32.add
            local.tee 16
            i32.store offset=4
            local.get 2
            local.get 26
            i32.const 2
            i32.add
            local.tee 23
            i32.store
            local.get 10
            i32.const 16
            i32.or
            local.set 25
            local.get 26
            i32.load16_u align=1
            local.get 10
            i32.shl
            local.get 11
            i32.or
            local.set 1
          end
          local.get 25
          local.get 7
          i32.const 255
          i32.and
          local.tee 4
          i32.sub
          local.set 10
          local.get 1
          local.get 4
          i32.shr_u
          local.set 11
          local.get 1
          i32.const -1
          local.get 4
          i32.shl
          i32.const -1
          i32.xor
          i32.and
          local.get 9
          i32.add
          local.set 9
        end
        block ;; label = @3
          local.get 24
          local.get 15
          local.get 9
          i32.lt_u
          i32.and
          br_if 0 (;@3;)
          local.get 9
          local.get 22
          i32.gt_u
          br_if 0 (;@3;)
          local.get 15
          local.get 9
          i32.sub
          local.get 6
          i32.and
          local.set 1
          block ;; label = @4
            block ;; label = @5
              local.get 8
              i32.const 3
              i32.ne
              br_if 0 (;@5;)
              local.get 15
              i32.const -4
              i32.gt_u
              br_if 1 (;@4;)
              local.get 15
              i32.const 3
              i32.add
              local.get 22
              i32.gt_u
              br_if 1 (;@4;)
              local.get 1
              i32.const 2
              i32.add
              local.get 6
              i32.and
              local.tee 4
              local.get 22
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              i32.const 1
              i32.add
              local.get 6
              i32.and
              local.tee 26
              local.get 22
              i32.ge_u
              br_if 1 (;@4;)
              local.get 1
              local.get 22
              i32.ge_u
              br_if 1 (;@4;)
              local.get 21
              local.get 15
              i32.add
              local.tee 25
              local.get 21
              local.get 1
              i32.add
              i32.load8_u
              i32.store8
              local.get 25
              local.get 21
              local.get 26
              i32.add
              i32.load8_u
              i32.store8 offset=1
              local.get 25
              local.get 21
              local.get 4
              i32.add
              i32.load8_u
              i32.store8 offset=2
              br 1 (;@4;)
            end
            local.get 21
            local.get 22
            local.get 1
            local.get 15
            local.get 8
            local.get 6
            call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7inflate4core8transfer
          end
          local.get 3
          local.get 8
          local.get 15
          i32.add
          local.tee 15
          i32.store offset=8
          local.get 14
          local.get 15
          i32.sub
          i32.const 259
          i32.lt_u
          br_if 2 (;@1;)
          local.get 16
          i32.const 13
          i32.gt_u
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
      end
      i32.const 255
      local.set 13
      i32.const 30
      local.set 12
    end
    local.get 5
    local.get 7
    i32.store8 offset=16
    local.get 5
    local.get 8
    i32.store offset=12
    local.get 5
    local.get 9
    i32.store offset=8
    local.get 5
    local.get 10
    i32.store offset=4
    local.get 5
    local.get 11
    i32.store
    local.get 0
    local.get 12
    i32.store8 offset=1
    local.get 0
    local.get 13
    i32.store8
  )

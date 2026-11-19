(func $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core11flush_block (;13;) (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 656
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    i32.const 1
    local.set 5
    local.get 1
    i32.load offset=40
    local.set 6
    local.get 1
    i32.load offset=4
    local.set 7
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i32.load
                  local.tee 8
                  i32.const 1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 7
                  local.set 9
                  br 1 (;@6;)
                end
                local.get 7
                local.set 9
                local.get 2
                i32.load offset=8
                local.tee 10
                local.get 6
                i32.sub
                i32.const 85196
                i32.lt_u
                br_if 0 (;@6;)
                local.get 6
                i32.const 85180
                i32.add
                local.set 11
                local.get 6
                i32.const -85181
                i32.gt_u
                br_if 1 (;@5;)
                local.get 11
                local.get 10
                i32.gt_u
                br_if 1 (;@5;)
                local.get 2
                i32.load offset=4
                local.get 6
                i32.add
                local.set 9
                i32.const 0
                local.set 5
              end
              local.get 1
              i32.load offset=12
              local.set 12
              local.get 1
              i32.load offset=48
              local.set 13
              local.get 1
              i32.load offset=44
              local.set 11
              i32.const 0
              local.set 10
              local.get 1
              i32.load offset=8
              local.tee 14
              i32.const 4096
              i32.and
              local.tee 15
              i32.eqz
              br_if 4 (;@1;)
              local.get 12
              br_if 4 (;@1;)
              i32.const 0
              local.set 10
              i32.const 0
              local.get 1
              i32.load8_u offset=55
              local.tee 16
              i32.const -8
              i32.add
              local.tee 17
              local.get 17
              local.get 16
              i32.gt_u
              select
              i32.const 4
              i32.shl
              i32.const 8
              i32.or
              local.tee 16
              i32.const 248
              i32.and
              local.get 13
              i32.shl
              i32.const 0
              i32.const 64
              local.get 14
              i32.const 4095
              i32.and
              local.tee 17
              i32.const 2
              i32.lt_u
              select
              i32.const -64
              i32.const -128
              local.get 17
              i32.const 767
              i32.gt_u
              select
              local.get 14
              i32.const 81920
              i32.and
              select
              local.tee 17
              local.get 16
              i32.const 8
              i32.shl
              local.get 17
              i32.const 192
              i32.and
              i32.or
              i32.const 63680
              i32.and
              i32.const 31
              i32.rem_u
              i32.or
              i32.const 31
              i32.xor
              i32.const 223
              i32.and
              local.get 13
              i32.const 8
              i32.add
              local.tee 17
              i32.shl
              i32.or
              local.get 11
              i32.or
              local.set 16
              block ;; label = @6
                local.get 13
                i32.const 16
                i32.add
                local.tee 18
                i32.const 8
                i32.ge_u
                br_if 0 (;@6;)
                local.get 16
                local.set 11
                local.get 18
                local.set 13
                br 5 (;@1;)
              end
              i32.const 0
              local.get 17
              i32.const 3
              i32.shr_u
              i32.sub
              local.set 19
              i32.const 4
              local.set 10
              block ;; label = @6
                block ;; label = @7
                  loop ;; label = @8
                    local.get 9
                    local.get 10
                    i32.add
                    local.tee 18
                    i32.const -4
                    i32.add
                    local.get 16
                    i32.store8
                    local.get 16
                    i32.const 8
                    i32.shr_u
                    local.set 11
                    local.get 19
                    local.get 10
                    i32.add
                    local.tee 17
                    i32.const 4
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 18
                    i32.const -3
                    i32.add
                    local.get 11
                    i32.store8
                    local.get 16
                    i32.const 16
                    i32.shr_u
                    local.set 11
                    local.get 17
                    i32.const 3
                    i32.eq
                    br_if 5 (;@3;)
                    local.get 18
                    i32.const -2
                    i32.add
                    local.get 11
                    i32.store8
                    local.get 16
                    i32.const 24
                    i32.shr_u
                    local.set 11
                    local.get 17
                    i32.const 2
                    i32.eq
                    br_if 6 (;@2;)
                    local.get 18
                    i32.const -1
                    i32.add
                    local.get 11
                    i32.store8
                    local.get 17
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    local.get 13
                    i32.const -32
                    i32.add
                    local.set 13
                    i32.const 0
                    local.set 16
                    local.get 10
                    i32.const 4
                    i32.add
                    local.tee 10
                    i32.const 85184
                    i32.eq
                    br_if 2 (;@6;)
                    br 0 (;@8;)
                  end
                end
                local.get 10
                i32.const -3
                i32.add
                local.set 10
                local.get 13
                i32.const 8
                i32.add
                local.set 13
                br 5 (;@1;)
              end
              i32.const 85180
              i32.const 85180
              i32.const 1049616
              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
              unreachable
            end
            local.get 6
            local.get 11
            local.get 10
            i32.const 1049520
            call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
            unreachable
          end
          local.get 13
          i32.const -16
          i32.add
          local.set 13
          i32.const 0
          local.set 11
          br 2 (;@1;)
        end
        local.get 10
        i32.const -2
        i32.add
        local.set 10
        br 1 (;@1;)
      end
      local.get 10
      i32.const -1
      i32.add
      local.set 10
      local.get 13
      i32.const -8
      i32.add
      local.set 13
    end
    local.get 1
    i32.load offset=72
    local.set 20
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 3
          i32.const 255
          i32.and
          i32.const 4
          i32.eq
          br_if 0 (;@3;)
          local.get 20
          i32.eqz
          br_if 1 (;@2;)
        end
        i32.const 0
        local.set 21
        block ;; label = @3
          local.get 14
          i32.const 524288
          i32.and
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=112
          local.get 1
          i32.load offset=104
          i32.sub
          local.get 1
          i32.load offset=116
          i32.le_u
          local.set 21
        end
        local.get 1
        i64.const 0
        i64.store offset=24 align=4
        local.get 1
        i32.load offset=60
        local.tee 22
        local.get 1
        i32.load16_u offset=68
        i32.add
        local.set 16
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.load offset=76
            local.tee 17
            i32.const 8
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            local.get 1
            i32.load offset=64
            i32.const -1
            i32.add
            i32.store offset=64
            i32.const 0
            local.set 17
            br 1 (;@3;)
          end
          local.get 16
          i32.load8_u
          local.get 17
          i32.const 7
          i32.and
          i32.shr_u
          local.set 17
        end
        local.get 16
        local.get 17
        i32.store8
        local.get 3
        i32.const 255
        i32.and
        i32.const 4
        i32.eq
        local.get 13
        i32.shl
        local.get 11
        i32.or
        local.set 17
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        block ;; label = @19
                                          block ;; label = @20
                                            block ;; label = @21
                                              block ;; label = @22
                                                block ;; label = @23
                                                  block ;; label = @24
                                                    block ;; label = @25
                                                      block ;; label = @26
                                                        block ;; label = @27
                                                          block ;; label = @28
                                                            block ;; label = @29
                                                              block ;; label = @30
                                                                block ;; label = @31
                                                                  block ;; label = @32
                                                                    block ;; label = @33
                                                                      block ;; label = @34
                                                                        local.get 13
                                                                        i32.const 1
                                                                        i32.add
                                                                        local.tee 19
                                                                        i32.const 8
                                                                        i32.ge_u
                                                                        br_if 0 (;@34;)
                                                                        local.get 10
                                                                        local.set 23
                                                                        br 1 (;@33;)
                                                                      end
                                                                      local.get 10
                                                                      i32.const 85180
                                                                      local.get 10
                                                                      i32.const 85180
                                                                      i32.gt_u
                                                                      select
                                                                      local.set 16
                                                                      local.get 10
                                                                      i32.const 85179
                                                                      i32.gt_u
                                                                      br_if 1 (;@32;)
                                                                      local.get 13
                                                                      i32.const -7
                                                                      i32.add
                                                                      local.tee 19
                                                                      i32.const 3
                                                                      i32.shr_u
                                                                      local.set 11
                                                                      block ;; label = @34
                                                                        loop ;; label = @35
                                                                          local.get 9
                                                                          local.get 10
                                                                          i32.add
                                                                          local.get 17
                                                                          i32.store8
                                                                          local.get 17
                                                                          i32.const 8
                                                                          i32.shr_u
                                                                          local.set 17
                                                                          local.get 11
                                                                          i32.eqz
                                                                          br_if 1 (;@34;)
                                                                          local.get 11
                                                                          i32.const -1
                                                                          i32.add
                                                                          local.set 11
                                                                          local.get 19
                                                                          i32.const -8
                                                                          i32.add
                                                                          local.set 19
                                                                          local.get 16
                                                                          local.get 10
                                                                          i32.const 1
                                                                          i32.add
                                                                          local.tee 10
                                                                          i32.eq
                                                                          br_if 3 (;@32;)
                                                                          br 0 (;@35;)
                                                                        end
                                                                      end
                                                                      local.get 10
                                                                      i32.const 1
                                                                      i32.add
                                                                      local.tee 23
                                                                      local.set 10
                                                                    end
                                                                    local.get 17
                                                                    local.set 11
                                                                    local.get 19
                                                                    local.set 13
                                                                    local.get 21
                                                                    br_if 18 (;@14;)
                                                                    local.get 1
                                                                    i32.load offset=80
                                                                    local.set 24
                                                                    local.get 14
                                                                    i32.const 262144
                                                                    i32.and
                                                                    br_if 1 (;@31;)
                                                                    local.get 20
                                                                    i32.const 48
                                                                    i32.lt_u
                                                                    br_if 1 (;@31;)
                                                                    local.get 24
                                                                    i32.const 1
                                                                    i32.store16 offset=512
                                                                    i32.const 0
                                                                    local.set 25
                                                                    local.get 24
                                                                    i32.const 0
                                                                    i32.const 288
                                                                    i32.const 15
                                                                    i32.const 0
                                                                    call $_RNvMsb_NtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4coreNtB5_12HuffmanOxide14optimize_table
                                                                    local.get 24
                                                                    i32.const 1
                                                                    i32.const 32
                                                                    i32.const 15
                                                                    i32.const 0
                                                                    call $_RNvMsb_NtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4coreNtB5_12HuffmanOxide14optimize_table
                                                                    local.get 24
                                                                    i32.const 3456
                                                                    i32.add
                                                                    local.set 16
                                                                    i32.const 0
                                                                    local.set 10
                                                                    block ;; label = @33
                                                                      block ;; label = @34
                                                                        block ;; label = @35
                                                                          loop ;; label = @36
                                                                            local.get 24
                                                                            local.get 10
                                                                            i32.add
                                                                            local.tee 11
                                                                            i32.const 3741
                                                                            i32.add
                                                                            i32.load8_u
                                                                            br_if 3 (;@33;)
                                                                            block ;; label = @37
                                                                              local.get 10
                                                                              i32.const -28
                                                                              i32.ne
                                                                              br_if 0 (;@37;)
                                                                              i32.const 29
                                                                              local.set 25
                                                                              br 4 (;@33;)
                                                                            end
                                                                            local.get 11
                                                                            i32.const 3740
                                                                            i32.add
                                                                            i32.load8_u
                                                                            br_if 2 (;@34;)
                                                                            local.get 11
                                                                            i32.const 3739
                                                                            i32.add
                                                                            i32.load8_u
                                                                            br_if 1 (;@35;)
                                                                            block ;; label = @37
                                                                              local.get 11
                                                                              i32.const 3738
                                                                              i32.add
                                                                              i32.load8_u
                                                                              br_if 0 (;@37;)
                                                                              local.get 10
                                                                              i32.const -4
                                                                              i32.add
                                                                              local.set 10
                                                                              local.get 25
                                                                              i32.const 4
                                                                              i32.add
                                                                              local.set 25
                                                                              br 1 (;@36;)
                                                                            end
                                                                          end
                                                                          i32.const 3
                                                                          local.get 10
                                                                          i32.sub
                                                                          local.set 25
                                                                          br 2 (;@33;)
                                                                        end
                                                                        i32.const 2
                                                                        local.get 10
                                                                        i32.sub
                                                                        local.set 25
                                                                        br 1 (;@33;)
                                                                      end
                                                                      i32.const 1
                                                                      local.get 10
                                                                      i32.sub
                                                                      local.set 25
                                                                    end
                                                                    local.get 24
                                                                    i32.const 3744
                                                                    i32.add
                                                                    local.set 18
                                                                    i32.const 286
                                                                    local.get 25
                                                                    i32.sub
                                                                    local.set 13
                                                                    i32.const 0
                                                                    local.set 26
                                                                    i32.const 0
                                                                    local.set 10
                                                                    block ;; label = @33
                                                                      block ;; label = @34
                                                                        block ;; label = @35
                                                                          loop ;; label = @36
                                                                            local.get 24
                                                                            local.get 10
                                                                            i32.add
                                                                            local.tee 11
                                                                            i32.const 3773
                                                                            i32.add
                                                                            i32.load8_u
                                                                            br_if 3 (;@33;)
                                                                            block ;; label = @37
                                                                              local.get 10
                                                                              i32.const -28
                                                                              i32.ne
                                                                              br_if 0 (;@37;)
                                                                              i32.const 29
                                                                              local.set 26
                                                                              br 4 (;@33;)
                                                                            end
                                                                            local.get 11
                                                                            i32.const 3772
                                                                            i32.add
                                                                            i32.load8_u
                                                                            br_if 2 (;@34;)
                                                                            local.get 11
                                                                            i32.const 3771
                                                                            i32.add
                                                                            i32.load8_u
                                                                            br_if 1 (;@35;)
                                                                            block ;; label = @37
                                                                              local.get 11
                                                                              i32.const 3770
                                                                              i32.add
                                                                              i32.load8_u
                                                                              br_if 0 (;@37;)
                                                                              local.get 10
                                                                              i32.const -4
                                                                              i32.add
                                                                              local.set 10
                                                                              local.get 26
                                                                              i32.const 4
                                                                              i32.add
                                                                              local.set 26
                                                                              br 1 (;@36;)
                                                                            end
                                                                          end
                                                                          i32.const 3
                                                                          local.get 10
                                                                          i32.sub
                                                                          local.set 26
                                                                          br 2 (;@33;)
                                                                        end
                                                                        i32.const 2
                                                                        local.get 10
                                                                        i32.sub
                                                                        local.set 26
                                                                        br 1 (;@33;)
                                                                      end
                                                                      i32.const 1
                                                                      local.get 10
                                                                      i32.sub
                                                                      local.set 26
                                                                    end
                                                                    local.get 4
                                                                    i32.const 4
                                                                    i32.add
                                                                    i32.const 0
                                                                    i32.const 320
                                                                    memory.fill
                                                                    local.get 4
                                                                    i32.const 324
                                                                    i32.add
                                                                    i32.const 0
                                                                    i32.const 320
                                                                    memory.fill
                                                                    block ;; label = @33
                                                                      local.get 13
                                                                      i32.const 321
                                                                      i32.ge_u
                                                                      br_if 0 (;@33;)
                                                                      block ;; label = @34
                                                                        local.get 13
                                                                        i32.const 289
                                                                        i32.ge_u
                                                                        br_if 0 (;@34;)
                                                                        i32.const 30
                                                                        local.get 26
                                                                        i32.sub
                                                                        local.tee 11
                                                                        local.get 13
                                                                        i32.add
                                                                        local.set 10
                                                                        block ;; label = @35
                                                                          local.get 13
                                                                          i32.eqz
                                                                          br_if 0 (;@35;)
                                                                          local.get 4
                                                                          i32.const 4
                                                                          i32.add
                                                                          local.get 16
                                                                          local.get 13
                                                                          memory.copy
                                                                        end
                                                                        block ;; label = @35
                                                                          local.get 10
                                                                          local.get 13
                                                                          i32.lt_u
                                                                          br_if 0 (;@35;)
                                                                          local.get 10
                                                                          i32.const 321
                                                                          i32.ge_u
                                                                          br_if 0 (;@35;)
                                                                          block ;; label = @36
                                                                            local.get 11
                                                                            i32.const 289
                                                                            i32.ge_u
                                                                            br_if 0 (;@36;)
                                                                            block ;; label = @37
                                                                              local.get 11
                                                                              i32.eqz
                                                                              br_if 0 (;@37;)
                                                                              local.get 4
                                                                              i32.const 4
                                                                              i32.add
                                                                              local.get 13
                                                                              i32.add
                                                                              local.get 18
                                                                              local.get 11
                                                                              memory.copy
                                                                            end
                                                                            local.get 24
                                                                            i64.const 0
                                                                            i64.store offset=1182 align=2
                                                                            local.get 24
                                                                            i64.const 0
                                                                            i64.store offset=1176 align=2
                                                                            local.get 24
                                                                            i64.const 0
                                                                            i64.store offset=1168 align=2
                                                                            local.get 24
                                                                            i64.const 0
                                                                            i64.store offset=1160 align=2
                                                                            local.get 24
                                                                            i64.const 0
                                                                            i64.store offset=1152 align=2
                                                                            i32.const 0
                                                                            local.set 27
                                                                            local.get 10
                                                                            i32.eqz
                                                                            br_if 7 (;@29;)
                                                                            local.get 24
                                                                            i32.const 1152
                                                                            i32.add
                                                                            local.set 28
                                                                            local.get 25
                                                                            local.get 26
                                                                            i32.add
                                                                            i32.const -315
                                                                            i32.add
                                                                            local.set 18
                                                                            local.get 4
                                                                            i32.const 4
                                                                            i32.add
                                                                            local.set 16
                                                                            i32.const 0
                                                                            local.set 14
                                                                            i32.const 0
                                                                            local.set 11
                                                                            i32.const 255
                                                                            local.set 29
                                                                            i32.const 0
                                                                            local.set 27
                                                                            block ;; label = @37
                                                                              loop ;; label = @38
                                                                                block ;; label = @39
                                                                                  block ;; label = @40
                                                                                    block ;; label = @41
                                                                                      block ;; label = @42
                                                                                        block ;; label = @43
                                                                                          block ;; label = @44
                                                                                            local.get 16
                                                                                            i32.load8_u
                                                                                            local.tee 13
                                                                                            br_if 0 (;@44;)
                                                                                            block ;; label = @45
                                                                                              local.get 14
                                                                                              i32.const 65535
                                                                                              i32.and
                                                                                              local.tee 30
                                                                                              br_if 0 (;@45;)
                                                                                              local.get 27
                                                                                              local.set 10
                                                                                              br 2 (;@43;)
                                                                                            end
                                                                                            block ;; label = @45
                                                                                              local.get 30
                                                                                              i32.const 3
                                                                                              i32.lt_u
                                                                                              br_if 0 (;@45;)
                                                                                              local.get 24
                                                                                              local.get 24
                                                                                              i32.load16_u offset=1184
                                                                                              i32.const 1
                                                                                              i32.add
                                                                                              i32.store16 offset=1184
                                                                                              local.get 27
                                                                                              i32.const 318
                                                                                              i32.gt_u
                                                                                              br_if 17 (;@28;)
                                                                                              local.get 4
                                                                                              i32.const 324
                                                                                              i32.add
                                                                                              local.get 27
                                                                                              i32.add
                                                                                              local.tee 10
                                                                                              local.get 14
                                                                                              i32.const -3
                                                                                              i32.add
                                                                                              i32.store8 offset=1
                                                                                              local.get 10
                                                                                              i32.const 16
                                                                                              i32.store8
                                                                                              local.get 27
                                                                                              i32.const 2
                                                                                              i32.add
                                                                                              local.set 10
                                                                                              br 2 (;@43;)
                                                                                            end
                                                                                            local.get 28
                                                                                            local.get 29
                                                                                            i32.const 255
                                                                                            i32.and
                                                                                            i32.const 1
                                                                                            i32.shl
                                                                                            i32.add
                                                                                            local.tee 10
                                                                                            local.get 10
                                                                                            i32.load16_u
                                                                                            local.get 14
                                                                                            i32.add
                                                                                            i32.store16
                                                                                            local.get 4
                                                                                            local.get 29
                                                                                            i32.store8 offset=646
                                                                                            local.get 4
                                                                                            local.get 29
                                                                                            i32.store8 offset=645
                                                                                            local.get 4
                                                                                            local.get 29
                                                                                            i32.store8 offset=644
                                                                                            local.get 27
                                                                                            local.get 30
                                                                                            i32.add
                                                                                            local.tee 10
                                                                                            local.get 27
                                                                                            i32.lt_u
                                                                                            br_if 16 (;@28;)
                                                                                            local.get 10
                                                                                            i32.const 321
                                                                                            i32.ge_u
                                                                                            br_if 16 (;@28;)
                                                                                            local.get 30
                                                                                            i32.eqz
                                                                                            br_if 1 (;@43;)
                                                                                            local.get 4
                                                                                            i32.const 324
                                                                                            i32.add
                                                                                            local.get 27
                                                                                            i32.add
                                                                                            local.get 4
                                                                                            i32.const 644
                                                                                            i32.add
                                                                                            local.get 30
                                                                                            memory.copy
                                                                                            br 1 (;@43;)
                                                                                          end
                                                                                          block ;; label = @44
                                                                                            block ;; label = @45
                                                                                              local.get 11
                                                                                              br_if 0 (;@45;)
                                                                                              local.get 27
                                                                                              local.set 10
                                                                                              br 1 (;@44;)
                                                                                            end
                                                                                            block ;; label = @45
                                                                                              block ;; label = @46
                                                                                                block ;; label = @47
                                                                                                  local.get 11
                                                                                                  i32.const 3
                                                                                                  i32.lt_u
                                                                                                  br_if 0 (;@47;)
                                                                                                  local.get 11
                                                                                                  i32.const 11
                                                                                                  i32.lt_u
                                                                                                  br_if 1 (;@46;)
                                                                                                  local.get 24
                                                                                                  local.get 24
                                                                                                  i32.load16_u offset=1188
                                                                                                  i32.const 1
                                                                                                  i32.add
                                                                                                  i32.store16 offset=1188
                                                                                                  local.get 27
                                                                                                  i32.const 318
                                                                                                  i32.gt_u
                                                                                                  br_if 19 (;@28;)
                                                                                                  local.get 4
                                                                                                  i32.const 324
                                                                                                  i32.add
                                                                                                  local.get 27
                                                                                                  i32.add
                                                                                                  local.tee 10
                                                                                                  local.get 11
                                                                                                  i32.const -11
                                                                                                  i32.add
                                                                                                  i32.store8 offset=1
                                                                                                  local.get 10
                                                                                                  i32.const 18
                                                                                                  i32.store8
                                                                                                  br 2 (;@45;)
                                                                                                end
                                                                                                local.get 28
                                                                                                local.get 28
                                                                                                i32.load16_u
                                                                                                local.get 11
                                                                                                i32.add
                                                                                                i32.store16
                                                                                                local.get 27
                                                                                                local.get 11
                                                                                                i32.add
                                                                                                local.tee 10
                                                                                                local.get 27
                                                                                                i32.lt_u
                                                                                                br_if 18 (;@28;)
                                                                                                local.get 10
                                                                                                i32.const 320
                                                                                                i32.gt_u
                                                                                                br_if 18 (;@28;)
                                                                                                local.get 11
                                                                                                i32.eqz
                                                                                                br_if 2 (;@44;)
                                                                                                local.get 4
                                                                                                i32.const 324
                                                                                                i32.add
                                                                                                local.get 27
                                                                                                i32.add
                                                                                                i32.const 0
                                                                                                local.get 11
                                                                                                memory.fill
                                                                                                br 2 (;@44;)
                                                                                              end
                                                                                              local.get 24
                                                                                              local.get 24
                                                                                              i32.load16_u offset=1186
                                                                                              i32.const 1
                                                                                              i32.add
                                                                                              i32.store16 offset=1186
                                                                                              local.get 27
                                                                                              i32.const 318
                                                                                              i32.gt_u
                                                                                              br_if 17 (;@28;)
                                                                                              local.get 4
                                                                                              i32.const 324
                                                                                              i32.add
                                                                                              local.get 27
                                                                                              i32.add
                                                                                              local.tee 10
                                                                                              local.get 11
                                                                                              i32.const -3
                                                                                              i32.add
                                                                                              i32.store8 offset=1
                                                                                              local.get 10
                                                                                              i32.const 17
                                                                                              i32.store8
                                                                                            end
                                                                                            local.get 27
                                                                                            i32.const 2
                                                                                            i32.add
                                                                                            local.set 10
                                                                                          end
                                                                                          block ;; label = @44
                                                                                            block ;; label = @45
                                                                                              local.get 13
                                                                                              local.get 29
                                                                                              i32.const 255
                                                                                              i32.and
                                                                                              local.tee 11
                                                                                              i32.ne
                                                                                              br_if 0 (;@45;)
                                                                                              local.get 14
                                                                                              i32.const 1
                                                                                              i32.add
                                                                                              local.tee 14
                                                                                              i32.const 65535
                                                                                              i32.and
                                                                                              i32.const 6
                                                                                              i32.ne
                                                                                              br_if 4 (;@41;)
                                                                                              local.get 24
                                                                                              local.get 24
                                                                                              i32.load16_u offset=1184
                                                                                              i32.const 1
                                                                                              i32.add
                                                                                              i32.store16 offset=1184
                                                                                              local.get 10
                                                                                              i32.const 318
                                                                                              i32.le_u
                                                                                              br_if 1 (;@44;)
                                                                                              br 17 (;@28;)
                                                                                            end
                                                                                            block ;; label = @45
                                                                                              block ;; label = @46
                                                                                                local.get 14
                                                                                                i32.const 65535
                                                                                                i32.and
                                                                                                local.tee 27
                                                                                                br_if 0 (;@46;)
                                                                                                local.get 10
                                                                                                local.set 11
                                                                                                br 1 (;@45;)
                                                                                              end
                                                                                              block ;; label = @46
                                                                                                local.get 27
                                                                                                i32.const 3
                                                                                                i32.lt_u
                                                                                                br_if 0 (;@46;)
                                                                                                local.get 24
                                                                                                local.get 24
                                                                                                i32.load16_u offset=1184
                                                                                                i32.const 1
                                                                                                i32.add
                                                                                                i32.store16 offset=1184
                                                                                                local.get 10
                                                                                                i32.const 318
                                                                                                i32.gt_u
                                                                                                br_if 18 (;@28;)
                                                                                                local.get 4
                                                                                                i32.const 324
                                                                                                i32.add
                                                                                                local.get 10
                                                                                                i32.add
                                                                                                local.tee 11
                                                                                                local.get 14
                                                                                                i32.const -3
                                                                                                i32.add
                                                                                                i32.store8 offset=1
                                                                                                local.get 11
                                                                                                i32.const 16
                                                                                                i32.store8
                                                                                                local.get 10
                                                                                                i32.const 2
                                                                                                i32.add
                                                                                                local.set 11
                                                                                                br 1 (;@45;)
                                                                                              end
                                                                                              local.get 28
                                                                                              local.get 11
                                                                                              i32.const 1
                                                                                              i32.shl
                                                                                              i32.add
                                                                                              local.tee 11
                                                                                              local.get 11
                                                                                              i32.load16_u
                                                                                              local.get 14
                                                                                              i32.add
                                                                                              i32.store16
                                                                                              local.get 4
                                                                                              local.get 29
                                                                                              i32.store8 offset=646
                                                                                              local.get 4
                                                                                              local.get 29
                                                                                              i32.store8 offset=645
                                                                                              local.get 4
                                                                                              local.get 29
                                                                                              i32.store8 offset=644
                                                                                              local.get 10
                                                                                              local.get 27
                                                                                              i32.add
                                                                                              local.tee 11
                                                                                              local.get 10
                                                                                              i32.lt_u
                                                                                              br_if 17 (;@28;)
                                                                                              local.get 11
                                                                                              i32.const 321
                                                                                              i32.ge_u
                                                                                              br_if 17 (;@28;)
                                                                                              local.get 27
                                                                                              i32.eqz
                                                                                              br_if 0 (;@45;)
                                                                                              local.get 4
                                                                                              i32.const 324
                                                                                              i32.add
                                                                                              local.get 10
                                                                                              i32.add
                                                                                              local.get 4
                                                                                              i32.const 644
                                                                                              i32.add
                                                                                              local.get 27
                                                                                              memory.copy
                                                                                            end
                                                                                            local.get 28
                                                                                            local.get 13
                                                                                            i32.const 1
                                                                                            i32.shl
                                                                                            i32.add
                                                                                            local.tee 10
                                                                                            local.get 10
                                                                                            i32.load16_u
                                                                                            i32.const 1
                                                                                            i32.add
                                                                                            i32.store16
                                                                                            local.get 11
                                                                                            i32.const 319
                                                                                            i32.gt_u
                                                                                            br_if 16 (;@28;)
                                                                                            local.get 4
                                                                                            i32.const 324
                                                                                            i32.add
                                                                                            local.get 11
                                                                                            i32.add
                                                                                            local.get 13
                                                                                            i32.store8
                                                                                            local.get 11
                                                                                            i32.const 1
                                                                                            i32.add
                                                                                            local.set 10
                                                                                            br 2 (;@42;)
                                                                                          end
                                                                                          local.get 4
                                                                                          i32.const 324
                                                                                          i32.add
                                                                                          local.get 10
                                                                                          i32.add
                                                                                          i32.const 784
                                                                                          i32.store16 align=1
                                                                                          local.get 10
                                                                                          i32.const 2
                                                                                          i32.add
                                                                                          local.set 27
                                                                                          i32.const 0
                                                                                          local.set 14
                                                                                          i32.const 0
                                                                                          local.set 11
                                                                                          local.get 18
                                                                                          i32.eqz
                                                                                          br_if 14 (;@29;)
                                                                                          br 4 (;@39;)
                                                                                        end
                                                                                        block ;; label = @43
                                                                                          local.get 11
                                                                                          i32.const 1
                                                                                          i32.add
                                                                                          local.tee 11
                                                                                          i32.const 138
                                                                                          i32.eq
                                                                                          br_if 0 (;@43;)
                                                                                          i32.const 0
                                                                                          local.set 14
                                                                                          br 3 (;@40;)
                                                                                        end
                                                                                        local.get 24
                                                                                        local.get 24
                                                                                        i32.load16_u offset=1188
                                                                                        i32.const 1
                                                                                        i32.add
                                                                                        i32.store16 offset=1188
                                                                                        local.get 10
                                                                                        i32.const 318
                                                                                        i32.gt_u
                                                                                        br_if 14 (;@28;)
                                                                                        local.get 4
                                                                                        i32.const 324
                                                                                        i32.add
                                                                                        local.get 10
                                                                                        i32.add
                                                                                        i32.const 32530
                                                                                        i32.store16 align=1
                                                                                        local.get 10
                                                                                        i32.const 2
                                                                                        i32.add
                                                                                        local.set 10
                                                                                      end
                                                                                      i32.const 0
                                                                                      local.set 14
                                                                                    end
                                                                                    i32.const 0
                                                                                    local.set 11
                                                                                  end
                                                                                  local.get 18
                                                                                  i32.eqz
                                                                                  br_if 2 (;@37;)
                                                                                  local.get 10
                                                                                  local.set 27
                                                                                end
                                                                                local.get 16
                                                                                i32.const 1
                                                                                i32.add
                                                                                local.set 16
                                                                                local.get 18
                                                                                i32.const 1
                                                                                i32.add
                                                                                local.set 18
                                                                                local.get 13
                                                                                local.set 29
                                                                                br 0 (;@38;)
                                                                              end
                                                                            end
                                                                            local.get 14
                                                                            i32.const 65535
                                                                            i32.and
                                                                            local.tee 16
                                                                            i32.eqz
                                                                            br_if 6 (;@30;)
                                                                            block ;; label = @37
                                                                              local.get 16
                                                                              i32.const 3
                                                                              i32.lt_u
                                                                              br_if 0 (;@37;)
                                                                              local.get 24
                                                                              local.get 24
                                                                              i32.load16_u offset=1184
                                                                              i32.const 1
                                                                              i32.add
                                                                              i32.store16 offset=1184
                                                                              local.get 10
                                                                              i32.const 318
                                                                              i32.gt_u
                                                                              br_if 9 (;@28;)
                                                                              local.get 4
                                                                              i32.const 324
                                                                              i32.add
                                                                              local.get 10
                                                                              i32.add
                                                                              local.tee 11
                                                                              local.get 14
                                                                              i32.const -3
                                                                              i32.add
                                                                              i32.store8 offset=1
                                                                              local.get 11
                                                                              i32.const 16
                                                                              i32.store8
                                                                              local.get 10
                                                                              i32.const 2
                                                                              i32.add
                                                                              local.set 27
                                                                              br 8 (;@29;)
                                                                            end
                                                                            local.get 28
                                                                            local.get 13
                                                                            i32.const 1
                                                                            i32.shl
                                                                            i32.add
                                                                            local.tee 11
                                                                            local.get 11
                                                                            i32.load16_u
                                                                            local.get 14
                                                                            i32.add
                                                                            i32.store16
                                                                            local.get 4
                                                                            local.get 13
                                                                            i32.store8 offset=646
                                                                            local.get 4
                                                                            local.get 13
                                                                            i32.store8 offset=645
                                                                            local.get 4
                                                                            local.get 13
                                                                            i32.store8 offset=644
                                                                            local.get 10
                                                                            local.get 14
                                                                            i32.const 65535
                                                                            i32.and
                                                                            local.tee 11
                                                                            i32.add
                                                                            local.tee 27
                                                                            local.get 10
                                                                            i32.lt_u
                                                                            br_if 8 (;@28;)
                                                                            local.get 27
                                                                            i32.const 321
                                                                            i32.ge_u
                                                                            br_if 8 (;@28;)
                                                                            local.get 11
                                                                            i32.eqz
                                                                            br_if 7 (;@29;)
                                                                            local.get 4
                                                                            i32.const 324
                                                                            i32.add
                                                                            local.get 10
                                                                            i32.add
                                                                            local.get 4
                                                                            i32.const 644
                                                                            i32.add
                                                                            local.get 11
                                                                            memory.copy
                                                                            br 7 (;@29;)
                                                                          end
                                                                          i32.const 0
                                                                          local.get 11
                                                                          i32.const 288
                                                                          i32.const 1049904
                                                                          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                                                                          unreachable
                                                                        end
                                                                        local.get 13
                                                                        local.get 10
                                                                        i32.const 320
                                                                        i32.const 1049920
                                                                        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                                                                        unreachable
                                                                      end
                                                                      i32.const 0
                                                                      local.get 13
                                                                      i32.const 288
                                                                      i32.const 1049776
                                                                      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                                                                      unreachable
                                                                    end
                                                                    i32.const 0
                                                                    local.get 13
                                                                    i32.const 320
                                                                    i32.const 1049936
                                                                    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                                                                    unreachable
                                                                  end
                                                                  local.get 16
                                                                  i32.const 85180
                                                                  i32.const 1049616
                                                                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                                                  unreachable
                                                                end
                                                                local.get 24
                                                                i32.const 3456
                                                                i32.add
                                                                i32.const 8
                                                                i32.const 144
                                                                memory.fill
                                                                local.get 24
                                                                i32.const 3600
                                                                i32.add
                                                                i32.const 9
                                                                i32.const 112
                                                                memory.fill
                                                                local.get 24
                                                                i64.const 506381209866536711
                                                                i64.store offset=3728 align=2
                                                                local.get 24
                                                                i64.const 506381209866536711
                                                                i64.store offset=3720 align=2
                                                                local.get 24
                                                                i64.const 506381209866536711
                                                                i64.store offset=3712 align=2
                                                                local.get 24
                                                                i64.const 361700864190383365
                                                                i64.store offset=3744 align=2
                                                                local.get 24
                                                                i64.const 578721382704613384
                                                                i64.store offset=3736 align=2
                                                                local.get 24
                                                                i64.const 361700864190383365
                                                                i64.store offset=3752 align=2
                                                                local.get 24
                                                                i64.const 361700864190383365
                                                                i64.store offset=3760 align=2
                                                                local.get 24
                                                                i64.const 361700864190383365
                                                                i64.store offset=3768 align=2
                                                                local.get 24
                                                                i32.const 0
                                                                i32.const 288
                                                                i32.const 15
                                                                i32.const 1
                                                                call $_RNvMsb_NtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4coreNtB5_12HuffmanOxide14optimize_table
                                                                local.get 24
                                                                i32.const 1
                                                                i32.const 32
                                                                i32.const 15
                                                                i32.const 1
                                                                call $_RNvMsb_NtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4coreNtB5_12HuffmanOxide14optimize_table
                                                                local.get 19
                                                                i32.const 2
                                                                i32.add
                                                                local.set 13
                                                                i32.const 1
                                                                local.get 19
                                                                i32.shl
                                                                local.get 17
                                                                i32.or
                                                                local.set 11
                                                                block ;; label = @31
                                                                  local.get 19
                                                                  i32.const 6
                                                                  i32.ge_u
                                                                  br_if 0 (;@31;)
                                                                  local.get 23
                                                                  local.set 10
                                                                  br 9 (;@22;)
                                                                end
                                                                local.get 23
                                                                i32.const 85180
                                                                local.get 23
                                                                i32.const 85180
                                                                i32.gt_u
                                                                select
                                                                local.set 16
                                                                local.get 23
                                                                local.set 10
                                                                block ;; label = @31
                                                                  loop ;; label = @32
                                                                    local.get 16
                                                                    local.get 10
                                                                    i32.eq
                                                                    br_if 1 (;@31;)
                                                                    local.get 9
                                                                    local.get 10
                                                                    i32.add
                                                                    local.get 11
                                                                    i32.store8
                                                                    local.get 11
                                                                    i32.const 8
                                                                    i32.shr_u
                                                                    local.set 11
                                                                    local.get 10
                                                                    i32.const 1
                                                                    i32.add
                                                                    local.set 10
                                                                    local.get 13
                                                                    i32.const -8
                                                                    i32.add
                                                                    local.tee 13
                                                                    i32.const 7
                                                                    i32.gt_u
                                                                    br_if 0 (;@32;)
                                                                    br 10 (;@22;)
                                                                  end
                                                                end
                                                                local.get 16
                                                                i32.const 85180
                                                                i32.const 1049616
                                                                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                                                unreachable
                                                              end
                                                              block ;; label = @30
                                                                local.get 11
                                                                br_if 0 (;@30;)
                                                                local.get 10
                                                                local.set 27
                                                                br 1 (;@29;)
                                                              end
                                                              block ;; label = @30
                                                                local.get 11
                                                                i32.const 3
                                                                i32.lt_u
                                                                br_if 0 (;@30;)
                                                                block ;; label = @31
                                                                  local.get 11
                                                                  i32.const 11
                                                                  i32.lt_u
                                                                  br_if 0 (;@31;)
                                                                  local.get 24
                                                                  local.get 24
                                                                  i32.load16_u offset=1188
                                                                  i32.const 1
                                                                  i32.add
                                                                  i32.store16 offset=1188
                                                                  local.get 10
                                                                  i32.const 318
                                                                  i32.gt_u
                                                                  br_if 3 (;@28;)
                                                                  local.get 4
                                                                  i32.const 324
                                                                  i32.add
                                                                  local.get 10
                                                                  i32.add
                                                                  local.tee 13
                                                                  local.get 11
                                                                  i32.const -11
                                                                  i32.add
                                                                  i32.store8 offset=1
                                                                  local.get 13
                                                                  i32.const 18
                                                                  i32.store8
                                                                  local.get 10
                                                                  i32.const 2
                                                                  i32.add
                                                                  local.set 27
                                                                  br 2 (;@29;)
                                                                end
                                                                local.get 24
                                                                local.get 24
                                                                i32.load16_u offset=1186
                                                                i32.const 1
                                                                i32.add
                                                                i32.store16 offset=1186
                                                                local.get 10
                                                                i32.const 318
                                                                i32.gt_u
                                                                br_if 2 (;@28;)
                                                                local.get 4
                                                                i32.const 324
                                                                i32.add
                                                                local.get 10
                                                                i32.add
                                                                local.tee 13
                                                                local.get 11
                                                                i32.const -3
                                                                i32.add
                                                                i32.store8 offset=1
                                                                local.get 13
                                                                i32.const 17
                                                                i32.store8
                                                                local.get 10
                                                                i32.const 2
                                                                i32.add
                                                                local.set 27
                                                                br 1 (;@29;)
                                                              end
                                                              local.get 28
                                                              local.get 28
                                                              i32.load16_u
                                                              local.get 11
                                                              i32.add
                                                              i32.store16
                                                              local.get 10
                                                              local.get 11
                                                              i32.add
                                                              local.tee 27
                                                              local.get 10
                                                              i32.lt_u
                                                              br_if 1 (;@28;)
                                                              local.get 27
                                                              i32.const 320
                                                              i32.gt_u
                                                              br_if 1 (;@28;)
                                                              local.get 11
                                                              i32.eqz
                                                              br_if 0 (;@29;)
                                                              local.get 4
                                                              i32.const 324
                                                              i32.add
                                                              local.get 10
                                                              i32.add
                                                              i32.const 0
                                                              local.get 11
                                                              memory.fill
                                                            end
                                                            i32.const 19
                                                            local.set 14
                                                            local.get 24
                                                            i32.const 2
                                                            i32.const 19
                                                            i32.const 7
                                                            i32.const 0
                                                            call $_RNvMsb_NtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4coreNtB5_12HuffmanOxide14optimize_table
                                                            block ;; label = @29
                                                              block ;; label = @30
                                                                block ;; label = @31
                                                                  local.get 24
                                                                  i32.load8_u offset=4047
                                                                  br_if 0 (;@31;)
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4033
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 18
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4046
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 17
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4034
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 16
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4045
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 15
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4035
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 14
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4044
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 13
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4036
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 12
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4043
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 11
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4037
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 10
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4042
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 9
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4038
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 8
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4041
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 7
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4039
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 6
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  block ;; label = @32
                                                                    local.get 24
                                                                    i32.load8_u offset=4040
                                                                    i32.eqz
                                                                    br_if 0 (;@32;)
                                                                    i32.const 5
                                                                    local.set 14
                                                                    br 1 (;@31;)
                                                                  end
                                                                  i32.const 4
                                                                  local.set 14
                                                                  local.get 24
                                                                  i32.load8_u offset=4032
                                                                  br_if 0 (;@31;)
                                                                  local.get 24
                                                                  i32.load8_u offset=4050
                                                                  i32.const 255
                                                                  i32.and
                                                                  br_if 0 (;@31;)
                                                                  local.get 24
                                                                  i32.load8_u offset=4049
                                                                  i32.const 255
                                                                  i32.and
                                                                  br_if 0 (;@31;)
                                                                  local.get 24
                                                                  i32.load8_u offset=4048
                                                                  i32.const 255
                                                                  i32.and
                                                                  i32.eqz
                                                                  br_if 1 (;@30;)
                                                                end
                                                                local.get 14
                                                                i32.const -4
                                                                i32.add
                                                                local.set 10
                                                                br 1 (;@29;)
                                                              end
                                                              i32.const 1
                                                              i32.eqz
                                                              br_if 2 (;@27;)
                                                              i32.const 0
                                                              local.set 10
                                                              i32.const 4
                                                              local.set 14
                                                            end
                                                            local.get 24
                                                            i32.const 4032
                                                            i32.add
                                                            local.set 29
                                                            i32.const 2
                                                            local.get 19
                                                            i32.shl
                                                            i32.const 29
                                                            local.get 25
                                                            i32.sub
                                                            local.get 19
                                                            i32.const 2
                                                            i32.add
                                                            i32.shl
                                                            i32.or
                                                            i32.const 29
                                                            local.get 26
                                                            i32.sub
                                                            local.get 19
                                                            i32.const 7
                                                            i32.add
                                                            i32.shl
                                                            i32.or
                                                            local.get 10
                                                            local.get 19
                                                            i32.const 12
                                                            i32.add
                                                            i32.shl
                                                            i32.or
                                                            local.get 17
                                                            i32.or
                                                            local.set 11
                                                            local.get 23
                                                            i32.const 85180
                                                            local.get 23
                                                            i32.const 85180
                                                            i32.gt_u
                                                            select
                                                            local.set 16
                                                            local.get 19
                                                            i32.const 16
                                                            i32.or
                                                            local.set 13
                                                            local.get 23
                                                            local.set 10
                                                            loop ;; label = @29
                                                              local.get 16
                                                              local.get 10
                                                              i32.eq
                                                              br_if 3 (;@26;)
                                                              local.get 9
                                                              local.get 10
                                                              i32.add
                                                              local.get 11
                                                              i32.store8
                                                              local.get 11
                                                              i32.const 8
                                                              i32.shr_u
                                                              local.set 11
                                                              local.get 10
                                                              i32.const 1
                                                              i32.add
                                                              local.set 10
                                                              local.get 13
                                                              i32.const -8
                                                              i32.add
                                                              local.tee 13
                                                              i32.const 7
                                                              i32.gt_u
                                                              br_if 0 (;@29;)
                                                            end
                                                            i32.const 0
                                                            local.set 16
                                                            loop ;; label = @29
                                                              local.get 29
                                                              local.get 16
                                                              i32.const 1049792
                                                              i32.add
                                                              i32.load8_u
                                                              i32.add
                                                              i32.load8_u
                                                              local.tee 18
                                                              i32.const 8
                                                              i32.ge_u
                                                              br_if 4 (;@25;)
                                                              local.get 18
                                                              local.get 13
                                                              i32.shl
                                                              local.get 11
                                                              i32.or
                                                              local.set 11
                                                              block ;; label = @30
                                                                block ;; label = @31
                                                                  local.get 13
                                                                  i32.const 5
                                                                  i32.ge_u
                                                                  br_if 0 (;@31;)
                                                                  local.get 13
                                                                  i32.const 3
                                                                  i32.add
                                                                  local.set 13
                                                                  br 1 (;@30;)
                                                                end
                                                                local.get 10
                                                                i32.const 85180
                                                                i32.ge_u
                                                                br_if 6 (;@24;)
                                                                local.get 9
                                                                local.get 10
                                                                i32.add
                                                                local.get 11
                                                                i32.store8
                                                                local.get 13
                                                                i32.const -5
                                                                i32.add
                                                                local.set 13
                                                                local.get 11
                                                                i32.const 8
                                                                i32.shr_u
                                                                local.set 11
                                                                local.get 10
                                                                i32.const 1
                                                                i32.add
                                                                local.set 10
                                                              end
                                                              local.get 14
                                                              local.get 16
                                                              i32.const 1
                                                              i32.add
                                                              local.tee 16
                                                              i32.eq
                                                              br_if 6 (;@23;)
                                                              br 0 (;@29;)
                                                            end
                                                          end
                                                          i32.const 1
                                                          local.set 16
                                                          br 26 (;@1;)
                                                        end
                                                        i32.const 1049552
                                                        i32.const 48
                                                        i32.const 1049600
                                                        call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                                                        unreachable
                                                      end
                                                      local.get 16
                                                      i32.const 85180
                                                      i32.const 1049616
                                                      call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                                      unreachable
                                                    end
                                                    i32.const 1049552
                                                    i32.const 48
                                                    i32.const 1049600
                                                    call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                                                    unreachable
                                                  end
                                                  local.get 10
                                                  i32.const 85180
                                                  i32.const 1049616
                                                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                                  unreachable
                                                end
                                                local.get 27
                                                i32.eqz
                                                br_if 0 (;@22;)
                                                local.get 24
                                                i32.const 2880
                                                i32.add
                                                local.set 26
                                                i32.const 0
                                                local.set 18
                                                loop ;; label = @23
                                                  local.get 18
                                                  i32.const 319
                                                  i32.gt_u
                                                  br_if 7 (;@16;)
                                                  local.get 4
                                                  i32.const 324
                                                  i32.add
                                                  local.get 18
                                                  i32.add
                                                  i32.load8_u
                                                  local.tee 14
                                                  i32.const 19
                                                  i32.ge_u
                                                  br_if 6 (;@17;)
                                                  local.get 26
                                                  local.get 14
                                                  i32.const 1
                                                  i32.shl
                                                  i32.add
                                                  i32.load16_u
                                                  local.tee 16
                                                  local.get 29
                                                  local.get 14
                                                  i32.add
                                                  i32.load8_u
                                                  local.tee 25
                                                  i32.shr_u
                                                  br_if 5 (;@18;)
                                                  local.get 16
                                                  local.get 13
                                                  i32.shl
                                                  local.get 11
                                                  i32.or
                                                  local.set 11
                                                  block ;; label = @24
                                                    local.get 13
                                                    local.get 25
                                                    i32.add
                                                    local.tee 13
                                                    i32.const 8
                                                    i32.lt_u
                                                    br_if 0 (;@24;)
                                                    local.get 10
                                                    i32.const 85180
                                                    local.get 10
                                                    i32.const 85180
                                                    i32.gt_u
                                                    select
                                                    local.set 16
                                                    loop ;; label = @25
                                                      local.get 16
                                                      local.get 10
                                                      i32.eq
                                                      br_if 6 (;@19;)
                                                      local.get 9
                                                      local.get 10
                                                      i32.add
                                                      local.get 11
                                                      i32.store8
                                                      local.get 11
                                                      i32.const 8
                                                      i32.shr_u
                                                      local.set 11
                                                      local.get 10
                                                      i32.const 1
                                                      i32.add
                                                      local.set 10
                                                      local.get 13
                                                      i32.const -8
                                                      i32.add
                                                      local.tee 13
                                                      i32.const 7
                                                      i32.gt_u
                                                      br_if 0 (;@25;)
                                                    end
                                                  end
                                                  local.get 18
                                                  i32.const 1
                                                  i32.add
                                                  local.set 16
                                                  block ;; label = @24
                                                    block ;; label = @25
                                                      block ;; label = @26
                                                        local.get 14
                                                        i32.const 15
                                                        i32.gt_u
                                                        br_if 0 (;@26;)
                                                        local.get 16
                                                        local.set 18
                                                        br 1 (;@25;)
                                                      end
                                                      local.get 18
                                                      i32.const 319
                                                      i32.eq
                                                      br_if 1 (;@24;)
                                                      local.get 4
                                                      i32.const 324
                                                      i32.add
                                                      local.get 16
                                                      i32.add
                                                      i32.load8_u
                                                      local.set 16
                                                      local.get 4
                                                      i32.const 7
                                                      i32.store offset=652
                                                      local.get 4
                                                      i64.const 12884901890
                                                      i64.store offset=644 align=4
                                                      local.get 16
                                                      local.get 4
                                                      i32.const 644
                                                      i32.add
                                                      local.get 14
                                                      i32.const 2
                                                      i32.shl
                                                      i32.add
                                                      i32.const -64
                                                      i32.add
                                                      i32.load
                                                      local.tee 14
                                                      i32.shr_u
                                                      br_if 5 (;@20;)
                                                      local.get 16
                                                      local.get 13
                                                      i32.shl
                                                      local.get 11
                                                      i32.or
                                                      local.set 11
                                                      block ;; label = @26
                                                        local.get 14
                                                        local.get 13
                                                        i32.add
                                                        local.tee 13
                                                        i32.const 8
                                                        i32.lt_u
                                                        br_if 0 (;@26;)
                                                        local.get 10
                                                        i32.const 85180
                                                        local.get 10
                                                        i32.const 85180
                                                        i32.gt_u
                                                        select
                                                        local.set 16
                                                        loop ;; label = @27
                                                          local.get 16
                                                          local.get 10
                                                          i32.eq
                                                          br_if 6 (;@21;)
                                                          local.get 9
                                                          local.get 10
                                                          i32.add
                                                          local.get 11
                                                          i32.store8
                                                          local.get 11
                                                          i32.const 8
                                                          i32.shr_u
                                                          local.set 11
                                                          local.get 10
                                                          i32.const 1
                                                          i32.add
                                                          local.set 10
                                                          local.get 13
                                                          i32.const -8
                                                          i32.add
                                                          local.tee 13
                                                          i32.const 7
                                                          i32.gt_u
                                                          br_if 0 (;@27;)
                                                        end
                                                      end
                                                      local.get 18
                                                      i32.const 2
                                                      i32.add
                                                      local.set 18
                                                    end
                                                    local.get 18
                                                    local.get 27
                                                    i32.lt_u
                                                    br_if 1 (;@23;)
                                                    br 2 (;@22;)
                                                  end
                                                end
                                                i32.const 320
                                                i32.const 320
                                                i32.const 1049888
                                                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                                unreachable
                                              end
                                              local.get 11
                                              i64.extend_i32_u
                                              local.set 31
                                              local.get 1
                                              i32.load offset=64
                                              local.tee 11
                                              i32.eqz
                                              br_if 6 (;@15;)
                                              local.get 11
                                              i32.const 65536
                                              local.get 11
                                              i32.const 65536
                                              i32.lt_u
                                              select
                                              local.set 26
                                              local.get 24
                                              i32.const 3744
                                              i32.add
                                              local.set 30
                                              local.get 24
                                              i32.const 2304
                                              i32.add
                                              local.set 32
                                              local.get 24
                                              i32.const 3456
                                              i32.add
                                              local.set 29
                                              local.get 24
                                              i32.const 1728
                                              i32.add
                                              local.set 25
                                              i32.const 1
                                              local.set 18
                                              i32.const 0
                                              local.set 14
                                              loop ;; label = @22
                                                block ;; label = @23
                                                  block ;; label = @24
                                                    local.get 18
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 0 (;@24;)
                                                    local.get 14
                                                    local.set 11
                                                    br 1 (;@23;)
                                                  end
                                                  local.get 14
                                                  i32.const 1
                                                  i32.add
                                                  local.set 11
                                                  local.get 22
                                                  local.get 14
                                                  i32.add
                                                  i32.load8_u
                                                  i32.const 256
                                                  i32.or
                                                  local.set 18
                                                end
                                                block ;; label = @23
                                                  block ;; label = @24
                                                    block ;; label = @25
                                                      block ;; label = @26
                                                        block ;; label = @27
                                                          block ;; label = @28
                                                            local.get 18
                                                            i32.const 1
                                                            i32.and
                                                            i32.eqz
                                                            br_if 0 (;@28;)
                                                            local.get 22
                                                            local.get 11
                                                            i32.const 2
                                                            i32.add
                                                            i32.const 65535
                                                            i32.and
                                                            i32.add
                                                            i32.load8_u
                                                            local.tee 16
                                                            i32.const 8
                                                            i32.shl
                                                            local.get 22
                                                            local.get 11
                                                            i32.const 1
                                                            i32.add
                                                            i32.const 65535
                                                            i32.and
                                                            i32.add
                                                            i32.load8_u
                                                            i32.or
                                                            local.set 27
                                                            local.get 29
                                                            local.get 22
                                                            local.get 11
                                                            i32.const 65535
                                                            i32.and
                                                            i32.add
                                                            i32.load8_u
                                                            local.tee 28
                                                            i32.load8_u offset=1050784
                                                            i32.const 31
                                                            i32.and
                                                            i32.const 256
                                                            i32.or
                                                            local.tee 14
                                                            i32.add
                                                            i32.load8_u
                                                            local.set 33
                                                            local.get 28
                                                            i32.load8_u offset=1051204
                                                            local.tee 34
                                                            i32.const 7
                                                            i32.and
                                                            i32.const 2
                                                            i32.shl
                                                            i32.load offset=1051136
                                                            local.set 35
                                                            local.get 25
                                                            local.get 14
                                                            i32.const 1
                                                            i32.shl
                                                            i32.add
                                                            i64.load16_u
                                                            local.set 36
                                                            local.get 16
                                                            i32.const 2
                                                            i32.lt_u
                                                            br_if 2 (;@26;)
                                                            local.get 16
                                                            i32.extend8_s
                                                            i32.const 0
                                                            i32.lt_s
                                                            br_if 1 (;@27;)
                                                            local.get 16
                                                            i32.const 1051476
                                                            i32.add
                                                            local.set 37
                                                            local.get 16
                                                            i32.const 1050144
                                                            i32.add
                                                            local.set 16
                                                            br 4 (;@24;)
                                                          end
                                                          local.get 11
                                                          i32.const 1
                                                          i32.add
                                                          local.set 14
                                                          local.get 18
                                                          i32.const 1
                                                          i32.shr_u
                                                          local.set 28
                                                          local.get 13
                                                          local.get 29
                                                          local.get 22
                                                          local.get 11
                                                          i32.const 65535
                                                          i32.and
                                                          i32.add
                                                          i32.load8_u
                                                          local.tee 16
                                                          i32.add
                                                          i32.load8_u
                                                          i32.add
                                                          local.set 27
                                                          local.get 25
                                                          local.get 16
                                                          i32.const 1
                                                          i32.shl
                                                          i32.add
                                                          i64.load16_u
                                                          local.get 13
                                                          i64.extend_i32_u
                                                          i64.shl
                                                          local.get 31
                                                          i64.or
                                                          local.set 31
                                                          block ;; label = @28
                                                            local.get 18
                                                            i32.const 2
                                                            i32.and
                                                            i32.eqz
                                                            br_if 0 (;@28;)
                                                            local.get 28
                                                            local.set 18
                                                            br 5 (;@23;)
                                                          end
                                                          block ;; label = @28
                                                            local.get 14
                                                            local.get 26
                                                            i32.lt_u
                                                            br_if 0 (;@28;)
                                                            local.get 28
                                                            local.set 18
                                                            br 5 (;@23;)
                                                          end
                                                          local.get 11
                                                          i32.const 2
                                                          i32.add
                                                          local.set 13
                                                          local.get 18
                                                          i32.const 2
                                                          i32.shr_u
                                                          local.set 28
                                                          local.get 27
                                                          local.get 29
                                                          local.get 22
                                                          local.get 14
                                                          i32.const 65535
                                                          i32.and
                                                          i32.add
                                                          i32.load8_u
                                                          local.tee 14
                                                          i32.add
                                                          i32.load8_u
                                                          i32.add
                                                          local.set 16
                                                          local.get 25
                                                          local.get 14
                                                          i32.const 1
                                                          i32.shl
                                                          i32.add
                                                          i64.load16_u
                                                          local.get 27
                                                          i64.extend_i32_u
                                                          i64.shl
                                                          local.get 31
                                                          i64.or
                                                          local.set 31
                                                          local.get 18
                                                          i32.const 4
                                                          i32.and
                                                          br_if 2 (;@25;)
                                                          local.get 13
                                                          local.get 26
                                                          i32.ge_u
                                                          br_if 2 (;@25;)
                                                          local.get 11
                                                          i32.const 3
                                                          i32.add
                                                          local.set 14
                                                          local.get 18
                                                          i32.const 3
                                                          i32.shr_u
                                                          local.set 18
                                                          local.get 16
                                                          local.get 29
                                                          local.get 22
                                                          local.get 13
                                                          i32.const 65535
                                                          i32.and
                                                          i32.add
                                                          i32.load8_u
                                                          local.tee 11
                                                          i32.add
                                                          i32.load8_u
                                                          i32.add
                                                          local.set 27
                                                          local.get 25
                                                          local.get 11
                                                          i32.const 1
                                                          i32.shl
                                                          i32.add
                                                          i64.load16_u
                                                          local.get 16
                                                          i64.extend_i32_u
                                                          i64.shl
                                                          local.get 31
                                                          i64.or
                                                          local.set 31
                                                          br 4 (;@23;)
                                                        end
                                                        local.get 16
                                                        i32.const 128
                                                        i32.const 1051460
                                                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                                        unreachable
                                                      end
                                                      local.get 27
                                                      i32.const 1050272
                                                      i32.add
                                                      local.set 16
                                                      local.get 27
                                                      i32.const 2
                                                      i32.shr_u
                                                      i32.const 1051604
                                                      i32.add
                                                      local.set 37
                                                      br 1 (;@24;)
                                                    end
                                                    local.get 16
                                                    local.set 27
                                                    local.get 13
                                                    local.set 14
                                                    local.get 28
                                                    local.set 18
                                                    br 1 (;@23;)
                                                  end
                                                  local.get 11
                                                  i32.const 3
                                                  i32.add
                                                  local.set 14
                                                  local.get 18
                                                  i32.const 1
                                                  i32.shr_u
                                                  local.set 18
                                                  local.get 36
                                                  local.get 13
                                                  i64.extend_i32_u
                                                  i64.shl
                                                  local.get 35
                                                  local.get 28
                                                  i32.and
                                                  i64.extend_i32_u
                                                  local.get 13
                                                  local.get 33
                                                  i32.add
                                                  local.tee 11
                                                  i64.extend_i32_u
                                                  i64.shl
                                                  i64.or
                                                  local.get 32
                                                  local.get 16
                                                  i32.load8_u
                                                  local.tee 13
                                                  i32.const 1
                                                  i32.shl
                                                  i32.add
                                                  i64.load16_u
                                                  local.get 11
                                                  local.get 34
                                                  i32.add
                                                  local.tee 11
                                                  i64.extend_i32_u
                                                  i64.shl
                                                  i64.or
                                                  local.get 37
                                                  i32.load8_u
                                                  local.tee 16
                                                  i32.const 15
                                                  i32.and
                                                  i32.const 2
                                                  i32.shl
                                                  i32.load offset=1051136
                                                  local.get 27
                                                  i32.and
                                                  i64.extend_i32_u
                                                  local.get 11
                                                  local.get 30
                                                  local.get 13
                                                  i32.add
                                                  i32.load8_u
                                                  i32.add
                                                  local.tee 11
                                                  i64.extend_i32_u
                                                  i64.shl
                                                  i64.or
                                                  local.get 31
                                                  i64.or
                                                  local.set 31
                                                  local.get 11
                                                  local.get 16
                                                  i32.add
                                                  local.set 27
                                                end
                                                block ;; label = @23
                                                  local.get 10
                                                  i32.const 85172
                                                  i32.gt_u
                                                  br_if 0 (;@23;)
                                                  local.get 9
                                                  local.get 10
                                                  i32.add
                                                  local.get 31
                                                  i64.store align=1
                                                  i32.const 1
                                                  local.set 16
                                                  local.get 27
                                                  i32.const 3
                                                  i32.shr_u
                                                  local.get 10
                                                  i32.add
                                                  local.tee 10
                                                  i32.const 85180
                                                  i32.gt_u
                                                  br_if 22 (;@1;)
                                                  local.get 27
                                                  i32.const 7
                                                  i32.and
                                                  local.set 13
                                                  local.get 31
                                                  local.get 27
                                                  i32.const 56
                                                  i32.and
                                                  i64.extend_i32_u
                                                  i64.shr_u
                                                  local.set 31
                                                  local.get 14
                                                  local.get 26
                                                  i32.lt_u
                                                  br_if 1 (;@22;)
                                                  br 8 (;@15;)
                                                end
                                              end
                                              local.get 10
                                              local.get 10
                                              i32.const 8
                                              i32.add
                                              i32.const 85180
                                              i32.const 1049632
                                              call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                                              unreachable
                                            end
                                            local.get 16
                                            i32.const 85180
                                            i32.const 1049616
                                            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                            unreachable
                                          end
                                          i32.const 1049552
                                          i32.const 48
                                          i32.const 1049600
                                          call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                                          unreachable
                                        end
                                        local.get 16
                                        i32.const 85180
                                        i32.const 1049616
                                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                        unreachable
                                      end
                                      i32.const 1049552
                                      i32.const 48
                                      i32.const 1049600
                                      call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                                      unreachable
                                    end
                                    i32.const 1049828
                                    i32.const 43
                                    i32.const 1049872
                                    call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                                    unreachable
                                  end
                                  local.get 18
                                  i32.const 320
                                  i32.const 1049812
                                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                  unreachable
                                end
                                block ;; label = @15
                                  block ;; label = @16
                                    local.get 13
                                    br_if 0 (;@16;)
                                    i32.const 0
                                    local.set 11
                                    i32.const 0
                                    local.set 16
                                    br 1 (;@15;)
                                  end
                                  i32.const 0
                                  local.set 16
                                  i32.const 0
                                  local.set 11
                                  block ;; label = @16
                                    block ;; label = @17
                                      loop ;; label = @18
                                        local.get 13
                                        i32.const 16
                                        local.get 13
                                        i32.const 16
                                        i32.lt_u
                                        select
                                        local.tee 14
                                        i32.const 2
                                        i32.shl
                                        i32.load offset=1051136
                                        local.get 31
                                        i32.wrap_i64
                                        i32.and
                                        local.tee 18
                                        local.get 14
                                        i32.shr_u
                                        br_if 1 (;@17;)
                                        local.get 18
                                        local.get 16
                                        i32.shl
                                        local.get 11
                                        i32.or
                                        local.set 11
                                        block ;; label = @19
                                          local.get 16
                                          local.get 14
                                          i32.add
                                          local.tee 16
                                          i32.const 8
                                          i32.lt_u
                                          br_if 0 (;@19;)
                                          local.get 10
                                          i32.const 85180
                                          local.get 10
                                          i32.const 85180
                                          i32.gt_u
                                          select
                                          local.set 18
                                          loop ;; label = @20
                                            local.get 18
                                            local.get 10
                                            i32.eq
                                            br_if 4 (;@16;)
                                            local.get 9
                                            local.get 10
                                            i32.add
                                            local.get 11
                                            i32.store8
                                            local.get 11
                                            i32.const 8
                                            i32.shr_u
                                            local.set 11
                                            local.get 10
                                            i32.const 1
                                            i32.add
                                            local.set 10
                                            local.get 16
                                            i32.const -8
                                            i32.add
                                            local.tee 16
                                            i32.const 7
                                            i32.gt_u
                                            br_if 0 (;@20;)
                                          end
                                        end
                                        local.get 31
                                        local.get 14
                                        i64.extend_i32_u
                                        i64.shr_u
                                        local.set 31
                                        local.get 13
                                        local.get 14
                                        i32.sub
                                        local.tee 13
                                        i32.eqz
                                        br_if 3 (;@15;)
                                        br 0 (;@18;)
                                      end
                                    end
                                    i32.const 1049552
                                    i32.const 48
                                    i32.const 1049600
                                    call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                                    unreachable
                                  end
                                  local.get 18
                                  i32.const 85180
                                  i32.const 1049616
                                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                  unreachable
                                end
                                local.get 24
                                i32.load16_u offset=2240
                                local.tee 13
                                local.get 24
                                i32.load8_u offset=3712
                                local.tee 18
                                i32.shr_u
                                br_if 4 (;@10;)
                                local.get 13
                                local.get 16
                                i32.shl
                                local.get 11
                                i32.or
                                local.set 11
                                local.get 16
                                local.get 18
                                i32.add
                                local.tee 13
                                i32.const 8
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 10
                                i32.const 85180
                                local.get 10
                                i32.const 85180
                                i32.gt_u
                                select
                                local.set 16
                                loop ;; label = @15
                                  local.get 16
                                  local.get 10
                                  i32.eq
                                  br_if 2 (;@13;)
                                  local.get 9
                                  local.get 10
                                  i32.add
                                  local.get 11
                                  i32.store8
                                  local.get 11
                                  i32.const 8
                                  i32.shr_u
                                  local.set 11
                                  local.get 10
                                  i32.const 1
                                  i32.add
                                  local.set 10
                                  local.get 13
                                  i32.const -8
                                  i32.add
                                  local.tee 13
                                  i32.const 7
                                  i32.gt_u
                                  br_if 0 (;@15;)
                                end
                              end
                              block ;; label = @14
                                local.get 20
                                i32.const 33
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 10
                                local.get 23
                                i32.sub
                                i32.const 1
                                i32.add
                                local.get 20
                                i32.ge_u
                                br_if 2 (;@12;)
                              end
                              local.get 21
                              br_if 2 (;@11;)
                              br 11 (;@2;)
                            end
                            local.get 16
                            i32.const 85180
                            i32.const 1049616
                            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                            unreachable
                          end
                          local.get 21
                          i32.const -1
                          i32.xor
                          local.get 1
                          i32.load offset=112
                          local.get 1
                          i32.load offset=104
                          i32.sub
                          local.get 1
                          i32.load offset=116
                          i32.gt_u
                          i32.and
                          br_if 9 (;@2;)
                        end
                        block ;; label = @11
                          block ;; label = @12
                            local.get 19
                            i32.const 6
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 23
                            i32.const 85179
                            i32.gt_u
                            br_if 9 (;@3;)
                            local.get 9
                            local.get 23
                            i32.add
                            local.get 17
                            i32.store8
                            local.get 23
                            i32.const 1
                            i32.add
                            local.set 23
                            local.get 17
                            i32.const 8
                            i32.shr_u
                            local.set 17
                            local.get 19
                            i32.const 6
                            i32.eq
                            br_if 1 (;@11;)
                          end
                          local.get 23
                          i32.const 85180
                          i32.ge_u
                          br_if 2 (;@9;)
                          local.get 9
                          local.get 23
                          i32.add
                          local.get 17
                          i32.store8
                          local.get 23
                          i32.const 1
                          i32.add
                          local.set 23
                          local.get 17
                          i32.const 8
                          i32.shr_u
                          local.set 17
                        end
                        local.get 23
                        i32.const 85179
                        i32.gt_u
                        br_if 2 (;@8;)
                        local.get 9
                        local.get 23
                        i32.add
                        local.tee 13
                        local.get 17
                        local.get 20
                        i32.or
                        local.tee 10
                        i32.store8
                        local.get 23
                        i32.const 1
                        i32.add
                        local.tee 11
                        i32.const 85180
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 9
                        local.get 11
                        i32.add
                        local.get 10
                        i32.const 8
                        i32.shr_u
                        i32.store8
                        local.get 23
                        i32.const 85177
                        i32.gt_u
                        br_if 3 (;@7;)
                        local.get 13
                        local.get 20
                        i32.const 65535
                        i32.xor
                        local.tee 10
                        local.get 17
                        i32.const 16
                        i32.shr_u
                        i32.or
                        i32.store8 offset=2
                        local.get 23
                        i32.const 3
                        i32.add
                        local.tee 11
                        i32.const 85180
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 9
                        local.get 11
                        i32.add
                        local.get 10
                        i32.const 8
                        i32.shr_u
                        i32.store8
                        local.get 23
                        i32.const 4
                        i32.add
                        local.set 13
                        block ;; label = @11
                          local.get 1
                          i32.load offset=104
                          local.tee 10
                          local.get 20
                          i32.add
                          i32.const 32767
                          i32.and
                          local.tee 16
                          local.get 10
                          i32.const 32767
                          i32.and
                          local.tee 17
                          i32.gt_u
                          br_if 0 (;@11;)
                          i32.const 0
                          local.set 11
                          block ;; label = @12
                            local.get 20
                            br_if 0 (;@12;)
                            local.get 13
                            local.set 10
                            i32.const 0
                            local.set 13
                            br 10 (;@2;)
                          end
                          i32.const 32768
                          local.get 17
                          i32.sub
                          local.tee 10
                          local.get 13
                          i32.add
                          local.tee 18
                          i32.const 85181
                          i32.ge_u
                          br_if 5 (;@6;)
                          local.get 1
                          i32.load offset=84
                          local.set 19
                          block ;; label = @12
                            local.get 10
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 9
                            local.get 13
                            i32.add
                            local.get 19
                            local.get 17
                            i32.add
                            local.get 10
                            memory.copy
                          end
                          local.get 18
                          local.get 16
                          i32.add
                          local.tee 10
                          i32.const 85181
                          i32.ge_u
                          br_if 6 (;@5;)
                          block ;; label = @12
                            local.get 16
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 9
                            local.get 18
                            i32.add
                            local.get 19
                            local.get 16
                            memory.copy
                          end
                          i32.const 0
                          local.set 13
                          br 9 (;@2;)
                        end
                        local.get 16
                        local.get 17
                        i32.sub
                        local.tee 11
                        local.get 13
                        i32.add
                        local.tee 10
                        i32.const 85181
                        i32.ge_u
                        br_if 6 (;@4;)
                        block ;; label = @11
                          local.get 11
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 9
                          local.get 13
                          i32.add
                          local.get 1
                          i32.load offset=84
                          local.get 17
                          i32.add
                          local.get 11
                          memory.copy
                        end
                        i32.const 0
                        local.set 11
                        i32.const 0
                        local.set 13
                        br 8 (;@2;)
                      end
                      i32.const 1049552
                      i32.const 48
                      i32.const 1049600
                      call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                      unreachable
                    end
                    local.get 23
                    i32.const 85180
                    i32.const 1049616
                    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                    unreachable
                  end
                  i32.const 85180
                  i32.const 85180
                  i32.const 1049616
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                i32.const 85180
                i32.const 85180
                i32.const 1049616
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 13
              local.get 18
              i32.const 85180
              i32.const 1049536
              call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
              unreachable
            end
            local.get 18
            local.get 10
            i32.const 85180
            i32.const 1049536
            call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
            unreachable
          end
          local.get 13
          local.get 10
          i32.const 85180
          i32.const 1049536
          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
          unreachable
        end
        local.get 23
        i32.const 85180
        i32.const 1049616
        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
        unreachable
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        block ;; label = @19
                                          local.get 3
                                          i32.const 255
                                          i32.and
                                          br_table 17 (;@2;) 0 (;@19;) 1 (;@18;) 1 (;@18;) 4 (;@15;) 2 (;@17;) 3 (;@16;) 17 (;@2;) 17 (;@2;)
                                        end
                                        i32.const 2
                                        local.get 13
                                        i32.shl
                                        local.get 11
                                        i32.or
                                        local.set 11
                                        block ;; label = @19
                                          local.get 13
                                          i32.const 10
                                          i32.add
                                          local.tee 16
                                          i32.const 8
                                          i32.ge_u
                                          br_if 0 (;@19;)
                                          local.get 16
                                          local.set 13
                                          br 17 (;@2;)
                                        end
                                        local.get 10
                                        i32.const 85180
                                        local.get 10
                                        i32.const 85180
                                        i32.gt_u
                                        select
                                        local.set 16
                                        block ;; label = @19
                                          local.get 10
                                          i32.const 85179
                                          i32.gt_u
                                          br_if 0 (;@19;)
                                          local.get 13
                                          i32.const 2
                                          i32.add
                                          local.set 13
                                          block ;; label = @20
                                            loop ;; label = @21
                                              local.get 9
                                              local.get 10
                                              i32.add
                                              local.get 11
                                              i32.store8
                                              local.get 11
                                              i32.const 8
                                              i32.shr_u
                                              local.set 11
                                              local.get 13
                                              i32.const 7
                                              i32.le_u
                                              br_if 1 (;@20;)
                                              local.get 13
                                              i32.const -8
                                              i32.add
                                              local.set 13
                                              local.get 16
                                              local.get 10
                                              i32.const 1
                                              i32.add
                                              local.tee 10
                                              i32.eq
                                              br_if 2 (;@19;)
                                              br 0 (;@21;)
                                            end
                                          end
                                          local.get 10
                                          i32.const 1
                                          i32.add
                                          local.set 10
                                          br 17 (;@2;)
                                        end
                                        local.get 16
                                        i32.const 85180
                                        i32.const 1049616
                                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                        unreachable
                                      end
                                      block ;; label = @18
                                        local.get 13
                                        i32.const 3
                                        i32.add
                                        local.tee 16
                                        i32.const 8
                                        i32.lt_u
                                        br_if 0 (;@18;)
                                        local.get 10
                                        i32.const 85180
                                        local.get 10
                                        i32.const 85180
                                        i32.gt_u
                                        select
                                        local.set 17
                                        local.get 10
                                        i32.const 85179
                                        i32.gt_u
                                        br_if 6 (;@12;)
                                        local.get 13
                                        i32.const -5
                                        i32.add
                                        local.set 16
                                        block ;; label = @19
                                          loop ;; label = @20
                                            local.get 9
                                            local.get 10
                                            i32.add
                                            local.get 11
                                            i32.store8
                                            local.get 11
                                            i32.const 8
                                            i32.shr_u
                                            local.set 11
                                            local.get 16
                                            i32.const 7
                                            i32.le_u
                                            br_if 1 (;@19;)
                                            local.get 16
                                            i32.const -8
                                            i32.add
                                            local.set 16
                                            local.get 17
                                            local.get 10
                                            i32.const 1
                                            i32.add
                                            local.tee 10
                                            i32.eq
                                            br_if 8 (;@12;)
                                            br 0 (;@20;)
                                          end
                                        end
                                        local.get 10
                                        i32.const 1
                                        i32.add
                                        local.set 10
                                      end
                                      block ;; label = @18
                                        local.get 16
                                        i32.eqz
                                        br_if 0 (;@18;)
                                        local.get 10
                                        i32.const 85180
                                        i32.ge_u
                                        br_if 7 (;@11;)
                                        local.get 9
                                        local.get 10
                                        i32.add
                                        local.get 11
                                        i32.store8
                                        local.get 10
                                        i32.const 1
                                        i32.add
                                        local.set 10
                                        local.get 11
                                        i32.const 8
                                        i32.shr_u
                                        local.set 11
                                      end
                                      local.get 10
                                      i32.const 85179
                                      i32.gt_u
                                      br_if 7 (;@10;)
                                      local.get 9
                                      local.get 10
                                      i32.add
                                      local.tee 16
                                      local.get 11
                                      i32.store8
                                      local.get 10
                                      i32.const 1
                                      i32.add
                                      local.tee 13
                                      i32.const 85180
                                      i32.eq
                                      br_if 7 (;@10;)
                                      local.get 9
                                      local.get 13
                                      i32.add
                                      local.get 11
                                      i32.const 8
                                      i32.shr_u
                                      i32.store8
                                      block ;; label = @18
                                        local.get 10
                                        i32.const 85177
                                        i32.gt_u
                                        br_if 0 (;@18;)
                                        local.get 16
                                        i32.const 255
                                        i32.store8 offset=2
                                        local.get 10
                                        i32.const 3
                                        i32.add
                                        local.tee 11
                                        i32.const 85180
                                        i32.ne
                                        br_if 14 (;@4;)
                                      end
                                      i32.const 85180
                                      i32.const 85180
                                      i32.const 1049616
                                      call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                      unreachable
                                    end
                                    local.get 13
                                    br_if 2 (;@14;)
                                    i32.const 0
                                    local.set 13
                                    br 14 (;@2;)
                                  end
                                  local.get 13
                                  br_if 2 (;@13;)
                                  i32.const 0
                                  local.set 13
                                  br 13 (;@2;)
                                end
                                block ;; label = @15
                                  local.get 13
                                  i32.eqz
                                  br_if 0 (;@15;)
                                  local.get 10
                                  i32.const 85180
                                  i32.ge_u
                                  br_if 6 (;@9;)
                                  local.get 9
                                  local.get 10
                                  i32.add
                                  local.get 11
                                  i32.store8
                                  local.get 10
                                  i32.const 1
                                  i32.add
                                  local.set 10
                                  local.get 11
                                  i32.const 8
                                  i32.shr_u
                                  local.set 11
                                end
                                block ;; label = @15
                                  local.get 15
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.set 13
                                  br 13 (;@2;)
                                end
                                block ;; label = @15
                                  local.get 10
                                  i32.const 85180
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 10
                                  local.set 16
                                  br 9 (;@6;)
                                end
                                local.get 9
                                local.get 10
                                i32.add
                                local.tee 13
                                local.get 1
                                i32.load offset=32
                                local.tee 9
                                i32.const 24
                                i32.shr_u
                                local.get 11
                                i32.or
                                i32.store8
                                i32.const 85180
                                local.set 16
                                local.get 10
                                i32.const 85179
                                i32.eq
                                br_if 8 (;@6;)
                                local.get 13
                                local.get 9
                                i32.const 16
                                i32.shr_u
                                local.get 11
                                i32.const 8
                                i32.shr_u
                                i32.or
                                i32.store8 offset=1
                                i32.const 85180
                                local.set 16
                                local.get 10
                                i32.const 85177
                                i32.gt_u
                                br_if 8 (;@6;)
                                local.get 13
                                local.get 9
                                i32.const 8
                                i32.shr_u
                                local.get 11
                                i32.const 16
                                i32.shr_u
                                i32.or
                                i32.store8 offset=2
                                i32.const 85180
                                local.set 16
                                local.get 10
                                i32.const 85177
                                i32.eq
                                br_if 8 (;@6;)
                                local.get 13
                                local.get 9
                                local.get 11
                                i32.const 24
                                i32.shr_u
                                i32.or
                                i32.store8 offset=3
                                local.get 10
                                i32.const 4
                                i32.add
                                local.set 10
                                br 11 (;@3;)
                              end
                              i32.const 2
                              local.get 13
                              i32.shl
                              local.get 11
                              i32.or
                              local.set 11
                              block ;; label = @14
                                local.get 13
                                i32.const 10
                                i32.add
                                local.tee 16
                                i32.const 8
                                i32.ge_u
                                br_if 0 (;@14;)
                                local.get 16
                                local.set 13
                                br 12 (;@2;)
                              end
                              local.get 10
                              i32.const 85180
                              local.get 10
                              i32.const 85180
                              i32.gt_u
                              select
                              local.set 16
                              block ;; label = @14
                                local.get 10
                                i32.const 85179
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 13
                                i32.const 2
                                i32.add
                                local.set 13
                                loop ;; label = @15
                                  local.get 9
                                  local.get 10
                                  i32.add
                                  local.get 11
                                  i32.store8
                                  local.get 11
                                  i32.const 8
                                  i32.shr_u
                                  local.set 11
                                  local.get 13
                                  i32.const 7
                                  i32.le_u
                                  br_if 10 (;@5;)
                                  local.get 13
                                  i32.const -8
                                  i32.add
                                  local.set 13
                                  local.get 16
                                  local.get 10
                                  i32.const 1
                                  i32.add
                                  local.tee 10
                                  i32.ne
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 16
                              i32.const 85180
                              i32.const 1049616
                              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                              unreachable
                            end
                            block ;; label = @13
                              local.get 13
                              i32.const 3
                              i32.add
                              local.tee 16
                              i32.const 8
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 10
                              i32.const 85180
                              local.get 10
                              i32.const 85180
                              i32.gt_u
                              select
                              local.set 17
                              local.get 10
                              i32.const 85179
                              i32.gt_u
                              br_if 5 (;@8;)
                              local.get 13
                              i32.const -5
                              i32.add
                              local.set 16
                              block ;; label = @14
                                loop ;; label = @15
                                  local.get 9
                                  local.get 10
                                  i32.add
                                  local.get 11
                                  i32.store8
                                  local.get 11
                                  i32.const 8
                                  i32.shr_u
                                  local.set 11
                                  local.get 16
                                  i32.const 7
                                  i32.le_u
                                  br_if 1 (;@14;)
                                  local.get 16
                                  i32.const -8
                                  i32.add
                                  local.set 16
                                  local.get 17
                                  local.get 10
                                  i32.const 1
                                  i32.add
                                  local.tee 10
                                  i32.eq
                                  br_if 7 (;@8;)
                                  br 0 (;@15;)
                                end
                              end
                              local.get 10
                              i32.const 1
                              i32.add
                              local.set 10
                            end
                            block ;; label = @13
                              local.get 16
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 10
                              i32.const 85180
                              i32.ge_u
                              br_if 6 (;@7;)
                              local.get 9
                              local.get 10
                              i32.add
                              local.get 11
                              i32.store8
                              local.get 10
                              i32.const 1
                              i32.add
                              local.set 10
                              local.get 11
                              i32.const 8
                              i32.shr_u
                              local.set 11
                            end
                            block ;; label = @13
                              block ;; label = @14
                                local.get 10
                                i32.const 85179
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 9
                                local.get 10
                                i32.add
                                local.tee 16
                                local.get 11
                                i32.store8
                                local.get 10
                                i32.const 1
                                i32.add
                                local.tee 13
                                i32.const 85180
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 9
                                local.get 13
                                i32.add
                                local.get 11
                                i32.const 8
                                i32.shr_u
                                i32.store8
                                local.get 10
                                i32.const 85177
                                i32.gt_u
                                br_if 1 (;@13;)
                                local.get 16
                                i32.const 255
                                i32.store8 offset=2
                                local.get 10
                                i32.const 3
                                i32.add
                                local.tee 11
                                i32.const 85180
                                i32.eq
                                br_if 1 (;@13;)
                                local.get 9
                                local.get 11
                                i32.add
                                i32.const 255
                                i32.store8
                                local.get 10
                                i32.const 4
                                i32.add
                                local.set 10
                                br 11 (;@3;)
                              end
                              local.get 10
                              i32.const 85180
                              local.get 10
                              i32.const 85180
                              i32.gt_u
                              select
                              i32.const 85180
                              i32.const 1049616
                              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                              unreachable
                            end
                            i32.const 85180
                            i32.const 85180
                            i32.const 1049616
                            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                            unreachable
                          end
                          local.get 17
                          i32.const 85180
                          i32.const 1049616
                          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                          unreachable
                        end
                        local.get 10
                        i32.const 85180
                        i32.const 1049616
                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                        unreachable
                      end
                      local.get 10
                      i32.const 85180
                      local.get 10
                      i32.const 85180
                      i32.gt_u
                      select
                      i32.const 85180
                      i32.const 1049616
                      call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                      unreachable
                    end
                    local.get 10
                    i32.const 85180
                    i32.const 1049616
                    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                    unreachable
                  end
                  local.get 17
                  i32.const 85180
                  i32.const 1049616
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 10
                i32.const 85180
                i32.const 1049616
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 16
              i32.const 85180
              i32.const 1049616
              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
              unreachable
            end
            local.get 10
            i32.const 1
            i32.add
            local.set 10
            br 2 (;@2;)
          end
          local.get 9
          local.get 11
          i32.add
          i32.const 255
          i32.store8
          local.get 10
          i32.const 4
          i32.add
          local.set 10
        end
        i32.const 0
        local.set 11
        i32.const 0
        local.set 13
      end
      i32.const 0
      local.set 16
      local.get 1
      i32.load offset=80
      i32.const 0
      i32.const 640
      memory.fill
      local.get 1
      i64.const 1
      i64.store offset=64 align=4
      local.get 1
      i64.const 34359738368
      i64.store offset=72 align=4
      local.get 1
      local.get 13
      i32.store offset=48
      local.get 1
      local.get 11
      i32.store offset=44
      local.get 1
      local.get 12
      i32.const 1
      i32.add
      i32.store offset=12
      local.get 1
      local.get 1
      i32.load offset=104
      local.get 20
      i32.add
      i32.store offset=104
      block ;; label = @2
        local.get 10
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=28
        local.set 11
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 2
        i32.load offset=20
        local.tee 11
        i32.eqz
        br_if 0 (;@2;)
        local.get 11
        local.get 1
        i32.load offset=36
        i32.store
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.eqz
                br_if 0 (;@6;)
                block ;; label = @7
                  local.get 5
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 10
                  local.get 6
                  i32.add
                  i32.store offset=40
                  br 5 (;@2;)
                end
                local.get 10
                local.get 2
                i32.load offset=8
                local.tee 16
                local.get 6
                i32.sub
                local.tee 9
                local.get 10
                local.get 9
                i32.lt_u
                select
                local.tee 11
                local.get 6
                i32.add
                local.tee 13
                local.get 11
                i32.lt_u
                br_if 1 (;@5;)
                local.get 13
                local.get 16
                i32.gt_u
                br_if 1 (;@5;)
                local.get 11
                i32.const 85197
                i32.ge_u
                br_if 2 (;@4;)
                block ;; label = @7
                  local.get 11
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.load offset=4
                  local.get 6
                  i32.add
                  local.get 7
                  local.get 11
                  memory.copy
                end
                local.get 1
                local.get 13
                i32.store offset=40
                local.get 10
                local.get 9
                i32.le_u
                br_if 4 (;@2;)
                local.get 1
                local.get 11
                i32.store offset=24
                local.get 1
                local.get 10
                local.get 11
                i32.sub
                i32.store offset=28
                br 4 (;@2;)
              end
              local.get 10
              i32.const 85197
              i32.ge_u
              br_if 2 (;@3;)
              local.get 2
              i32.load offset=4
              local.get 7
              local.get 10
              local.get 2
              i32.load offset=8
              i32.load offset=16
              call_indirect (type 0)
              br_if 3 (;@2;)
              i32.const -1
              local.set 11
              local.get 1
              i32.const -1
              i32.store
              i32.const 0
              local.set 16
              br 4 (;@1;)
            end
            local.get 6
            local.get 13
            local.get 16
            i32.const 1049504
            call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
            unreachable
          end
          i32.const 0
          local.get 11
          i32.const 85196
          i32.const 1049488
          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
          unreachable
        end
        i32.const 0
        local.get 10
        i32.const 85196
        i32.const 1049472
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      local.get 1
      i32.load offset=28
      local.set 11
      i32.const 0
      local.set 16
    end
    local.get 0
    local.get 11
    i32.store offset=4
    local.get 0
    local.get 16
    i32.store
    local.get 4
    i32.const 656
    i32.add
    global.set $__stack_pointer
  )

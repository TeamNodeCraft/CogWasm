(func $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core14compress_inner (;15;) (type 11) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 1
    i64.const 0
    i64.store offset=36 align=4
    local.get 1
    i32.load
    local.set 5
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.load8_u offset=54
            i32.const 4
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i32.store8 offset=54
            local.get 3
            i32.const 255
            i32.and
            i32.const 4
            i32.ne
            br_if 1 (;@3;)
            local.get 5
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 1
          local.get 3
          i32.store8 offset=54
          local.get 5
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        i64.const 4294967294
        i64.store align=4
        local.get 1
        i32.const -2
        i32.store
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load offset=28
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load8_u offset=53
                    i32.const 1
                    i32.and
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 1
                        i32.load offset=8
                        local.tee 3
                        i32.const 524288
                        i32.and
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 151551
                        i32.and
                        i32.const 16385
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 2
                        i32.load offset=12
                        local.tee 6
                        br_if 1 (;@9;)
                        i32.const 0
                        local.set 7
                        br 8 (;@2;)
                      end
                      local.get 1
                      local.get 2
                      call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate6stored15compress_stored
                      i32.eqz
                      br_if 3 (;@6;)
                      local.get 2
                      i32.load offset=16
                      local.set 7
                      local.get 2
                      i32.load offset=12
                      local.set 6
                      br 6 (;@3;)
                    end
                    local.get 1
                    i32.const 60
                    i32.add
                    local.set 8
                    local.get 1
                    i32.const 84
                    i32.add
                    local.set 9
                    local.get 1
                    i32.load offset=20
                    local.set 10
                    local.get 1
                    i32.load offset=16
                    local.set 11
                    local.get 1
                    i32.load8_u offset=56
                    local.set 12
                    local.get 1
                    i32.load offset=112
                    local.set 13
                    local.get 1
                    i32.load offset=108
                    local.set 14
                    local.get 2
                    i32.load offset=16
                    local.set 7
                    i32.const 0
                    local.set 15
                    loop ;; label = @9
                      block ;; label = @10
                        local.get 7
                        local.get 15
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 1
                        i32.load8_u offset=54
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 14
                        i32.eqz
                        br_if 6 (;@4;)
                      end
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 7
                            local.get 15
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 6
                            local.get 15
                            i32.add
                            local.set 5
                            block ;; label = @13
                              local.get 7
                              local.get 15
                              i32.sub
                              local.tee 3
                              i32.const 258
                              local.get 14
                              i32.sub
                              local.tee 16
                              local.get 3
                              local.get 16
                              i32.lt_u
                              select
                              local.tee 17
                              i32.eqz
                              br_if 0 (;@13;)
                              local.get 1
                              i32.load offset=116
                              local.get 14
                              i32.add
                              i32.const 1
                              i32.gt_u
                              br_if 2 (;@11;)
                            end
                            local.get 17
                            i32.eqz
                            br_if 2 (;@10;)
                            local.get 14
                            local.get 13
                            i32.add
                            local.set 18
                            i32.const 0
                            local.set 3
                            loop ;; label = @13
                              local.get 9
                              i32.load
                              local.get 18
                              local.get 3
                              i32.add
                              local.tee 19
                              i32.const 32767
                              i32.and
                              local.tee 16
                              i32.add
                              local.get 5
                              local.get 3
                              i32.add
                              i32.load8_u
                              local.tee 20
                              i32.store8
                              block ;; label = @14
                                local.get 16
                                i32.const 257
                                i32.ge_u
                                br_if 0 (;@14;)
                                local.get 9
                                i32.load
                                local.get 16
                                i32.add
                                local.get 20
                                i32.store8 offset=32768
                              end
                              block ;; label = @14
                                local.get 14
                                local.get 3
                                i32.add
                                local.get 1
                                i32.load offset=116
                                i32.add
                                i32.const 1
                                i32.add
                                i32.const 2
                                i32.le_u
                                br_if 0 (;@14;)
                                local.get 1
                                i32.load offset=88
                                local.get 19
                                i32.const -2
                                i32.add
                                local.tee 16
                                i32.const 32767
                                i32.and
                                local.tee 21
                                i32.const 1
                                i32.shl
                                i32.add
                                local.get 1
                                i32.load offset=92
                                local.get 1
                                i32.load offset=84
                                local.tee 22
                                local.get 19
                                i32.const -1
                                i32.add
                                i32.const 32767
                                i32.and
                                i32.add
                                i32.load8_u
                                i32.const 5
                                i32.shl
                                local.get 20
                                i32.xor
                                local.get 22
                                local.get 21
                                i32.add
                                i32.load8_u
                                i32.const 10
                                i32.shl
                                i32.const 31744
                                i32.and
                                i32.xor
                                i32.const 1
                                i32.shl
                                local.tee 19
                                i32.add
                                i32.load16_u
                                i32.store16
                                local.get 1
                                i32.load offset=92
                                local.get 19
                                i32.add
                                local.get 16
                                i32.store16
                              end
                              local.get 17
                              local.get 3
                              i32.const 1
                              i32.add
                              local.tee 3
                              i32.ne
                              br_if 0 (;@13;)
                            end
                            local.get 14
                            local.get 3
                            i32.add
                            local.set 14
                            br 2 (;@10;)
                          end
                          local.get 15
                          local.get 7
                          local.get 7
                          i32.const 1051120
                          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                          unreachable
                        end
                        local.get 9
                        i32.load
                        local.tee 19
                        local.get 14
                        local.get 13
                        i32.add
                        local.tee 16
                        i32.const -2
                        i32.add
                        local.tee 3
                        i32.const 32767
                        i32.and
                        i32.add
                        i32.load8_u
                        i32.const 5
                        i32.shl
                        local.get 19
                        local.get 16
                        i32.const -1
                        i32.add
                        i32.const 32767
                        i32.and
                        i32.add
                        i32.load8_u
                        i32.xor
                        local.set 20
                        local.get 17
                        local.set 18
                        loop ;; label = @11
                          local.get 9
                          i32.load
                          local.get 16
                          i32.const 32767
                          i32.and
                          local.tee 16
                          i32.add
                          local.get 5
                          i32.load8_u
                          local.tee 19
                          i32.store8
                          block ;; label = @12
                            local.get 16
                            i32.const 257
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 9
                            i32.load
                            local.get 16
                            i32.add
                            local.get 19
                            i32.store8 offset=32768
                          end
                          local.get 5
                          i32.const 1
                          i32.add
                          local.set 5
                          local.get 1
                          i32.load offset=88
                          local.get 3
                          i32.const 32767
                          i32.and
                          i32.const 1
                          i32.shl
                          i32.add
                          local.get 1
                          i32.load offset=92
                          local.get 20
                          i32.const 5
                          i32.shl
                          i32.const 32736
                          i32.and
                          local.get 19
                          i32.const 255
                          i32.and
                          i32.xor
                          local.tee 20
                          i32.const 1
                          i32.shl
                          local.tee 19
                          i32.add
                          i32.load16_u
                          i32.store16
                          local.get 1
                          i32.load offset=92
                          local.get 19
                          i32.add
                          local.get 3
                          i32.store16
                          local.get 3
                          i32.const 1
                          i32.add
                          local.set 3
                          local.get 16
                          i32.const 1
                          i32.add
                          local.set 16
                          local.get 18
                          i32.const -1
                          i32.add
                          local.tee 18
                          br_if 0 (;@11;)
                        end
                        local.get 17
                        local.get 14
                        i32.add
                        local.set 14
                      end
                      local.get 1
                      i32.const 32768
                      local.get 14
                      i32.sub
                      local.tee 3
                      local.get 1
                      i32.load offset=116
                      local.tee 5
                      local.get 3
                      local.get 5
                      i32.lt_u
                      select
                      local.tee 16
                      i32.store offset=116
                      local.get 17
                      local.get 15
                      i32.add
                      local.set 15
                      block ;; label = @10
                        local.get 1
                        i32.load8_u offset=54
                        br_if 0 (;@10;)
                        local.get 14
                        i32.const 258
                        i32.lt_u
                        br_if 6 (;@4;)
                      end
                      local.get 13
                      i32.const 32767
                      i32.and
                      local.set 19
                      local.get 1
                      i32.load offset=8
                      local.tee 23
                      i32.const 65536
                      i32.and
                      local.set 24
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
                                                          local.get 10
                                                          br_if 0 (;@27;)
                                                          i32.const 2
                                                          local.set 3
                                                          local.get 24
                                                          i32.eqz
                                                          br_if 1 (;@26;)
                                                          local.get 16
                                                          br_if 3 (;@24;)
                                                          i32.const 0
                                                          local.set 22
                                                          i32.const 2
                                                          local.set 25
                                                          br 6 (;@21;)
                                                        end
                                                        local.get 10
                                                        local.set 3
                                                        local.get 24
                                                        br_if 1 (;@25;)
                                                      end
                                                      i32.const 0
                                                      local.set 22
                                                      local.get 14
                                                      i32.const 258
                                                      local.get 14
                                                      i32.const 258
                                                      i32.lt_u
                                                      select
                                                      local.tee 26
                                                      local.get 3
                                                      i32.le_u
                                                      br_if 3 (;@22;)
                                                      local.get 9
                                                      i32.const 12
                                                      i32.const 16
                                                      local.get 3
                                                      i32.const 32
                                                      i32.lt_u
                                                      select
                                                      i32.add
                                                      i32.load
                                                      i32.const -1
                                                      i32.add
                                                      local.tee 17
                                                      i32.eqz
                                                      br_if 3 (;@22;)
                                                      local.get 19
                                                      i32.const 2
                                                      i32.add
                                                      local.set 27
                                                      local.get 1
                                                      i32.load8_u offset=120
                                                      local.tee 28
                                                      i32.const 3
                                                      i32.shl
                                                      local.set 29
                                                      local.get 9
                                                      i32.load
                                                      local.tee 30
                                                      local.get 19
                                                      i32.add
                                                      local.tee 5
                                                      local.get 3
                                                      i32.add
                                                      i32.const -1
                                                      i32.add
                                                      i32.load16_u align=1
                                                      local.set 31
                                                      local.get 1
                                                      i32.load offset=88
                                                      local.set 20
                                                      i32.const 0
                                                      local.set 25
                                                      local.get 5
                                                      i32.load16_u align=1
                                                      i32.const 65535
                                                      i32.and
                                                      local.set 32
                                                      i32.const 0
                                                      local.set 22
                                                      local.get 19
                                                      local.set 5
                                                      loop ;; label = @26
                                                        local.get 3
                                                        i32.eqz
                                                        br_if 5 (;@21;)
                                                        local.get 30
                                                        local.get 3
                                                        i32.add
                                                        local.set 18
                                                        i32.const 1
                                                        local.get 17
                                                        i32.sub
                                                        local.set 33
                                                        loop ;; label = @27
                                                          local.get 20
                                                          local.get 5
                                                          i32.const 1
                                                          i32.shl
                                                          i32.add
                                                          i32.load16_u
                                                          local.tee 5
                                                          i32.eqz
                                                          br_if 5 (;@22;)
                                                          local.get 13
                                                          local.get 5
                                                          i32.sub
                                                          i32.const 65535
                                                          i32.and
                                                          local.tee 17
                                                          local.get 16
                                                          i32.gt_u
                                                          br_if 5 (;@22;)
                                                          block ;; label = @28
                                                            local.get 18
                                                            local.get 5
                                                            i32.const 32767
                                                            i32.and
                                                            local.tee 5
                                                            i32.add
                                                            i32.const -1
                                                            i32.add
                                                            i32.load16_u align=1
                                                            local.get 31
                                                            i32.const 65535
                                                            i32.and
                                                            local.tee 21
                                                            i32.eq
                                                            br_if 0 (;@28;)
                                                            local.get 20
                                                            local.get 5
                                                            i32.const 1
                                                            i32.shl
                                                            i32.add
                                                            i32.load16_u
                                                            local.tee 5
                                                            i32.eqz
                                                            br_if 6 (;@22;)
                                                            local.get 13
                                                            local.get 5
                                                            i32.sub
                                                            i32.const 65535
                                                            i32.and
                                                            local.tee 17
                                                            local.get 16
                                                            i32.gt_u
                                                            br_if 6 (;@22;)
                                                            local.get 18
                                                            local.get 5
                                                            i32.const 32767
                                                            i32.and
                                                            local.tee 5
                                                            i32.add
                                                            i32.const -1
                                                            i32.add
                                                            i32.load16_u align=1
                                                            local.get 21
                                                            i32.eq
                                                            br_if 0 (;@28;)
                                                            local.get 20
                                                            local.get 5
                                                            i32.const 1
                                                            i32.shl
                                                            i32.add
                                                            i32.load16_u
                                                            local.tee 5
                                                            i32.eqz
                                                            br_if 6 (;@22;)
                                                            local.get 13
                                                            local.get 5
                                                            i32.sub
                                                            i32.const 65535
                                                            i32.and
                                                            local.tee 17
                                                            local.get 16
                                                            i32.gt_u
                                                            br_if 6 (;@22;)
                                                            local.get 18
                                                            local.get 5
                                                            i32.const 32767
                                                            i32.and
                                                            local.tee 5
                                                            i32.add
                                                            i32.const -1
                                                            i32.add
                                                            i32.load16_u align=1
                                                            local.get 21
                                                            i32.eq
                                                            br_if 0 (;@28;)
                                                            local.get 33
                                                            i32.const 1
                                                            i32.add
                                                            local.tee 33
                                                            i32.const 1
                                                            i32.eq
                                                            br_if 6 (;@22;)
                                                            br 1 (;@27;)
                                                          end
                                                        end
                                                        local.get 17
                                                        i32.eqz
                                                        br_if 4 (;@22;)
                                                        block ;; label = @27
                                                          local.get 32
                                                          local.get 30
                                                          local.get 5
                                                          i32.add
                                                          i32.load16_u align=1
                                                          i32.ne
                                                          br_if 0 (;@27;)
                                                          local.get 28
                                                          i32.eqz
                                                          br_if 4 (;@23;)
                                                          local.get 5
                                                          i32.const 2
                                                          i32.add
                                                          local.set 21
                                                          i32.const 0
                                                          local.set 18
                                                          block ;; label = @28
                                                            loop ;; label = @29
                                                              local.get 30
                                                              local.get 27
                                                              local.get 18
                                                              i32.add
                                                              i32.const 32767
                                                              i32.and
                                                              i32.add
                                                              i64.load align=1
                                                              local.tee 34
                                                              local.get 30
                                                              local.get 21
                                                              local.get 18
                                                              i32.add
                                                              i32.const 32767
                                                              i32.and
                                                              i32.add
                                                              i64.load align=1
                                                              local.tee 35
                                                              i64.ne
                                                              br_if 1 (;@28;)
                                                              local.get 29
                                                              local.get 18
                                                              i32.const 8
                                                              i32.add
                                                              local.tee 18
                                                              i32.eq
                                                              br_if 6 (;@23;)
                                                              br 0 (;@29;)
                                                            end
                                                          end
                                                          local.get 35
                                                          local.get 34
                                                          i64.xor
                                                          i64.ctz
                                                          i32.wrap_i64
                                                          i32.const 3
                                                          i32.shr_u
                                                          local.get 18
                                                          i32.add
                                                          i32.const 2
                                                          i32.add
                                                          local.tee 18
                                                          local.get 3
                                                          i32.le_u
                                                          br_if 0 (;@27;)
                                                          local.get 26
                                                          local.get 18
                                                          local.get 26
                                                          local.get 18
                                                          i32.lt_u
                                                          select
                                                          local.set 3
                                                          block ;; label = @28
                                                            local.get 18
                                                            local.get 26
                                                            i32.lt_u
                                                            br_if 0 (;@28;)
                                                            local.get 17
                                                            local.set 22
                                                            br 6 (;@22;)
                                                          end
                                                          local.get 30
                                                          local.get 3
                                                          local.get 19
                                                          i32.add
                                                          local.tee 18
                                                          local.get 18
                                                          i32.const 0
                                                          i32.ne
                                                          i32.sub
                                                          i32.add
                                                          i32.load16_u align=1
                                                          local.set 31
                                                          local.get 17
                                                          local.set 22
                                                        end
                                                        i32.const 0
                                                        local.get 33
                                                        i32.sub
                                                        local.set 17
                                                        local.get 33
                                                        br_if 0 (;@26;)
                                                        br 4 (;@22;)
                                                      end
                                                    end
                                                    local.get 16
                                                    br_if 0 (;@24;)
                                                    i32.const 0
                                                    local.set 22
                                                    local.get 10
                                                    local.set 3
                                                    br 2 (;@22;)
                                                  end
                                                  block ;; label = @24
                                                    block ;; label = @25
                                                      local.get 14
                                                      local.get 19
                                                      i32.add
                                                      local.tee 3
                                                      local.get 19
                                                      i32.lt_u
                                                      br_if 0 (;@25;)
                                                      local.get 3
                                                      i32.const 33027
                                                      i32.lt_u
                                                      br_if 1 (;@24;)
                                                    end
                                                    local.get 19
                                                    local.get 3
                                                    i32.const 33026
                                                    i32.const 1051104
                                                    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                                                    unreachable
                                                  end
                                                  block ;; label = @24
                                                    local.get 14
                                                    i32.eqz
                                                    br_if 0 (;@24;)
                                                    local.get 9
                                                    i32.load
                                                    local.tee 16
                                                    local.get 19
                                                    i32.add
                                                    local.set 5
                                                    i32.const 0
                                                    local.set 3
                                                    local.get 16
                                                    local.get 13
                                                    i32.const -1
                                                    i32.add
                                                    i32.const 32767
                                                    i32.and
                                                    i32.add
                                                    i32.load8_u
                                                    i32.const 255
                                                    i32.and
                                                    local.set 16
                                                    block ;; label = @25
                                                      loop ;; label = @26
                                                        local.get 5
                                                        local.get 3
                                                        i32.add
                                                        i32.load8_u
                                                        local.get 16
                                                        i32.ne
                                                        br_if 1 (;@25;)
                                                        local.get 14
                                                        local.get 3
                                                        i32.const 1
                                                        i32.add
                                                        local.tee 3
                                                        i32.ne
                                                        br_if 0 (;@26;)
                                                      end
                                                      local.get 14
                                                      local.set 3
                                                    end
                                                    local.get 3
                                                    i32.const 2
                                                    i32.le_u
                                                    br_if 0 (;@24;)
                                                    i32.const 1
                                                    local.set 22
                                                    br 2 (;@22;)
                                                  end
                                                  i32.const 0
                                                  local.set 25
                                                  i32.const 0
                                                  local.set 22
                                                  br 2 (;@21;)
                                                end
                                                local.get 26
                                                local.set 3
                                                local.get 17
                                                local.set 22
                                              end
                                              local.get 3
                                              i32.const 3
                                              i32.eq
                                              br_if 1 (;@20;)
                                              local.get 3
                                              local.set 25
                                            end
                                            local.get 23
                                            i32.const 131072
                                            i32.and
                                            i32.eqz
                                            br_if 1 (;@19;)
                                            local.get 25
                                            i32.const 6
                                            i32.lt_u
                                            br_if 2 (;@18;)
                                            br 1 (;@19;)
                                          end
                                          local.get 23
                                          i32.const 131072
                                          i32.and
                                          br_if 1 (;@18;)
                                          i32.const 3
                                          local.set 25
                                          local.get 22
                                          i32.const 8192
                                          i32.ge_u
                                          br_if 1 (;@18;)
                                        end
                                        local.get 19
                                        local.get 22
                                        i32.eq
                                        br_if 0 (;@18;)
                                        local.get 10
                                        br_if 3 (;@15;)
                                        local.get 22
                                        i32.eqz
                                        br_if 1 (;@17;)
                                        block ;; label = @19
                                          local.get 1
                                          i32.load8_u offset=52
                                          br_if 0 (;@19;)
                                          local.get 24
                                          br_if 0 (;@19;)
                                          local.get 25
                                          i32.const 128
                                          i32.lt_u
                                          br_if 3 (;@16;)
                                        end
                                        local.get 1
                                        i32.load offset=80
                                        local.get 8
                                        local.get 25
                                        local.get 22
                                        call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core12record_match
                                        br 7 (;@11;)
                                      end
                                      local.get 10
                                      br_if 3 (;@14;)
                                    end
                                    local.get 1
                                    i32.load offset=80
                                    local.set 5
                                    local.get 1
                                    i32.load offset=60
                                    local.tee 16
                                    local.get 1
                                    i32.load offset=64
                                    local.tee 3
                                    i32.const 65535
                                    i32.and
                                    i32.add
                                    local.get 1
                                    i32.load offset=84
                                    local.get 19
                                    i32.add
                                    i32.load8_u
                                    local.tee 19
                                    i32.store8
                                    local.get 1
                                    local.get 1
                                    i32.load offset=72
                                    i32.const 1
                                    i32.add
                                    i32.store offset=72
                                    local.get 1
                                    local.get 3
                                    i32.const 1
                                    i32.add
                                    local.tee 18
                                    i32.store offset=64
                                    local.get 1
                                    local.get 1
                                    i32.load offset=76
                                    i32.const -1
                                    i32.add
                                    local.tee 20
                                    i32.store offset=76
                                    local.get 16
                                    local.get 1
                                    i32.load16_u offset=68
                                    i32.add
                                    local.tee 16
                                    local.get 16
                                    i32.load8_u
                                    i32.const 1
                                    i32.shr_u
                                    i32.store8
                                    block ;; label = @17
                                      local.get 20
                                      br_if 0 (;@17;)
                                      local.get 1
                                      local.get 18
                                      i32.store offset=68
                                      local.get 1
                                      i32.const 8
                                      i32.store offset=76
                                      local.get 1
                                      local.get 3
                                      i32.const 2
                                      i32.add
                                      i32.store offset=64
                                    end
                                    i32.const 1
                                    local.set 3
                                    local.get 5
                                    local.get 19
                                    i32.const 1
                                    i32.shl
                                    i32.add
                                    local.tee 5
                                    local.get 5
                                    i32.load16_u
                                    i32.const 1
                                    i32.add
                                    i32.store16
                                    i32.const 0
                                    local.set 10
                                    br 6 (;@10;)
                                  end
                                  local.get 9
                                  i32.load
                                  local.get 19
                                  i32.add
                                  i32.load8_u
                                  local.set 12
                                  br 3 (;@12;)
                                end
                                local.get 25
                                local.get 10
                                i32.gt_u
                                br_if 1 (;@13;)
                              end
                              local.get 1
                              i32.load offset=80
                              local.get 8
                              local.get 10
                              local.get 11
                              call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core12record_match
                              local.get 10
                              i32.const -1
                              i32.add
                              local.set 3
                              i32.const 0
                              local.set 10
                              br 3 (;@10;)
                            end
                            local.get 1
                            i32.load offset=80
                            local.set 5
                            local.get 1
                            i32.load offset=60
                            local.tee 16
                            local.get 1
                            i32.load offset=64
                            local.tee 3
                            i32.const 65535
                            i32.and
                            i32.add
                            local.get 12
                            i32.store8
                            local.get 1
                            local.get 1
                            i32.load offset=72
                            i32.const 1
                            i32.add
                            i32.store offset=72
                            local.get 1
                            local.get 3
                            i32.const 1
                            i32.add
                            local.tee 18
                            i32.store offset=64
                            local.get 1
                            local.get 1
                            i32.load offset=76
                            i32.const -1
                            i32.add
                            local.tee 20
                            i32.store offset=76
                            local.get 16
                            local.get 1
                            i32.load16_u offset=68
                            i32.add
                            local.tee 16
                            local.get 16
                            i32.load8_u
                            i32.const 1
                            i32.shr_u
                            i32.store8
                            block ;; label = @13
                              local.get 20
                              br_if 0 (;@13;)
                              local.get 1
                              local.get 18
                              i32.store offset=68
                              local.get 1
                              i32.const 8
                              i32.store offset=76
                              local.get 1
                              local.get 3
                              i32.const 2
                              i32.add
                              i32.store offset=64
                            end
                            local.get 5
                            local.get 12
                            i32.const 255
                            i32.and
                            i32.const 1
                            i32.shl
                            i32.add
                            local.tee 3
                            local.get 3
                            i32.load16_u
                            i32.const 1
                            i32.add
                            i32.store16
                            block ;; label = @13
                              local.get 25
                              i32.const 128
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 1
                              i32.load offset=80
                              local.get 8
                              local.get 25
                              local.get 22
                              call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core12record_match
                              br 2 (;@11;)
                            end
                            local.get 9
                            i32.load
                            local.get 19
                            i32.add
                            i32.load8_u
                            local.set 12
                          end
                          i32.const 1
                          local.set 3
                          local.get 25
                          local.set 10
                          local.get 22
                          local.set 11
                          br 1 (;@10;)
                        end
                        i32.const 0
                        local.set 10
                        local.get 25
                        local.set 3
                      end
                      local.get 1
                      local.get 1
                      i32.load offset=116
                      local.get 3
                      i32.add
                      local.tee 5
                      i32.const 32768
                      local.get 5
                      i32.const 32768
                      i32.lt_u
                      select
                      i32.store offset=116
                      local.get 14
                      local.get 3
                      i32.sub
                      local.set 14
                      local.get 3
                      local.get 13
                      i32.add
                      local.set 13
                      local.get 1
                      i32.load offset=64
                      local.set 3
                      block ;; label = @10
                        block ;; label = @11
                          local.get 1
                          i32.load offset=72
                          local.tee 5
                          i32.const 31744
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 3
                          i32.const 65528
                          i32.le_u
                          br_if 2 (;@9;)
                          br 1 (;@10;)
                        end
                        local.get 3
                        i32.const 65528
                        i32.gt_u
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 115
                        i32.mul
                        i32.const 7
                        i32.shr_u
                        local.get 5
                        i32.lt_u
                        br_if 1 (;@9;)
                      end
                      local.get 1
                      local.get 13
                      i32.store offset=112
                      local.get 1
                      local.get 14
                      i32.store offset=108
                      local.get 1
                      local.get 15
                      i32.store offset=36
                      local.get 4
                      i32.const 32
                      i32.add
                      local.get 1
                      local.get 2
                      i32.const 0
                      call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core11flush_block
                      block ;; label = @10
                        local.get 4
                        i32.load offset=32
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 10
                        i32.store offset=20
                        local.get 1
                        local.get 11
                        i32.store offset=16
                        local.get 1
                        local.get 12
                        i32.store8 offset=56
                        br 4 (;@6;)
                      end
                      local.get 4
                      i32.load offset=36
                      local.tee 3
                      i32.eqz
                      br_if 0 (;@9;)
                    end
                    local.get 1
                    local.get 10
                    i32.store offset=20
                    local.get 1
                    local.get 11
                    i32.store offset=16
                    local.get 1
                    local.get 12
                    i32.store8 offset=56
                    local.get 3
                    i32.const 0
                    i32.le_s
                    br_if 2 (;@6;)
                    br 5 (;@3;)
                  end
                  local.get 0
                  local.get 2
                  local.get 1
                  call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core19flush_output_buffer
                  local.get 1
                  local.get 0
                  i32.load
                  i32.store
                  br 6 (;@1;)
                end
                block ;; label = @7
                  local.get 2
                  i32.load offset=12
                  local.tee 6
                  br_if 0 (;@7;)
                  i32.const 0
                  local.set 7
                  br 5 (;@2;)
                end
                local.get 1
                i32.load offset=112
                local.tee 19
                i32.const 32767
                i32.and
                local.set 9
                local.get 2
                i32.load offset=16
                local.set 7
                local.get 1
                i32.load offset=108
                local.set 22
                i32.const 0
                local.set 20
                block ;; label = @7
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 7
                      local.get 20
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 22
                      i32.eqz
                      br_if 4 (;@5;)
                      local.get 1
                      i32.load8_u offset=54
                      i32.const 255
                      i32.and
                      i32.eqz
                      br_if 4 (;@5;)
                    end
                    block ;; label = @9
                      local.get 7
                      local.get 20
                      i32.sub
                      local.tee 3
                      i32.const 4096
                      local.get 22
                      i32.sub
                      local.tee 5
                      local.get 3
                      local.get 5
                      i32.lt_u
                      select
                      local.tee 13
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 19
                      local.get 22
                      i32.add
                      local.set 5
                      local.get 13
                      local.set 16
                      loop ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 32768
                            local.get 5
                            i32.const 32767
                            i32.and
                            local.tee 5
                            i32.sub
                            local.tee 3
                            local.get 16
                            local.get 3
                            local.get 16
                            i32.lt_u
                            select
                            local.tee 3
                            local.get 20
                            local.tee 18
                            i32.add
                            local.tee 20
                            local.get 3
                            i32.lt_u
                            br_if 0 (;@12;)
                            local.get 20
                            local.get 7
                            i32.le_u
                            br_if 1 (;@11;)
                          end
                          local.get 18
                          local.get 20
                          local.get 7
                          i32.const 1051072
                          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                          unreachable
                        end
                        local.get 6
                        local.get 18
                        i32.add
                        local.set 14
                        block ;; label = @11
                          local.get 3
                          i32.eqz
                          br_if 0 (;@11;)
                          local.get 1
                          i32.load offset=84
                          local.get 5
                          i32.add
                          local.get 14
                          local.get 3
                          memory.copy
                        end
                        block ;; label = @11
                          block ;; label = @12
                            local.get 5
                            i32.const 256
                            i32.gt_u
                            br_if 0 (;@12;)
                            local.get 3
                            i32.const 257
                            local.get 5
                            i32.sub
                            local.tee 17
                            local.get 3
                            local.get 17
                            i32.lt_u
                            select
                            local.tee 17
                            local.get 18
                            i32.add
                            local.tee 21
                            local.get 17
                            i32.lt_u
                            br_if 1 (;@11;)
                            local.get 21
                            local.get 7
                            i32.gt_u
                            br_if 1 (;@11;)
                            local.get 17
                            i32.eqz
                            br_if 0 (;@12;)
                            local.get 1
                            i32.load offset=84
                            local.get 5
                            i32.add
                            i32.const 32768
                            i32.add
                            local.get 14
                            local.get 17
                            memory.copy
                          end
                          local.get 3
                          local.get 5
                          i32.add
                          local.set 5
                          local.get 16
                          local.get 3
                          i32.sub
                          local.tee 16
                          br_if 1 (;@10;)
                          br 2 (;@9;)
                        end
                      end
                      local.get 18
                      local.get 21
                      local.get 7
                      i32.const 1051056
                      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                      unreachable
                    end
                    local.get 1
                    i32.const 32768
                    local.get 13
                    local.get 22
                    i32.add
                    local.tee 16
                    i32.sub
                    local.tee 3
                    local.get 1
                    i32.load offset=116
                    local.tee 5
                    local.get 3
                    local.get 5
                    i32.lt_u
                    select
                    i32.store offset=116
                    block ;; label = @9
                      local.get 16
                      i32.const 4095
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 1
                      i32.load8_u offset=54
                      i32.const 255
                      i32.and
                      br_if 0 (;@9;)
                      local.get 16
                      local.set 22
                      br 4 (;@5;)
                    end
                    block ;; label = @9
                      local.get 16
                      i32.const 4
                      i32.lt_u
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        local.get 1
                        i32.load offset=92
                        local.get 1
                        i32.load offset=84
                        local.get 9
                        i32.add
                        i32.load align=1
                        local.tee 22
                        i32.const 16777215
                        i32.and
                        local.tee 5
                        i32.const 17
                        i32.shr_u
                        local.get 22
                        i32.const 4095
                        i32.and
                        i32.xor
                        i32.const 1
                        i32.shl
                        i32.add
                        local.tee 3
                        i32.load16_u
                        local.set 13
                        local.get 3
                        local.get 19
                        i32.store16
                        block ;; label = @11
                          local.get 1
                          i32.load offset=116
                          local.get 19
                          local.get 13
                          i32.sub
                          local.tee 10
                          i32.const 65535
                          i32.and
                          local.tee 15
                          i32.lt_u
                          br_if 0 (;@11;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 5
                              local.get 1
                              i32.load offset=84
                              local.tee 3
                              local.get 13
                              i32.const 32767
                              i32.and
                              local.tee 21
                              i32.add
                              i32.load align=1
                              i32.const 16777215
                              i32.and
                              i32.eq
                              br_if 0 (;@13;)
                              local.get 1
                              i32.load offset=60
                              local.get 1
                              i32.load16_u offset=64
                              i32.add
                              local.get 22
                              i32.store8
                              i32.const 1
                              local.set 3
                              local.get 1
                              local.get 1
                              i32.load offset=64
                              i32.const 1
                              i32.add
                              i32.store offset=64
                              local.get 1
                              i32.load offset=60
                              local.get 1
                              i32.load16_u offset=68
                              i32.add
                              local.tee 5
                              local.get 5
                              i32.load8_u
                              i32.const 1
                              i32.shr_u
                              i32.store8
                              local.get 1
                              i32.load offset=80
                              local.get 22
                              i32.const 255
                              i32.and
                              i32.const 1
                              i32.shl
                              i32.add
                              local.tee 5
                              local.get 5
                              i32.load16_u
                              i32.const 1
                              i32.add
                              i32.store16
                              br 1 (;@12;)
                            end
                            i32.const -256
                            local.set 5
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        loop ;; label = @19
                                          local.get 3
                                          local.get 9
                                          local.get 5
                                          i32.add
                                          local.tee 14
                                          i32.const 259
                                          i32.add
                                          local.tee 18
                                          i32.const 32767
                                          i32.and
                                          i32.add
                                          i64.load align=1
                                          local.tee 34
                                          local.get 3
                                          local.get 21
                                          local.get 5
                                          i32.add
                                          local.tee 17
                                          i32.const 259
                                          i32.add
                                          i32.const 32767
                                          i32.and
                                          i32.add
                                          i64.load align=1
                                          local.tee 35
                                          i64.ne
                                          br_if 1 (;@18;)
                                          local.get 3
                                          local.get 14
                                          i32.const 267
                                          i32.add
                                          local.tee 18
                                          i32.const 32767
                                          i32.and
                                          i32.add
                                          i64.load align=1
                                          local.tee 34
                                          local.get 3
                                          local.get 17
                                          i32.const 267
                                          i32.add
                                          i32.const 32767
                                          i32.and
                                          i32.add
                                          i64.load align=1
                                          local.tee 35
                                          i64.ne
                                          br_if 1 (;@18;)
                                          local.get 5
                                          i32.const 16
                                          i32.add
                                          local.tee 5
                                          br_if 0 (;@19;)
                                        end
                                        i32.const 258
                                        local.set 3
                                        local.get 13
                                        i32.const 65535
                                        i32.and
                                        local.get 19
                                        i32.const 65535
                                        i32.and
                                        i32.ne
                                        br_if 1 (;@17;)
                                        br 2 (;@16;)
                                      end
                                      local.get 35
                                      local.get 34
                                      i64.xor
                                      i64.ctz
                                      i32.wrap_i64
                                      i32.const 3
                                      i32.shr_u
                                      local.get 18
                                      local.get 9
                                      i32.sub
                                      i32.add
                                      local.tee 3
                                      i32.const 3
                                      i32.lt_u
                                      br_if 1 (;@16;)
                                      local.get 15
                                      i32.const 8192
                                      i32.lt_u
                                      br_if 0 (;@17;)
                                      local.get 3
                                      i32.const 3
                                      i32.eq
                                      br_if 1 (;@16;)
                                    end
                                    local.get 1
                                    i32.load offset=60
                                    local.get 1
                                    i32.load16_u offset=64
                                    i32.add
                                    local.get 3
                                    local.get 16
                                    local.get 3
                                    local.get 16
                                    i32.lt_u
                                    select
                                    local.tee 3
                                    i32.const -3
                                    i32.add
                                    local.tee 17
                                    i32.store8
                                    local.get 1
                                    local.get 1
                                    i32.load offset=64
                                    i32.const 1
                                    i32.add
                                    local.tee 5
                                    i32.store offset=64
                                    local.get 1
                                    i32.load offset=60
                                    local.get 5
                                    i32.const 65535
                                    i32.and
                                    i32.add
                                    local.get 10
                                    i32.const -1
                                    i32.add
                                    local.tee 5
                                    i32.store8
                                    local.get 1
                                    local.get 1
                                    i32.load offset=64
                                    i32.const 1
                                    i32.add
                                    local.tee 18
                                    i32.store offset=64
                                    local.get 1
                                    i32.load offset=60
                                    local.get 18
                                    i32.const 65535
                                    i32.and
                                    i32.add
                                    local.get 5
                                    i32.const 65535
                                    i32.and
                                    local.tee 18
                                    i32.const 8
                                    i32.shr_u
                                    local.tee 21
                                    i32.store8
                                    local.get 1
                                    local.get 1
                                    i32.load offset=64
                                    i32.const 1
                                    i32.add
                                    i32.store offset=64
                                    local.get 1
                                    i32.load offset=60
                                    local.get 1
                                    i32.load16_u offset=68
                                    i32.add
                                    local.tee 14
                                    local.get 14
                                    i32.load8_u
                                    i32.const 1
                                    i32.shr_u
                                    i32.store8
                                    local.get 1
                                    i32.load offset=60
                                    local.get 1
                                    i32.load16_u offset=68
                                    i32.add
                                    local.tee 14
                                    local.get 14
                                    i32.load8_u
                                    i32.const 128
                                    i32.or
                                    i32.store8
                                    local.get 18
                                    i32.const 512
                                    i32.ge_u
                                    br_if 1 (;@15;)
                                    local.get 18
                                    i32.const 1050272
                                    i32.add
                                    local.set 5
                                    br 2 (;@14;)
                                  end
                                  local.get 1
                                  i32.load offset=60
                                  local.get 1
                                  i32.load16_u offset=64
                                  i32.add
                                  local.get 22
                                  i32.store8
                                  i32.const 1
                                  local.set 3
                                  local.get 1
                                  local.get 1
                                  i32.load offset=64
                                  i32.const 1
                                  i32.add
                                  i32.store offset=64
                                  local.get 1
                                  i32.load offset=60
                                  local.get 1
                                  i32.load16_u offset=68
                                  i32.add
                                  local.tee 5
                                  local.get 5
                                  i32.load8_u
                                  i32.const 1
                                  i32.shr_u
                                  i32.store8
                                  local.get 1
                                  i32.load offset=80
                                  local.get 22
                                  i32.const 255
                                  i32.and
                                  i32.const 1
                                  i32.shl
                                  i32.add
                                  local.tee 5
                                  local.get 5
                                  i32.load16_u
                                  i32.const 1
                                  i32.add
                                  i32.store16
                                  br 3 (;@12;)
                                end
                                local.get 5
                                i32.extend16_s
                                i32.const -1
                                i32.le_s
                                br_if 1 (;@13;)
                                local.get 21
                                i32.const 1050144
                                i32.add
                                local.set 5
                              end
                              local.get 1
                              i32.load offset=80
                              local.get 5
                              i32.load8_u
                              i32.const 1
                              i32.shl
                              i32.add
                              local.tee 5
                              local.get 5
                              i32.load16_u offset=576
                              i32.const 1
                              i32.add
                              i32.store16 offset=576
                              local.get 1
                              i32.load offset=80
                              local.get 17
                              i32.const 255
                              i32.and
                              i32.load8_u offset=1050784
                              i32.const 31
                              i32.and
                              i32.const 1
                              i32.shl
                              i32.add
                              local.tee 5
                              local.get 5
                              i32.load16_u offset=512
                              i32.const 1
                              i32.add
                              i32.store16 offset=512
                              br 1 (;@12;)
                            end
                            local.get 21
                            i32.const 128
                            i32.const 1051040
                            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                            unreachable
                          end
                          local.get 1
                          local.get 1
                          i32.load offset=76
                          i32.const -1
                          i32.add
                          local.tee 18
                          i32.store offset=76
                          local.get 1
                          i32.load offset=64
                          local.set 5
                          block ;; label = @12
                            local.get 18
                            br_if 0 (;@12;)
                            local.get 1
                            local.get 5
                            i32.store offset=68
                            local.get 1
                            i32.const 8
                            i32.store offset=76
                            local.get 1
                            local.get 5
                            i32.const 1
                            i32.add
                            local.tee 5
                            i32.store offset=64
                          end
                          local.get 1
                          local.get 1
                          i32.load offset=72
                          local.get 3
                          i32.add
                          i32.store offset=72
                          local.get 1
                          local.get 1
                          i32.load offset=116
                          local.get 3
                          i32.add
                          local.tee 18
                          i32.const 32768
                          local.get 18
                          i32.const 32768
                          i32.lt_u
                          select
                          i32.store offset=116
                          local.get 16
                          local.get 3
                          i32.sub
                          local.set 16
                          local.get 3
                          local.get 19
                          i32.add
                          local.set 19
                          local.get 3
                          local.get 9
                          i32.add
                          i32.const 32767
                          i32.and
                          local.set 9
                          local.get 5
                          i32.const 65529
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 19
                          i32.store offset=112
                          local.get 1
                          local.get 16
                          i32.store offset=108
                          local.get 4
                          i32.const 24
                          i32.add
                          local.get 1
                          local.get 2
                          i32.const 0
                          call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core11flush_block
                          block ;; label = @12
                            local.get 4
                            i32.load offset=24
                            i32.const 1
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const -1
                            i32.store
                            local.get 1
                            local.get 20
                            i32.store offset=36
                            br 6 (;@6;)
                          end
                          block ;; label = @12
                            local.get 4
                            i32.load offset=28
                            local.tee 3
                            br_if 0 (;@12;)
                            local.get 1
                            i32.load offset=112
                            local.set 19
                            local.get 1
                            i32.load offset=108
                            local.set 16
                            br 1 (;@11;)
                          end
                          local.get 1
                          local.get 20
                          i32.store offset=36
                          local.get 3
                          i32.const 0
                          i32.le_s
                          br_if 5 (;@6;)
                          br 8 (;@3;)
                        end
                        local.get 16
                        i32.const 3
                        i32.gt_u
                        br_if 0 (;@10;)
                      end
                    end
                    i32.const 0
                    local.set 22
                    local.get 16
                    i32.eqz
                    br_if 0 (;@8;)
                    loop ;; label = @9
                      local.get 1
                      i32.load offset=84
                      local.get 9
                      i32.add
                      i32.load8_u
                      local.set 3
                      local.get 1
                      local.get 1
                      i32.load offset=72
                      i32.const 1
                      i32.add
                      i32.store offset=72
                      local.get 1
                      i32.load offset=60
                      local.get 1
                      i32.load16_u offset=64
                      i32.add
                      local.get 3
                      i32.store8
                      local.get 1
                      local.get 1
                      i32.load offset=64
                      i32.const 1
                      i32.add
                      i32.store offset=64
                      local.get 1
                      i32.load offset=60
                      local.get 1
                      i32.load16_u offset=68
                      i32.add
                      local.tee 5
                      local.get 5
                      i32.load8_u
                      i32.const 1
                      i32.shr_u
                      i32.store8
                      local.get 1
                      local.get 1
                      i32.load offset=76
                      i32.const -1
                      i32.add
                      local.tee 5
                      i32.store offset=76
                      block ;; label = @10
                        local.get 5
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 8
                        i32.store offset=76
                        local.get 1
                        local.get 1
                        i32.load offset=64
                        local.tee 5
                        i32.store offset=68
                        local.get 1
                        local.get 5
                        i32.const 1
                        i32.add
                        i32.store offset=64
                      end
                      local.get 1
                      i32.load offset=80
                      local.get 3
                      i32.const 1
                      i32.shl
                      i32.add
                      local.tee 3
                      local.get 3
                      i32.load16_u
                      i32.const 1
                      i32.add
                      i32.store16
                      local.get 1
                      local.get 1
                      i32.load offset=116
                      i32.const 1
                      i32.add
                      local.tee 3
                      i32.const 32768
                      local.get 3
                      i32.const 32768
                      i32.lt_u
                      select
                      i32.store offset=116
                      local.get 16
                      i32.const -1
                      i32.add
                      local.set 16
                      local.get 19
                      i32.const 1
                      i32.add
                      local.set 19
                      block ;; label = @10
                        block ;; label = @11
                          local.get 1
                          i32.load offset=64
                          i32.const 65529
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 19
                          i32.store offset=112
                          local.get 1
                          local.get 16
                          i32.store offset=108
                          local.get 4
                          i32.const 16
                          i32.add
                          local.get 1
                          local.get 2
                          i32.const 0
                          call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core11flush_block
                          local.get 4
                          i32.load offset=16
                          i32.const 1
                          i32.and
                          br_if 1 (;@10;)
                          local.get 4
                          i32.load offset=20
                          local.tee 3
                          br_if 4 (;@7;)
                          local.get 1
                          i32.load offset=112
                          local.set 19
                          local.get 1
                          i32.load offset=108
                          local.set 16
                        end
                        local.get 9
                        i32.const 1
                        i32.add
                        i32.const 32767
                        i32.and
                        local.set 9
                        local.get 16
                        i32.eqz
                        br_if 2 (;@8;)
                        br 1 (;@9;)
                      end
                    end
                  end
                  local.get 1
                  local.get 20
                  i32.store offset=36
                  local.get 1
                  i32.const -1
                  i32.store
                  br 1 (;@6;)
                end
                local.get 1
                local.get 20
                i32.store offset=36
                local.get 3
                i32.const 0
                i32.gt_s
                br_if 3 (;@3;)
              end
              local.get 0
              local.get 1
              i32.load offset=40
              i32.store offset=8
              local.get 0
              local.get 1
              i32.load offset=36
              i32.store offset=4
              local.get 0
              local.get 1
              i32.load
              i32.store
              br 4 (;@1;)
            end
            local.get 1
            local.get 19
            i32.store offset=112
            local.get 1
            local.get 22
            i32.store offset=108
            local.get 1
            local.get 20
            i32.store offset=36
            br 1 (;@3;)
          end
          local.get 1
          local.get 13
          i32.store offset=112
          local.get 1
          local.get 14
          i32.store offset=108
          local.get 1
          local.get 15
          i32.store offset=36
          local.get 1
          local.get 12
          i32.store8 offset=56
          local.get 1
          local.get 10
          i32.store offset=20
          local.get 1
          local.get 11
          i32.store offset=16
        end
        block ;; label = @3
          local.get 6
          br_if 0 (;@3;)
          i32.const 0
          local.set 7
          br 1 (;@2;)
        end
        local.get 1
        i32.load8_u offset=9
        i32.const 48
        i32.and
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 1
          i32.load offset=36
          local.tee 3
          local.get 7
          i32.gt_u
          br_if 0 (;@3;)
          local.get 4
          local.get 1
          i32.load offset=32
          i32.store offset=44
          local.get 4
          i32.const 44
          i32.add
          local.get 6
          local.get 3
          call $_RNvMCskQLZIWzWry1_6adler2NtB2_7Adler3211write_slice
          local.get 1
          local.get 4
          i32.load offset=44
          i32.store offset=32
          local.get 2
          i32.load offset=16
          i32.const 0
          local.get 2
          i32.load offset=12
          select
          local.set 7
          br 1 (;@2;)
        end
        i32.const 0
        local.get 3
        local.get 7
        i32.const 1051088
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      block ;; label = @2
        local.get 1
        i32.load8_u offset=54
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        local.get 1
        i32.load offset=36
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=108
        local.get 1
        i32.load offset=28
        i32.or
        br_if 0 (;@2;)
        local.get 4
        i32.const 8
        i32.add
        local.get 1
        local.get 2
        local.get 3
        call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core11flush_block
        block ;; label = @3
          local.get 4
          i32.load offset=8
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const -1
          i32.store
          local.get 0
          i32.const -1
          i32.store
          local.get 0
          local.get 1
          i64.load offset=36 align=4
          i64.store offset=4 align=4
          br 2 (;@1;)
        end
        block ;; label = @3
          local.get 4
          i32.load offset=12
          i32.const 0
          i32.lt_s
          br_if 0 (;@3;)
          local.get 1
          local.get 1
          i32.load8_u offset=54
          local.tee 3
          i32.const 4
          i32.eq
          i32.store8 offset=53
          local.get 3
          i32.const 3
          i32.ne
          br_if 1 (;@2;)
          local.get 1
          i32.load offset=92
          i32.const 0
          i32.const 65536
          memory.fill
          local.get 1
          i32.load offset=88
          i32.const 0
          i32.const 65536
          memory.fill
          local.get 1
          i32.const 0
          i32.store offset=116
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.load offset=40
        i32.store offset=8
        local.get 0
        local.get 1
        i32.load offset=36
        i32.store offset=4
        local.get 0
        local.get 1
        i32.load
        i32.store
        br 1 (;@1;)
      end
      local.get 0
      local.get 2
      local.get 1
      call $_RNvNtNtCsb4gaQ25Ok8b_11miniz_oxide7deflate4core19flush_output_buffer
      local.get 1
      local.get 0
      i32.load
      i32.store
    end
    local.get 4
    i32.const 48
    i32.add
    global.set $__stack_pointer
  )

(func $_RNvCsbB8S1oLEkkY_4expr12parse_source (;8;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 3
    i32.const 0
    i32.store offset=72
    local.get 3
    i64.const 34359738368
    i64.store offset=64 align=4
    i32.const 0
    local.get 2
    i32.sub
    local.set 4
    i32.const 8
    local.set 5
    i32.const 0
    local.set 6
    i32.const 0
    local.set 7
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            i32.const 12
            local.set 8
            block ;; label = @5
              block ;; label = @6
                local.get 7
                local.get 2
                i32.lt_u
                br_if 0 (;@6;)
                i32.const 1
                local.set 9
                br 1 (;@5;)
              end
              local.get 7
              local.set 10
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
                                          loop ;; label = @20
                                            block ;; label = @21
                                              local.get 1
                                              local.get 10
                                              i32.add
                                              local.tee 9
                                              i32.load8_u
                                              local.tee 7
                                              i32.const -9
                                              i32.add
                                              br_table 0 (;@21;) 0 (;@21;) 11 (;@10;) 11 (;@10;) 0 (;@21;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 0 (;@21;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 6 (;@15;) 11 (;@10;) 11 (;@10;) 7 (;@14;) 8 (;@13;) 4 (;@17;) 2 (;@19;) 11 (;@10;) 3 (;@18;) 11 (;@10;) 5 (;@16;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 10 (;@11;) 11 (;@10;) 9 (;@12;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 11 (;@10;) 12 (;@9;) 11 (;@10;)
                                            end
                                            i32.const 1
                                            local.set 9
                                            local.get 2
                                            local.get 10
                                            i32.const 1
                                            i32.add
                                            local.tee 10
                                            i32.ne
                                            br_if 0 (;@20;)
                                          end
                                          local.get 2
                                          local.set 7
                                          br 14 (;@5;)
                                        end
                                        local.get 10
                                        i32.const 1
                                        i32.add
                                        local.set 7
                                        i32.const 3
                                        local.set 8
                                        br 12 (;@6;)
                                      end
                                      local.get 10
                                      i32.const 1
                                      i32.add
                                      local.set 7
                                      i32.const 4
                                      local.set 8
                                      br 11 (;@6;)
                                    end
                                    local.get 10
                                    i32.const 1
                                    i32.add
                                    local.set 7
                                    i32.const 5
                                    local.set 8
                                    br 10 (;@6;)
                                  end
                                  local.get 10
                                  i32.const 1
                                  i32.add
                                  local.set 7
                                  i32.const 6
                                  local.set 8
                                  br 9 (;@6;)
                                end
                                local.get 10
                                i32.const 1
                                i32.add
                                local.set 7
                                i32.const 7
                                local.set 8
                                br 8 (;@6;)
                              end
                              local.get 10
                              i32.const 1
                              i32.add
                              local.set 7
                              i32.const 8
                              local.set 8
                              br 7 (;@6;)
                            end
                            local.get 10
                            i32.const 1
                            i32.add
                            local.set 7
                            i32.const 9
                            local.set 8
                            br 6 (;@6;)
                          end
                          local.get 10
                          i32.const 1
                          i32.add
                          local.set 7
                          i32.const 10
                          local.set 8
                          br 5 (;@6;)
                        end
                        local.get 10
                        i32.const 1
                        i32.add
                        local.set 7
                        i32.const 11
                        local.set 8
                        br 4 (;@6;)
                      end
                      local.get 7
                      i32.const -48
                      i32.add
                      i32.const 255
                      i32.and
                      i32.const 10
                      i32.lt_u
                      br_if 1 (;@8;)
                      local.get 7
                      i32.const 223
                      i32.and
                      i32.const -65
                      i32.add
                      i32.const 255
                      i32.and
                      i32.const 26
                      i32.ge_u
                      br_if 6 (;@3;)
                    end
                    local.get 10
                    local.set 7
                    br 1 (;@7;)
                  end
                  local.get 4
                  local.get 10
                  i32.add
                  local.set 11
                  local.get 2
                  local.get 10
                  i32.sub
                  local.set 12
                  i64.const 0
                  local.set 13
                  i32.const 0
                  local.set 8
                  block ;; label = @8
                    loop ;; label = @9
                      block ;; label = @10
                        local.get 9
                        local.get 8
                        i32.add
                        i32.load8_u
                        i32.const -48
                        i32.add
                        local.tee 7
                        i32.const 255
                        i32.and
                        i32.const 10
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 10
                        local.get 8
                        i32.add
                        local.set 7
                        br 2 (;@8;)
                      end
                      local.get 13
                      i64.const 10
                      i64.mul
                      local.get 7
                      i64.extend_i32_u
                      i64.const 255
                      i64.and
                      i64.add
                      local.set 13
                      local.get 11
                      local.get 8
                      i32.const 1
                      i32.add
                      local.tee 8
                      i32.add
                      br_if 0 (;@9;)
                    end
                    local.get 2
                    local.set 7
                    local.get 12
                    local.set 8
                  end
                  local.get 8
                  i32.eqz
                  br_if 4 (;@3;)
                  local.get 13
                  i64.const 32
                  i64.shr_u
                  i32.wrap_i64
                  local.set 12
                  local.get 13
                  i32.wrap_i64
                  local.set 11
                  i32.const 0
                  local.set 9
                  i32.const 0
                  local.set 8
                  br 2 (;@5;)
                end
                block ;; label = @7
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 1
                      local.get 7
                      i32.add
                      i32.load8_u
                      local.tee 8
                      i32.const 95
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 8
                      i32.const 123
                      i32.lt_u
                      local.get 8
                      i32.const 91
                      i32.lt_u
                      local.get 8
                      i32.const -48
                      i32.add
                      i32.const 255
                      i32.and
                      i32.const 10
                      i32.lt_u
                      local.get 8
                      i32.const 64
                      i32.gt_u
                      select
                      local.get 8
                      i32.const 96
                      i32.gt_u
                      select
                      i32.eqz
                      br_if 2 (;@7;)
                    end
                    local.get 2
                    local.get 7
                    i32.const 1
                    i32.add
                    local.tee 7
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 2
                  local.set 7
                end
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 7
                          local.get 10
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 7
                          local.get 2
                          i32.gt_u
                          br_if 0 (;@11;)
                          local.get 3
                          i32.const 80
                          i32.add
                          local.get 9
                          local.get 7
                          local.get 10
                          i32.sub
                          call $_RNvNtNtCs8euEcNjxCEf_4core3str8converts9from_utf8
                          local.get 3
                          i32.load offset=80
                          br_if 8 (;@3;)
                          local.get 3
                          i32.load offset=84
                          local.set 9
                          local.get 3
                          i32.load offset=88
                          local.tee 10
                          i32.const 3
                          i32.ne
                          br_if 1 (;@10;)
                          i32.const 2
                          local.set 8
                          local.get 9
                          i32.load16_u align=1
                          i32.const 25964
                          i32.xor
                          local.get 9
                          i32.const 2
                          i32.add
                          i32.load8_u
                          i32.const 116
                          i32.xor
                          i32.or
                          i32.const 65535
                          i32.and
                          i32.eqz
                          br_if 5 (;@6;)
                          br 2 (;@9;)
                        end
                        local.get 10
                        local.get 7
                        local.get 2
                        i32.const 1049576
                        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                        unreachable
                      end
                      i32.const 0
                      local.set 12
                      local.get 10
                      i32.const 0
                      i32.lt_s
                      br_if 1 (;@8;)
                      local.get 10
                      br_if 0 (;@9;)
                      i32.const 0
                      local.set 12
                      i32.const 1
                      local.set 11
                      i32.const 0
                      local.set 14
                      i32.const 0
                      local.set 9
                      i32.const 1
                      local.set 8
                      br 4 (;@5;)
                    end
                    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 0
                        i32.load8_u offset=1066512
                        i32.eqz
                        br_if 0 (;@10;)
                        i32.const 0
                        i32.load offset=1066504
                        local.set 11
                        br 1 (;@9;)
                      end
                      block ;; label = @10
                        memory.size
                        i32.const 16
                        i32.shl
                        local.tee 11
                        br_if 0 (;@10;)
                        i32.const 1
                        local.set 12
                        i32.const 65536
                        local.set 11
                        i32.const 1
                        memory.grow
                        i32.const -1
                        i32.eq
                        br_if 2 (;@8;)
                      end
                      i32.const 0
                      local.get 11
                      i32.store offset=1066508
                      i32.const 0
                      local.get 11
                      i32.store offset=1066504
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1066512
                    end
                    i32.const 1
                    local.set 12
                    local.get 11
                    local.get 10
                    i32.add
                    local.tee 8
                    local.get 11
                    i32.lt_u
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 8
                      i32.const 0
                      i32.load offset=1066508
                      local.tee 14
                      i32.le_u
                      br_if 0 (;@9;)
                      local.get 8
                      local.get 14
                      i32.sub
                      local.tee 14
                      i32.const 16
                      i32.shr_u
                      local.get 14
                      i32.const 65535
                      i32.and
                      i32.const 0
                      i32.ne
                      i32.add
                      local.tee 14
                      memory.grow
                      local.tee 15
                      i32.const -1
                      i32.eq
                      br_if 1 (;@8;)
                      i32.const 0
                      local.get 15
                      local.get 14
                      i32.add
                      i32.const 16
                      i32.shl
                      i32.store offset=1066508
                    end
                    i32.const 0
                    local.get 8
                    i32.store offset=1066504
                    local.get 11
                    br_if 1 (;@7;)
                  end
                  local.get 12
                  local.get 10
                  call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
                  unreachable
                end
                block ;; label = @7
                  local.get 10
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 11
                  local.get 9
                  local.get 10
                  memory.copy
                end
                i32.const 1
                local.set 8
                i32.const 0
                local.set 9
                local.get 10
                local.set 12
                local.get 10
                local.set 14
                br 1 (;@5;)
              end
              i32.const 0
              local.set 9
            end
            block ;; label = @5
              local.get 6
              local.get 3
              i32.load offset=64
              i32.ne
              br_if 0 (;@5;)
              local.get 3
              i32.const 64
              i32.add
              call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecNtCsbB8S1oLEkkY_4expr5TokenE8grow_oneBO_
              local.get 3
              i32.load offset=68
              local.set 5
            end
            local.get 5
            local.get 6
            i32.const 4
            i32.shl
            i32.add
            local.tee 10
            local.get 12
            i32.store offset=12
            local.get 10
            local.get 11
            i32.store offset=8
            local.get 10
            local.get 14
            i32.store offset=4
            local.get 10
            local.get 8
            i32.store
            local.get 3
            local.get 6
            i32.const 1
            i32.add
            local.tee 6
            i32.store offset=72
            local.get 9
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 3
          i32.load offset=64
          local.tee 10
          i32.const -1
          i32.ne
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const -1
        i32.store offset=16
        br 1 (;@1;)
      end
      local.get 3
      i64.load offset=68 align=4
      local.set 13
      local.get 3
      local.get 10
      i32.store offset=4
      local.get 3
      local.get 13
      i64.store offset=8 align=4
      block ;; label = @2
        local.get 13
        i64.const 4294967295
        i64.gt_u
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            local.get 10
            i32.eqz
            br_if 0 (;@4;)
            local.get 13
            i32.wrap_i64
            local.set 10
            br 1 (;@3;)
          end
          local.get 3
          i32.const 4
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecNtCsbB8S1oLEkkY_4expr5TokenE8grow_oneBO_
          local.get 3
          i32.load offset=8
          local.set 10
        end
        local.get 10
        i32.const 12
        i32.store
        local.get 3
        i32.const 1
        i32.store offset=12
      end
      local.get 3
      local.get 3
      i32.load offset=12
      local.tee 10
      i32.store offset=24
      local.get 3
      local.get 3
      i64.load offset=4 align=4
      i64.store offset=16
      i32.const 0
      local.set 2
      local.get 3
      i32.const 0
      i32.store offset=28
      local.get 3
      i32.const 0
      i32.store offset=44
      local.get 3
      i64.const 34359738368
      i64.store offset=36 align=4
      i32.const 0
      local.set 6
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 10
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 3
                    i32.const 80
                    i32.add
                    i32.const 4
                    i32.or
                    local.set 1
                    local.get 3
                    i32.const 96
                    i32.add
                    local.set 11
                    i32.const 8
                    local.set 5
                    i32.const 0
                    local.set 7
                    i32.const 0
                    local.set 9
                    local.get 10
                    local.set 2
                    i32.const 0
                    local.set 10
                    i32.const 0
                    local.set 6
                    loop ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 3
                            i32.load offset=20
                            local.tee 8
                            local.get 6
                            i32.const 4
                            i32.shl
                            i32.add
                            i32.load
                            i32.const 2
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 3
                            i32.const 80
                            i32.add
                            local.get 3
                            i32.const 16
                            i32.add
                            i32.const 0
                            call $_RNvMs_CsbB8S1oLEkkY_4exprNtB4_6Parser10parse_expr
                            local.get 3
                            i32.load8_u offset=80
                            i32.const 255
                            i32.ne
                            br_if 1 (;@11;)
                            local.get 0
                            i32.const -1
                            i32.store offset=16
                            br 10 (;@2;)
                          end
                          local.get 9
                          local.get 2
                          i32.const -1
                          i32.add
                          local.tee 6
                          local.get 9
                          local.get 6
                          i32.lt_u
                          select
                          local.tee 6
                          local.get 2
                          i32.ge_u
                          br_if 5 (;@6;)
                          block ;; label = @12
                            local.get 8
                            local.get 6
                            i32.const 4
                            i32.shl
                            i32.add
                            local.tee 6
                            i32.load
                            i32.const 1
                            i32.ne
                            br_if 0 (;@12;)
                            local.get 1
                            local.get 6
                            i32.const 4
                            i32.add
                            call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                          end
                          block ;; label = @12
                            block ;; label = @13
                              local.get 9
                              i32.const 1
                              i32.add
                              local.tee 9
                              local.get 2
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 3
                              i32.load offset=28
                              local.set 9
                              br 1 (;@12;)
                            end
                            local.get 3
                            local.get 9
                            i32.store offset=28
                          end
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 9
                                  local.get 3
                                  i32.load offset=24
                                  local.tee 2
                                  i32.const -1
                                  i32.add
                                  local.tee 6
                                  local.get 9
                                  local.get 6
                                  i32.lt_u
                                  select
                                  local.tee 6
                                  local.get 2
                                  i32.ge_u
                                  br_if 0 (;@15;)
                                  local.get 3
                                  i32.load offset=20
                                  local.get 6
                                  i32.const 4
                                  i32.shl
                                  i32.add
                                  local.tee 8
                                  i32.load
                                  local.tee 6
                                  i32.const -2
                                  i32.add
                                  i32.const 11
                                  i32.lt_u
                                  br_if 3 (;@12;)
                                  local.get 6
                                  br_table 1 (;@14;) 2 (;@13;) 1 (;@14;)
                                end
                                local.get 6
                                local.get 2
                                i32.const 1049592
                                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                unreachable
                              end
                              local.get 3
                              local.get 8
                              i64.load offset=8
                              i64.store offset=88
                              br 1 (;@12;)
                            end
                            local.get 1
                            local.get 8
                            i32.const 4
                            i32.add
                            call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                          end
                          block ;; label = @12
                            local.get 9
                            i32.const 1
                            i32.add
                            local.tee 9
                            local.get 2
                            i32.ge_u
                            br_if 0 (;@12;)
                            local.get 3
                            local.get 9
                            i32.store offset=28
                          end
                          local.get 6
                          i32.const 1
                          i32.eq
                          br_if 1 (;@10;)
                          local.get 0
                          i32.const -1
                          i32.store offset=16
                          br 9 (;@2;)
                        end
                        local.get 3
                        local.get 3
                        i64.load offset=88
                        i64.store offset=72
                        local.get 3
                        local.get 3
                        i64.load offset=80
                        i64.store offset=64
                        local.get 3
                        i32.load offset=28
                        local.tee 9
                        local.get 3
                        i32.load offset=24
                        local.tee 2
                        i32.const -1
                        i32.add
                        local.tee 1
                        local.get 9
                        local.get 1
                        i32.lt_u
                        select
                        local.tee 9
                        local.get 2
                        i32.ge_u
                        br_if 5 (;@5;)
                        local.get 3
                        i32.load offset=20
                        local.get 9
                        i32.const 4
                        i32.shl
                        i32.add
                        i32.load
                        i32.const 12
                        i32.eq
                        br_if 3 (;@7;)
                        local.get 0
                        i32.const -1
                        i32.store offset=16
                        local.get 3
                        i32.const 64
                        i32.add
                        call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueNtCsbB8S1oLEkkY_4expr4ExprEBD_
                        br 8 (;@2;)
                      end
                      local.get 3
                      local.get 1
                      i32.load offset=8
                      i32.store offset=56
                      local.get 3
                      local.get 1
                      i64.load align=4
                      i64.store offset=48
                      local.get 3
                      i32.load offset=28
                      local.tee 9
                      local.get 3
                      i32.load offset=24
                      local.tee 2
                      i32.const -1
                      i32.add
                      local.tee 6
                      local.get 9
                      local.get 6
                      i32.lt_u
                      select
                      local.tee 6
                      local.get 2
                      i32.ge_u
                      br_if 5 (;@4;)
                      block ;; label = @10
                        local.get 3
                        i32.load offset=20
                        local.get 6
                        i32.const 4
                        i32.shl
                        i32.add
                        local.tee 8
                        i32.load
                        local.tee 6
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 8
                        i32.const 4
                        i32.add
                        call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                      end
                      block ;; label = @10
                        local.get 9
                        i32.const 1
                        i32.add
                        local.tee 9
                        local.get 2
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 3
                        local.get 9
                        i32.store offset=28
                      end
                      block ;; label = @10
                        local.get 6
                        i32.const 10
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const -1
                        i32.store offset=16
                        br 8 (;@2;)
                      end
                      local.get 3
                      i32.const 80
                      i32.add
                      local.get 3
                      i32.const 16
                      i32.add
                      i32.const 0
                      call $_RNvMs_CsbB8S1oLEkkY_4exprNtB4_6Parser10parse_expr
                      block ;; label = @10
                        local.get 3
                        i32.load8_u offset=80
                        i32.const 255
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const -1
                        i32.store offset=16
                        br 8 (;@2;)
                      end
                      local.get 3
                      local.get 3
                      i64.load offset=88
                      i64.store offset=72
                      local.get 3
                      local.get 3
                      i64.load offset=80
                      i64.store offset=64
                      local.get 3
                      i32.load offset=28
                      local.tee 9
                      local.get 3
                      i32.load offset=24
                      local.tee 2
                      i32.const -1
                      i32.add
                      local.tee 6
                      local.get 9
                      local.get 6
                      i32.lt_u
                      select
                      local.tee 6
                      local.get 2
                      i32.ge_u
                      br_if 6 (;@3;)
                      block ;; label = @10
                        local.get 3
                        i32.load offset=20
                        local.get 6
                        i32.const 4
                        i32.shl
                        i32.add
                        local.tee 8
                        i32.load
                        local.tee 6
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 8
                        i32.const 4
                        i32.add
                        call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                      end
                      block ;; label = @10
                        local.get 9
                        i32.const 1
                        i32.add
                        local.tee 9
                        local.get 2
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 3
                        local.get 9
                        i32.store offset=28
                      end
                      block ;; label = @10
                        local.get 6
                        i32.const 11
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 0
                        i32.const -1
                        i32.store offset=16
                        local.get 3
                        i32.const 64
                        i32.add
                        call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueNtCsbB8S1oLEkkY_4expr4ExprEBD_
                        br 8 (;@2;)
                      end
                      local.get 11
                      local.get 3
                      i64.load offset=64
                      i64.store
                      local.get 11
                      local.get 3
                      i64.load offset=72
                      i64.store offset=8
                      local.get 3
                      local.get 3
                      i32.load offset=56
                      i32.store offset=88
                      local.get 3
                      local.get 3
                      i64.load offset=48
                      i64.store offset=80
                      block ;; label = @10
                        local.get 10
                        local.get 3
                        i32.load offset=36
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 36
                        i32.add
                        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecTNtNtB7_6string6StringNtCsbB8S1oLEkkY_4expr4ExprEE8grow_oneB1a_
                        local.get 3
                        i32.load offset=40
                        local.set 5
                      end
                      local.get 5
                      local.get 7
                      i32.add
                      local.tee 2
                      local.get 3
                      i64.load offset=80
                      i64.store
                      local.get 2
                      local.get 3
                      i64.load offset=88
                      i64.store offset=8
                      local.get 2
                      local.get 3
                      i64.load offset=104
                      i64.store offset=24
                      local.get 2
                      local.get 3
                      i64.load offset=96
                      i64.store offset=16
                      local.get 3
                      local.get 10
                      i32.const 1
                      i32.add
                      local.tee 10
                      i32.store offset=44
                      local.get 7
                      i32.const 32
                      i32.add
                      local.set 7
                      local.get 3
                      i32.load offset=28
                      local.tee 9
                      local.get 3
                      i32.load offset=24
                      local.tee 2
                      i32.const -1
                      i32.add
                      local.tee 6
                      local.get 9
                      local.get 6
                      i32.lt_u
                      select
                      local.tee 6
                      local.get 2
                      i32.lt_u
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 6
                  local.get 2
                  i32.const 1049592
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 0
                local.get 3
                i64.load offset=64
                i64.store
                local.get 0
                local.get 3
                i64.load offset=72
                i64.store offset=8
                local.get 0
                local.get 3
                i64.load offset=36 align=4
                i64.store offset=16
                local.get 3
                local.get 3
                i32.load offset=44
                i32.store offset=104
                local.get 0
                local.get 3
                i64.load offset=104
                i64.store offset=24
                br 5 (;@1;)
              end
              local.get 6
              local.get 2
              i32.const 1049592
              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
              unreachable
            end
            local.get 9
            local.get 2
            i32.const 1049592
            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
            unreachable
          end
          local.get 6
          local.get 2
          i32.const 1049592
          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
          unreachable
        end
        local.get 6
        local.get 2
        i32.const 1049592
        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
        unreachable
      end
      local.get 10
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.const 16
      i32.add
      local.set 2
      loop ;; label = @2
        local.get 2
        call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueNtCsbB8S1oLEkkY_4expr4ExprEBD_
        local.get 2
        i32.const 32
        i32.add
        local.set 2
        local.get 10
        i32.const -1
        i32.add
        local.tee 10
        br_if 0 (;@2;)
      end
    end
    local.get 3
    i32.const 112
    i32.add
    global.set $__stack_pointer
  )

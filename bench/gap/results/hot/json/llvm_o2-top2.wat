(func $_RNvMs0_CskpYeTZolUQS_4jsonNtB5_6Parser11parse_value (;23;) (type 6) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 96
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    block ;; label = @1
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
                            local.get 1
                            i32.load offset=12
                            local.tee 3
                            local.get 1
                            i32.load offset=8
                            local.tee 4
                            i32.const -1
                            i32.add
                            local.tee 5
                            local.get 3
                            local.get 5
                            i32.lt_u
                            select
                            local.tee 5
                            local.get 4
                            i32.ge_u
                            br_if 0 (;@12;)
                            block ;; label = @13
                              i32.const 1
                              local.get 1
                              i32.load offset=4
                              local.get 5
                              i32.const 4
                              i32.shl
                              i32.add
                              local.tee 6
                              i32.load
                              local.tee 5
                              i32.shl
                              i32.const 3903
                              i32.and
                              br_if 0 (;@13;)
                              block ;; label = @14
                                local.get 5
                                i32.const 6
                                i32.ne
                                br_if 0 (;@14;)
                                local.get 2
                                i32.const 4
                                i32.or
                                local.get 6
                                i32.const 4
                                i32.add
                                call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                                br 1 (;@13;)
                              end
                              local.get 2
                              local.get 6
                              i64.load offset=8
                              i64.store offset=8
                            end
                            local.get 2
                            local.get 5
                            i32.store
                            block ;; label = @13
                              local.get 3
                              i32.const 1
                              i32.add
                              local.tee 3
                              local.get 4
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 1
                              local.get 3
                              i32.store offset=12
                            end
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  block ;; label = @16
                                    block ;; label = @17
                                      block ;; label = @18
                                        block ;; label = @19
                                          block ;; label = @20
                                            local.get 5
                                            br_table 1 (;@19;) 0 (;@20;) 2 (;@18;) 0 (;@20;) 0 (;@20;) 0 (;@20;) 3 (;@17;) 4 (;@16;) 5 (;@15;) 6 (;@14;) 7 (;@13;) 0 (;@20;)
                                          end
                                          local.get 0
                                          i32.const -1
                                          i32.store
                                          br 17 (;@2;)
                                        end
                                        local.get 2
                                        i32.const 0
                                        i32.store offset=28
                                        local.get 2
                                        i64.const 34359738368
                                        i64.store offset=20 align=4
                                        local.get 1
                                        i32.load offset=12
                                        local.tee 4
                                        local.get 1
                                        i32.load offset=8
                                        local.tee 5
                                        i32.const -1
                                        i32.add
                                        local.tee 3
                                        local.get 4
                                        local.get 3
                                        i32.lt_u
                                        select
                                        local.tee 6
                                        local.get 5
                                        i32.ge_u
                                        br_if 17 (;@1;)
                                        local.get 1
                                        i32.load offset=4
                                        local.get 6
                                        i32.const 4
                                        i32.shl
                                        i32.add
                                        i32.load
                                        i32.const 1
                                        i32.eq
                                        br_if 15 (;@3;)
                                        local.get 2
                                        i32.const 32
                                        i32.add
                                        i32.const 4
                                        i32.or
                                        local.set 3
                                        local.get 2
                                        i32.const 48
                                        i32.add
                                        local.set 7
                                        block ;; label = @19
                                          loop ;; label = @20
                                            block ;; label = @21
                                              i32.const 1
                                              local.get 1
                                              i32.load offset=4
                                              local.get 6
                                              i32.const 4
                                              i32.shl
                                              i32.add
                                              local.tee 8
                                              i32.load
                                              local.tee 6
                                              i32.shl
                                              i32.const 3903
                                              i32.and
                                              br_if 0 (;@21;)
                                              block ;; label = @22
                                                local.get 6
                                                i32.const 6
                                                i32.ne
                                                br_if 0 (;@22;)
                                                local.get 3
                                                local.get 8
                                                i32.const 4
                                                i32.add
                                                call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                                                br 1 (;@21;)
                                              end
                                              local.get 2
                                              local.get 8
                                              i64.load offset=8
                                              i64.store offset=40
                                            end
                                            block ;; label = @21
                                              local.get 4
                                              i32.const 1
                                              i32.add
                                              local.tee 4
                                              local.get 5
                                              i32.ge_u
                                              br_if 0 (;@21;)
                                              local.get 1
                                              local.get 4
                                              i32.store offset=12
                                            end
                                            local.get 6
                                            i32.const 6
                                            i32.ne
                                            br_if 10 (;@10;)
                                            local.get 2
                                            local.get 3
                                            i32.load offset=8
                                            i32.store offset=72
                                            local.get 2
                                            local.get 3
                                            i64.load align=4
                                            i64.store offset=64
                                            local.get 1
                                            i32.load offset=12
                                            local.tee 4
                                            local.get 1
                                            i32.load offset=8
                                            local.tee 5
                                            i32.const -1
                                            i32.add
                                            local.tee 6
                                            local.get 4
                                            local.get 6
                                            i32.lt_u
                                            select
                                            local.tee 6
                                            local.get 5
                                            i32.ge_u
                                            br_if 9 (;@11;)
                                            block ;; label = @21
                                              local.get 1
                                              i32.load offset=4
                                              local.get 6
                                              i32.const 4
                                              i32.shl
                                              i32.add
                                              local.tee 8
                                              i32.load
                                              local.tee 6
                                              i32.const 6
                                              i32.ne
                                              br_if 0 (;@21;)
                                              local.get 3
                                              local.get 8
                                              i32.const 4
                                              i32.add
                                              call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                                            end
                                            block ;; label = @21
                                              local.get 4
                                              i32.const 1
                                              i32.add
                                              local.tee 4
                                              local.get 5
                                              i32.ge_u
                                              br_if 0 (;@21;)
                                              local.get 1
                                              local.get 4
                                              i32.store offset=12
                                            end
                                            local.get 6
                                            i32.const 6
                                            i32.eq
                                            br_if 1 (;@19;)
                                            local.get 6
                                            i32.const 4
                                            i32.ne
                                            br_if 1 (;@19;)
                                            local.get 2
                                            i32.const 80
                                            i32.add
                                            local.get 1
                                            call $_RNvMs0_CskpYeTZolUQS_4jsonNtB5_6Parser11parse_value
                                            local.get 2
                                            i32.load offset=80
                                            i32.const -1
                                            i32.eq
                                            br_if 1 (;@19;)
                                            local.get 7
                                            local.get 2
                                            i64.load offset=88
                                            i64.store offset=8
                                            local.get 7
                                            local.get 2
                                            i64.load offset=80
                                            i64.store
                                            local.get 2
                                            local.get 2
                                            i64.load offset=64
                                            i64.store offset=32
                                            local.get 2
                                            local.get 2
                                            i32.load offset=72
                                            i32.store offset=40
                                            block ;; label = @21
                                              local.get 2
                                              i32.load offset=28
                                              local.tee 4
                                              local.get 2
                                              i32.load offset=20
                                              i32.ne
                                              br_if 0 (;@21;)
                                              local.get 2
                                              i32.const 20
                                              i32.add
                                              call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecTNtNtB7_6string6StringNtCskpYeTZolUQS_4json5ValueEE8grow_oneB1a_
                                            end
                                            local.get 2
                                            i32.load offset=24
                                            local.get 4
                                            i32.const 5
                                            i32.shl
                                            i32.add
                                            local.tee 5
                                            local.get 2
                                            i64.load offset=32
                                            i64.store
                                            local.get 5
                                            local.get 2
                                            i64.load offset=40
                                            i64.store offset=8
                                            local.get 5
                                            local.get 2
                                            i64.load offset=48
                                            i64.store offset=16
                                            local.get 5
                                            local.get 2
                                            i64.load offset=56
                                            i64.store offset=24
                                            local.get 2
                                            local.get 4
                                            i32.const 1
                                            i32.add
                                            i32.store offset=28
                                            local.get 1
                                            i32.load offset=12
                                            local.tee 4
                                            local.get 1
                                            i32.load offset=8
                                            local.tee 5
                                            i32.const -1
                                            i32.add
                                            local.tee 6
                                            local.get 4
                                            local.get 6
                                            i32.lt_u
                                            select
                                            local.tee 6
                                            local.get 5
                                            i32.ge_u
                                            br_if 11 (;@9;)
                                            block ;; label = @21
                                              local.get 1
                                              i32.load offset=4
                                              local.get 6
                                              i32.const 4
                                              i32.shl
                                              i32.add
                                              local.tee 8
                                              i32.load
                                              local.tee 6
                                              i32.const 6
                                              i32.ne
                                              br_if 0 (;@21;)
                                              local.get 3
                                              local.get 8
                                              i32.const 4
                                              i32.add
                                              call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                                            end
                                            block ;; label = @21
                                              local.get 4
                                              i32.const 1
                                              i32.add
                                              local.tee 4
                                              local.get 5
                                              i32.ge_u
                                              br_if 0 (;@21;)
                                              local.get 1
                                              local.get 4
                                              i32.store offset=12
                                            end
                                            block ;; label = @21
                                              local.get 6
                                              i32.const 5
                                              i32.eq
                                              br_if 0 (;@21;)
                                              block ;; label = @22
                                                local.get 6
                                                i32.const 1
                                                i32.eq
                                                br_if 0 (;@22;)
                                                local.get 0
                                                i32.const -1
                                                i32.store
                                                br 18 (;@4;)
                                              end
                                              local.get 0
                                              local.get 2
                                              i32.load offset=28
                                              i32.store offset=8
                                              local.get 0
                                              local.get 2
                                              i64.load offset=20 align=4
                                              i64.store
                                              br 19 (;@2;)
                                            end
                                            local.get 1
                                            i32.load offset=12
                                            local.tee 4
                                            local.get 1
                                            i32.load offset=8
                                            local.tee 5
                                            i32.const -1
                                            i32.add
                                            local.tee 6
                                            local.get 4
                                            local.get 6
                                            i32.lt_u
                                            select
                                            local.tee 6
                                            local.get 5
                                            i32.lt_u
                                            br_if 0 (;@20;)
                                          end
                                          local.get 6
                                          local.get 5
                                          i32.const 1050256
                                          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                                          unreachable
                                        end
                                        local.get 0
                                        i32.const -1
                                        i32.store
                                        br 14 (;@4;)
                                      end
                                      local.get 2
                                      i32.const 0
                                      i32.store offset=72
                                      local.get 2
                                      i64.const 34359738368
                                      i64.store offset=64 align=4
                                      local.get 1
                                      i32.load offset=12
                                      local.tee 4
                                      local.get 1
                                      i32.load offset=8
                                      local.tee 5
                                      i32.const -1
                                      i32.add
                                      local.tee 3
                                      local.get 4
                                      local.get 3
                                      i32.lt_u
                                      select
                                      local.tee 3
                                      local.get 5
                                      i32.ge_u
                                      br_if 12 (;@5;)
                                      local.get 1
                                      i32.load offset=4
                                      local.get 3
                                      i32.const 4
                                      i32.shl
                                      i32.add
                                      i32.load
                                      i32.const 3
                                      i32.eq
                                      br_if 10 (;@7;)
                                      local.get 2
                                      i32.const 32
                                      i32.add
                                      local.get 1
                                      call $_RNvMs0_CskpYeTZolUQS_4jsonNtB5_6Parser11parse_value
                                      block ;; label = @18
                                        block ;; label = @19
                                          local.get 2
                                          i32.load offset=32
                                          i32.const -1
                                          i32.eq
                                          br_if 0 (;@19;)
                                          local.get 2
                                          i32.const 32
                                          i32.add
                                          i32.const 4
                                          i32.or
                                          local.set 7
                                          loop ;; label = @20
                                            local.get 2
                                            local.get 2
                                            i64.load offset=40
                                            i64.store offset=88
                                            local.get 2
                                            local.get 2
                                            i64.load offset=32
                                            i64.store offset=80
                                            block ;; label = @21
                                              local.get 2
                                              i32.load offset=72
                                              local.tee 5
                                              local.get 2
                                              i32.load offset=64
                                              i32.ne
                                              br_if 0 (;@21;)
                                              local.get 2
                                              i32.const 64
                                              i32.add
                                              call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecNtCskpYeTZolUQS_4json5TokenE8grow_oneBO_
                                            end
                                            local.get 2
                                            i32.load offset=68
                                            local.get 5
                                            i32.const 4
                                            i32.shl
                                            i32.add
                                            local.tee 4
                                            local.get 2
                                            i64.load offset=80
                                            i64.store
                                            local.get 4
                                            local.get 2
                                            i64.load offset=88
                                            i64.store offset=8
                                            local.get 2
                                            local.get 5
                                            i32.const 1
                                            i32.add
                                            i32.store offset=72
                                            local.get 1
                                            i32.load offset=12
                                            local.tee 4
                                            local.get 1
                                            i32.load offset=8
                                            local.tee 5
                                            i32.const -1
                                            i32.add
                                            local.tee 3
                                            local.get 4
                                            local.get 3
                                            i32.lt_u
                                            select
                                            local.tee 3
                                            local.get 5
                                            i32.ge_u
                                            br_if 12 (;@8;)
                                            block ;; label = @21
                                              local.get 1
                                              i32.load offset=4
                                              local.get 3
                                              i32.const 4
                                              i32.shl
                                              i32.add
                                              local.tee 6
                                              i32.load
                                              local.tee 3
                                              i32.const 6
                                              i32.ne
                                              br_if 0 (;@21;)
                                              local.get 7
                                              local.get 6
                                              i32.const 4
                                              i32.add
                                              call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                                            end
                                            block ;; label = @21
                                              local.get 4
                                              i32.const 1
                                              i32.add
                                              local.tee 4
                                              local.get 5
                                              i32.ge_u
                                              br_if 0 (;@21;)
                                              local.get 1
                                              local.get 4
                                              i32.store offset=12
                                            end
                                            block ;; label = @21
                                              local.get 3
                                              i32.const 5
                                              i32.eq
                                              br_if 0 (;@21;)
                                              local.get 3
                                              i32.const 3
                                              i32.eq
                                              br_if 15 (;@6;)
                                              local.get 0
                                              i32.const -1
                                              i32.store
                                              br 3 (;@18;)
                                            end
                                            local.get 2
                                            i32.const 32
                                            i32.add
                                            local.get 1
                                            call $_RNvMs0_CskpYeTZolUQS_4jsonNtB5_6Parser11parse_value
                                            local.get 2
                                            i32.load offset=32
                                            i32.const -1
                                            i32.ne
                                            br_if 0 (;@20;)
                                          end
                                        end
                                        local.get 0
                                        i32.const -1
                                        i32.store
                                      end
                                      local.get 2
                                      i32.load offset=72
                                      local.tee 0
                                      i32.eqz
                                      br_if 15 (;@2;)
                                      local.get 2
                                      i32.load offset=68
                                      local.set 6
                                      i32.const 0
                                      local.set 4
                                      loop ;; label = @18
                                        block ;; label = @19
                                          local.get 6
                                          local.get 4
                                          i32.const 4
                                          i32.shl
                                          i32.add
                                          local.tee 3
                                          i32.load
                                          local.tee 1
                                          i32.const -2147483648
                                          i32.xor
                                          i32.const 5
                                          local.get 1
                                          i32.const 0
                                          i32.lt_s
                                          select
                                          local.tee 1
                                          i32.const 4
                                          i32.lt_u
                                          br_if 0 (;@19;)
                                          block ;; label = @20
                                            local.get 1
                                            i32.const 4
                                            i32.eq
                                            br_if 0 (;@20;)
                                            local.get 3
                                            i32.load offset=8
                                            local.tee 5
                                            i32.eqz
                                            br_if 1 (;@19;)
                                            local.get 3
                                            i32.load offset=4
                                            local.set 1
                                            loop ;; label = @21
                                              local.get 1
                                              call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueTNtNtCseFzb18zECBI_5alloc6string6StringNtCskpYeTZolUQS_4json5ValueEEB1g_.llvm.12285963892569529774
                                              local.get 1
                                              i32.const 32
                                              i32.add
                                              local.set 1
                                              local.get 5
                                              i32.const -1
                                              i32.add
                                              local.tee 5
                                              br_if 0 (;@21;)
                                              br 2 (;@19;)
                                            end
                                          end
                                          local.get 3
                                          i32.const 4
                                          i32.add
                                          call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueINtNtCseFzb18zECBI_5alloc3vec3VecNtCskpYeTZolUQS_4json5ValueEEB1a_.llvm.12285963892569529774
                                        end
                                        local.get 0
                                        local.get 4
                                        i32.const 1
                                        i32.add
                                        local.tee 4
                                        i32.ne
                                        br_if 0 (;@18;)
                                        br 16 (;@2;)
                                      end
                                    end
                                    local.get 0
                                    local.get 2
                                    i32.load offset=12
                                    i32.store offset=12
                                    local.get 0
                                    local.get 2
                                    i64.load offset=4 align=4
                                    i64.store offset=4 align=4
                                    local.get 0
                                    i32.const -2147483645
                                    i32.store
                                    br 14 (;@2;)
                                  end
                                  local.get 0
                                  local.get 2
                                  i64.load offset=8
                                  i64.store offset=8
                                  local.get 0
                                  i32.const -2147483646
                                  i32.store
                                  br 13 (;@2;)
                                end
                                local.get 0
                                i32.const 1
                                i32.store8 offset=4
                                local.get 0
                                i32.const -2147483647
                                i32.store
                                br 12 (;@2;)
                              end
                              local.get 0
                              i32.const 0
                              i32.store8 offset=4
                              local.get 0
                              i32.const -2147483647
                              i32.store
                              br 11 (;@2;)
                            end
                            local.get 0
                            i32.const -2147483648
                            i32.store
                            br 10 (;@2;)
                          end
                          local.get 5
                          local.get 4
                          i32.const 1050256
                          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                          unreachable
                        end
                        local.get 6
                        local.get 5
                        i32.const 1050256
                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                        unreachable
                      end
                      local.get 0
                      i32.const -1
                      i32.store
                      br 5 (;@4;)
                    end
                    local.get 6
                    local.get 5
                    i32.const 1050256
                    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                    unreachable
                  end
                  local.get 3
                  local.get 5
                  i32.const 1050256
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 4
                i32.const 1
                i32.add
                local.tee 4
                local.get 5
                i32.ge_u
                br_if 0 (;@6;)
                local.get 1
                local.get 4
                i32.store offset=12
              end
              local.get 0
              local.get 2
              i32.load offset=72
              i32.store offset=12
              local.get 0
              local.get 2
              i64.load offset=64 align=4
              i64.store offset=4 align=4
              local.get 0
              i32.const -2147483644
              i32.store
              br 3 (;@2;)
            end
            local.get 3
            local.get 5
            i32.const 1050256
            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
            unreachable
          end
          local.get 2
          i32.const 20
          i32.add
          call $_RNvXsp_NtCseFzb18zECBI_5alloc3vecINtB5_3VecTNtNtB7_6string6StringNtCskpYeTZolUQS_4json5ValueEENtNtNtCs8euEcNjxCEf_4core3ops4drop4Drop4dropB13_
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          local.get 5
          i32.ge_u
          br_if 0 (;@3;)
          local.get 1
          local.get 4
          i32.store offset=12
        end
        local.get 0
        local.get 2
        i32.load offset=28
        i32.store offset=8
        local.get 0
        local.get 2
        i64.load offset=20 align=4
        i64.store
      end
      local.get 2
      i32.const 96
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 6
    local.get 5
    i32.const 1050256
    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
    unreachable
  )

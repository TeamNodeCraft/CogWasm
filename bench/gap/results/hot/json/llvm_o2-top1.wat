(func $run (;27;) (type 12) (param i64) (result i64)
    (local i32 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 208
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i64.const -7046029254386353131
    i64.add
    local.tee 2
    i64.const 30
    i64.shr_u
    local.get 2
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 2
    i64.const 27
    i64.shr_u
    local.get 2
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 2
    i64.const 31
    i64.shr_u
    local.get 2
    i64.xor
    local.set 2
    local.get 0
    i64.const 29
    i64.rem_u
    i64.const -7
    i64.add
    local.set 3
    local.get 1
    i32.const 112
    i32.add
    local.set 4
    local.get 1
    i32.const 16
    i32.add
    i32.const 4
    i32.or
    local.set 5
    local.get 1
    i32.const 184
    i32.add
    i32.const 4
    i32.or
    local.set 6
    local.get 0
    i32.wrap_i64
    local.set 7
    i32.const 0
    local.set 8
    loop ;; label = @1
      local.get 1
      i32.const 0
      i32.store offset=148
      local.get 1
      local.get 8
      local.get 7
      i32.add
      i32.const 3
      i32.and
      i32.const 3
      i32.shl
      local.tee 9
      i32.load offset=1050132
      local.tee 10
      i32.store offset=144
      local.get 1
      local.get 9
      i32.load offset=1050128
      local.tee 11
      i32.store offset=140
      local.get 1
      i32.const 0
      i32.store offset=176
      local.get 1
      i64.const 34359738368
      i64.store offset=168 align=4
      i32.const 0
      local.set 9
      i32.const 0
      local.set 12
      i32.const 0
      local.set 13
      i32.const 0
      local.set 14
      local.get 10
      local.set 15
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  loop ;; label = @8
                    i32.const 11
                    local.set 16
                    i32.const 1
                    local.set 17
                    block ;; label = @9
                      local.get 14
                      local.get 15
                      i32.ge_u
                      br_if 0 (;@9;)
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
                                                  loop ;; label = @24
                                                    block ;; label = @25
                                                      local.get 11
                                                      local.get 14
                                                      i32.add
                                                      i32.load8_u
                                                      local.tee 18
                                                      i32.const -9
                                                      i32.add
                                                      br_table 0 (;@25;) 0 (;@25;) 11 (;@14;) 11 (;@14;) 0 (;@25;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 0 (;@25;) 11 (;@14;) 8 (;@17;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 7 (;@18;) 9 (;@16;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 6 (;@19;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 4 (;@21;) 11 (;@14;) 5 (;@20;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 11 (;@14;) 2 (;@23;) 11 (;@14;) 3 (;@22;) 11 (;@14;)
                                                    end
                                                    local.get 14
                                                    i32.const 1
                                                    i32.add
                                                    local.tee 9
                                                    local.set 13
                                                    local.get 9
                                                    local.set 14
                                                    local.get 15
                                                    local.get 9
                                                    i32.eq
                                                    br_if 9 (;@15;)
                                                    br 0 (;@24;)
                                                  end
                                                end
                                                local.get 1
                                                local.get 14
                                                i32.const 1
                                                i32.add
                                                local.tee 9
                                                i32.store offset=148
                                                i32.const 0
                                                local.set 17
                                                local.get 9
                                                local.set 13
                                                local.get 9
                                                local.set 14
                                                i32.const 0
                                                local.set 16
                                                br 13 (;@9;)
                                              end
                                              i32.const 1
                                              local.set 16
                                              local.get 1
                                              local.get 14
                                              i32.const 1
                                              i32.add
                                              local.tee 9
                                              i32.store offset=148
                                              br 10 (;@11;)
                                            end
                                            local.get 1
                                            local.get 14
                                            i32.const 1
                                            i32.add
                                            local.tee 9
                                            i32.store offset=148
                                            i32.const 2
                                            local.set 16
                                            br 9 (;@11;)
                                          end
                                          local.get 1
                                          local.get 14
                                          i32.const 1
                                          i32.add
                                          local.tee 9
                                          i32.store offset=148
                                          i32.const 3
                                          local.set 16
                                          br 8 (;@11;)
                                        end
                                        local.get 1
                                        local.get 14
                                        i32.const 1
                                        i32.add
                                        local.tee 9
                                        i32.store offset=148
                                        i32.const 4
                                        local.set 16
                                        br 7 (;@11;)
                                      end
                                      local.get 1
                                      local.get 14
                                      i32.const 1
                                      i32.add
                                      local.tee 9
                                      i32.store offset=148
                                      i32.const 5
                                      local.set 16
                                      br 6 (;@11;)
                                    end
                                    local.get 1
                                    i32.const 0
                                    i32.store offset=192
                                    local.get 1
                                    i64.const 4294967296
                                    i64.store offset=184 align=4
                                    local.get 14
                                    i32.const 1
                                    i32.add
                                    local.tee 14
                                    local.get 15
                                    i32.ge_u
                                    br_if 9 (;@7;)
                                    i32.const 0
                                    local.set 19
                                    loop ;; label = @17
                                      local.get 1
                                      local.get 14
                                      i32.const 1
                                      i32.add
                                      local.tee 9
                                      i32.store offset=148
                                      block ;; label = @18
                                        block ;; label = @19
                                          block ;; label = @20
                                            block ;; label = @21
                                              local.get 1
                                              i32.load offset=140
                                              local.tee 11
                                              local.get 14
                                              i32.add
                                              i32.load8_u
                                              local.tee 13
                                              i32.const 92
                                              i32.eq
                                              br_if 0 (;@21;)
                                              local.get 13
                                              i32.const 34
                                              i32.eq
                                              br_if 1 (;@20;)
                                              local.get 13
                                              i32.const 32
                                              i32.lt_u
                                              br_if 14 (;@7;)
                                              block ;; label = @22
                                                local.get 19
                                                local.get 1
                                                i32.load offset=184
                                                i32.ne
                                                br_if 0 (;@22;)
                                                local.get 1
                                                i32.const 184
                                                i32.add
                                                call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                              end
                                              local.get 1
                                              i32.load offset=188
                                              local.get 19
                                              i32.add
                                              local.get 13
                                              i32.store8
                                              local.get 1
                                              local.get 19
                                              i32.const 1
                                              i32.add
                                              local.tee 19
                                              i32.store offset=192
                                              local.get 9
                                              local.set 14
                                              br 3 (;@18;)
                                            end
                                            local.get 9
                                            local.get 15
                                            i32.ge_u
                                            br_if 13 (;@7;)
                                            local.get 1
                                            local.get 14
                                            i32.const 2
                                            i32.add
                                            local.tee 14
                                            i32.store offset=148
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
                                                                local.get 11
                                                                local.get 9
                                                                i32.add
                                                                i32.load8_u
                                                                i32.const -34
                                                                i32.add
                                                                br_table 1 (;@29;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 3 (;@27;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 2 (;@28;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 4 (;@26;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 5 (;@25;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 6 (;@24;) 23 (;@7;) 23 (;@7;) 23 (;@7;) 7 (;@23;) 23 (;@7;) 8 (;@22;) 0 (;@30;) 23 (;@7;)
                                                              end
                                                              local.get 1
                                                              i32.const 8
                                                              i32.add
                                                              local.get 1
                                                              i32.const 140
                                                              i32.add
                                                              call $_RNvMCskpYeTZolUQS_4jsonNtB2_5Lexer4hex4
                                                              local.get 1
                                                              i32.load offset=8
                                                              i32.const 1
                                                              i32.and
                                                              br_if 22 (;@7;)
                                                              block ;; label = @30
                                                                local.get 1
                                                                i32.load offset=12
                                                                local.tee 9
                                                                i32.const -1024
                                                                i32.and
                                                                local.tee 14
                                                                i32.const 55296
                                                                i32.eq
                                                                br_if 0 (;@30;)
                                                                local.get 14
                                                                i32.const 56320
                                                                i32.eq
                                                                br_if 23 (;@7;)
                                                                br 11 (;@19;)
                                                              end
                                                              local.get 1
                                                              i32.load offset=148
                                                              local.tee 14
                                                              local.get 1
                                                              i32.load offset=144
                                                              local.tee 15
                                                              i32.ge_u
                                                              br_if 22 (;@7;)
                                                              local.get 1
                                                              i32.load offset=140
                                                              local.tee 11
                                                              local.get 14
                                                              i32.add
                                                              i32.load8_u
                                                              i32.const 92
                                                              i32.ne
                                                              br_if 22 (;@7;)
                                                              local.get 14
                                                              i32.const 1
                                                              i32.add
                                                              local.tee 13
                                                              local.get 15
                                                              i32.ge_u
                                                              br_if 22 (;@7;)
                                                              local.get 11
                                                              local.get 13
                                                              i32.add
                                                              i32.load8_u
                                                              i32.const 117
                                                              i32.ne
                                                              br_if 22 (;@7;)
                                                              local.get 1
                                                              local.get 14
                                                              i32.const 2
                                                              i32.add
                                                              i32.store offset=148
                                                              local.get 1
                                                              local.get 1
                                                              i32.const 140
                                                              i32.add
                                                              call $_RNvMCskpYeTZolUQS_4jsonNtB2_5Lexer4hex4
                                                              local.get 1
                                                              i32.load
                                                              i32.const 1
                                                              i32.and
                                                              br_if 22 (;@7;)
                                                              local.get 1
                                                              i32.load offset=4
                                                              local.tee 14
                                                              i32.const -1024
                                                              i32.and
                                                              i32.const 56320
                                                              i32.ne
                                                              br_if 22 (;@7;)
                                                              local.get 9
                                                              i32.const 10
                                                              i32.shl
                                                              local.get 14
                                                              i32.add
                                                              i32.const -56613888
                                                              i32.add
                                                              local.set 9
                                                              br 10 (;@19;)
                                                            end
                                                            block ;; label = @29
                                                              local.get 19
                                                              local.get 1
                                                              i32.load offset=184
                                                              i32.ne
                                                              br_if 0 (;@29;)
                                                              local.get 1
                                                              i32.const 184
                                                              i32.add
                                                              call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                            end
                                                            local.get 1
                                                            i32.load offset=188
                                                            local.get 19
                                                            i32.add
                                                            i32.const 34
                                                            i32.store8
                                                            br 7 (;@21;)
                                                          end
                                                          block ;; label = @28
                                                            local.get 19
                                                            local.get 1
                                                            i32.load offset=184
                                                            i32.ne
                                                            br_if 0 (;@28;)
                                                            local.get 1
                                                            i32.const 184
                                                            i32.add
                                                            call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                          end
                                                          local.get 1
                                                          i32.load offset=188
                                                          local.get 19
                                                          i32.add
                                                          i32.const 92
                                                          i32.store8
                                                          br 6 (;@21;)
                                                        end
                                                        block ;; label = @27
                                                          local.get 19
                                                          local.get 1
                                                          i32.load offset=184
                                                          i32.ne
                                                          br_if 0 (;@27;)
                                                          local.get 1
                                                          i32.const 184
                                                          i32.add
                                                          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                        end
                                                        local.get 1
                                                        i32.load offset=188
                                                        local.get 19
                                                        i32.add
                                                        i32.const 47
                                                        i32.store8
                                                        br 5 (;@21;)
                                                      end
                                                      block ;; label = @26
                                                        local.get 19
                                                        local.get 1
                                                        i32.load offset=184
                                                        i32.ne
                                                        br_if 0 (;@26;)
                                                        local.get 1
                                                        i32.const 184
                                                        i32.add
                                                        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                      end
                                                      local.get 1
                                                      i32.load offset=188
                                                      local.get 19
                                                      i32.add
                                                      i32.const 8
                                                      i32.store8
                                                      br 4 (;@21;)
                                                    end
                                                    block ;; label = @25
                                                      local.get 19
                                                      local.get 1
                                                      i32.load offset=184
                                                      i32.ne
                                                      br_if 0 (;@25;)
                                                      local.get 1
                                                      i32.const 184
                                                      i32.add
                                                      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                    end
                                                    local.get 1
                                                    i32.load offset=188
                                                    local.get 19
                                                    i32.add
                                                    i32.const 12
                                                    i32.store8
                                                    br 3 (;@21;)
                                                  end
                                                  block ;; label = @24
                                                    local.get 19
                                                    local.get 1
                                                    i32.load offset=184
                                                    i32.ne
                                                    br_if 0 (;@24;)
                                                    local.get 1
                                                    i32.const 184
                                                    i32.add
                                                    call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                  end
                                                  local.get 1
                                                  i32.load offset=188
                                                  local.get 19
                                                  i32.add
                                                  i32.const 10
                                                  i32.store8
                                                  br 2 (;@21;)
                                                end
                                                block ;; label = @23
                                                  local.get 19
                                                  local.get 1
                                                  i32.load offset=184
                                                  i32.ne
                                                  br_if 0 (;@23;)
                                                  local.get 1
                                                  i32.const 184
                                                  i32.add
                                                  call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                                end
                                                local.get 1
                                                i32.load offset=188
                                                local.get 19
                                                i32.add
                                                i32.const 13
                                                i32.store8
                                                br 1 (;@21;)
                                              end
                                              block ;; label = @22
                                                local.get 19
                                                local.get 1
                                                i32.load offset=184
                                                i32.ne
                                                br_if 0 (;@22;)
                                                local.get 1
                                                i32.const 184
                                                i32.add
                                                call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                                              end
                                              local.get 1
                                              i32.load offset=188
                                              local.get 19
                                              i32.add
                                              i32.const 9
                                              i32.store8
                                            end
                                            local.get 1
                                            local.get 19
                                            i32.const 1
                                            i32.add
                                            local.tee 19
                                            i32.store offset=192
                                            br 2 (;@18;)
                                          end
                                          local.get 1
                                          i32.load offset=184
                                          local.set 20
                                          local.get 1
                                          i32.const 32
                                          i32.add
                                          local.get 1
                                          i32.load offset=188
                                          local.tee 21
                                          local.get 19
                                          call $_RNvNtNtCs8euEcNjxCEf_4core3str8converts9from_utf8
                                          local.get 1
                                          i32.load offset=32
                                          br_if 12 (;@7;)
                                          local.get 20
                                          i32.const -1
                                          i32.eq
                                          br_if 12 (;@7;)
                                          i32.const 6
                                          local.set 16
                                          i32.const 0
                                          local.set 17
                                          local.get 1
                                          i32.load offset=176
                                          local.set 12
                                          br 9 (;@10;)
                                        end
                                        local.get 9
                                        i32.const 55296
                                        i32.xor
                                        i32.const -1114112
                                        i32.add
                                        i32.const -1112064
                                        i32.lt_u
                                        br_if 11 (;@7;)
                                        local.get 1
                                        i32.const 0
                                        i32.store offset=32
                                        block ;; label = @19
                                          block ;; label = @20
                                            local.get 9
                                            i32.const 128
                                            i32.lt_u
                                            br_if 0 (;@20;)
                                            local.get 9
                                            i32.const 63
                                            i32.and
                                            i32.const -128
                                            i32.or
                                            local.set 14
                                            local.get 9
                                            i32.const 6
                                            i32.shr_u
                                            local.set 15
                                            block ;; label = @21
                                              local.get 9
                                              i32.const 2048
                                              i32.ge_u
                                              br_if 0 (;@21;)
                                              local.get 1
                                              local.get 14
                                              i32.store8 offset=33
                                              local.get 1
                                              local.get 15
                                              i32.const 192
                                              i32.or
                                              i32.store8 offset=32
                                              i32.const 2
                                              local.set 9
                                              br 2 (;@19;)
                                            end
                                            local.get 9
                                            i32.const 12
                                            i32.shr_u
                                            local.set 11
                                            local.get 15
                                            i32.const 63
                                            i32.and
                                            i32.const -128
                                            i32.or
                                            local.set 15
                                            block ;; label = @21
                                              local.get 9
                                              i32.const 65535
                                              i32.gt_u
                                              br_if 0 (;@21;)
                                              local.get 1
                                              local.get 14
                                              i32.store8 offset=34
                                              local.get 1
                                              local.get 15
                                              i32.store8 offset=33
                                              local.get 1
                                              local.get 11
                                              i32.const 224
                                              i32.or
                                              i32.store8 offset=32
                                              i32.const 3
                                              local.set 9
                                              br 2 (;@19;)
                                            end
                                            local.get 1
                                            local.get 14
                                            i32.store8 offset=35
                                            local.get 1
                                            local.get 15
                                            i32.store8 offset=34
                                            local.get 1
                                            local.get 11
                                            i32.const 63
                                            i32.and
                                            i32.const -128
                                            i32.or
                                            i32.store8 offset=33
                                            local.get 1
                                            local.get 9
                                            i32.const 18
                                            i32.shr_u
                                            i32.const -16
                                            i32.or
                                            i32.store8 offset=32
                                            i32.const 4
                                            local.set 9
                                            br 1 (;@19;)
                                          end
                                          local.get 1
                                          local.get 9
                                          i32.store8 offset=32
                                          i32.const 1
                                          local.set 9
                                        end
                                        block ;; label = @19
                                          local.get 9
                                          local.get 1
                                          i32.load offset=184
                                          local.get 19
                                          i32.sub
                                          i32.le_u
                                          br_if 0 (;@19;)
                                          local.get 1
                                          i32.const 184
                                          i32.add
                                          local.get 19
                                          local.get 9
                                          call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
                                          local.get 1
                                          i32.load offset=192
                                          local.set 19
                                        end
                                        block ;; label = @19
                                          local.get 9
                                          i32.eqz
                                          br_if 0 (;@19;)
                                          local.get 1
                                          i32.load offset=188
                                          local.get 19
                                          i32.add
                                          local.get 1
                                          i32.const 32
                                          i32.add
                                          local.get 9
                                          memory.copy
                                        end
                                        local.get 1
                                        local.get 1
                                        i32.load offset=192
                                        local.get 9
                                        i32.add
                                        local.tee 19
                                        i32.store offset=192
                                        local.get 1
                                        i32.load offset=148
                                        local.set 14
                                        local.get 1
                                        i32.load offset=144
                                        local.set 15
                                      end
                                      local.get 14
                                      local.get 15
                                      i32.lt_u
                                      br_if 0 (;@17;)
                                      br 10 (;@7;)
                                    end
                                  end
                                  local.get 1
                                  local.get 9
                                  i32.store offset=148
                                  i32.const 1
                                  local.set 17
                                  local.get 14
                                  i32.const 1
                                  i32.add
                                  local.set 14
                                  br 2 (;@13;)
                                end
                                local.get 1
                                local.get 15
                                i32.store offset=148
                                local.get 15
                                local.set 9
                                local.get 15
                                local.set 13
                                local.get 15
                                local.set 14
                                br 5 (;@9;)
                              end
                              local.get 1
                              local.get 9
                              i32.store offset=148
                              i32.const 0
                              local.set 17
                              local.get 18
                              i32.const -48
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 10
                              i32.ge_u
                              br_if 1 (;@12;)
                            end
                            local.get 15
                            local.get 14
                            i32.le_u
                            br_if 5 (;@7;)
                            local.get 11
                            local.get 14
                            i32.add
                            i32.load8_u
                            i32.const -48
                            i32.add
                            local.tee 9
                            i32.const 255
                            i32.and
                            i32.const 9
                            i32.gt_u
                            br_if 5 (;@7;)
                            local.get 9
                            i64.extend_i32_u
                            i64.const 255
                            i64.and
                            local.set 0
                            block ;; label = @13
                              block ;; label = @14
                                local.get 15
                                local.get 14
                                i32.sub
                                i32.const 1
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 14
                                i32.const 1
                                i32.add
                                local.set 9
                                loop ;; label = @15
                                  local.get 11
                                  local.get 9
                                  i32.add
                                  i32.load8_u
                                  i32.const -48
                                  i32.add
                                  local.tee 14
                                  i32.const 255
                                  i32.and
                                  i32.const 9
                                  i32.gt_u
                                  br_if 2 (;@13;)
                                  local.get 0
                                  i64.const 10
                                  i64.mul
                                  local.get 14
                                  i64.extend_i32_u
                                  i64.const 255
                                  i64.and
                                  i64.add
                                  local.set 0
                                  local.get 15
                                  local.get 9
                                  i32.const 1
                                  i32.add
                                  local.tee 9
                                  i32.ne
                                  br_if 0 (;@15;)
                                end
                              end
                              local.get 15
                              local.set 9
                            end
                            local.get 1
                            local.get 9
                            i32.store offset=148
                            i64.const 0
                            local.get 0
                            i64.sub
                            local.get 0
                            local.get 17
                            select
                            local.tee 0
                            i32.wrap_i64
                            local.set 21
                            local.get 0
                            i64.const 32
                            i64.shr_u
                            i32.wrap_i64
                            local.set 19
                            i32.const 7
                            local.set 16
                            br 1 (;@11;)
                          end
                          local.get 18
                          i32.const -97
                          i32.add
                          i32.const 255
                          i32.and
                          i32.const 25
                          i32.gt_u
                          br_if 4 (;@7;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 13
                              local.get 15
                              i32.lt_u
                              br_if 0 (;@13;)
                              local.get 13
                              local.set 14
                              br 1 (;@12;)
                            end
                            local.get 13
                            local.set 14
                            block ;; label = @13
                              local.get 11
                              local.get 13
                              i32.add
                              i32.load8_u
                              i32.const 223
                              i32.and
                              i32.const -65
                              i32.add
                              i32.const 255
                              i32.and
                              i32.const 25
                              i32.gt_u
                              br_if 0 (;@13;)
                              local.get 15
                              local.set 9
                              local.get 15
                              local.set 14
                              local.get 13
                              i32.const 1
                              i32.add
                              local.tee 18
                              local.get 15
                              i32.eq
                              br_if 0 (;@13;)
                              loop ;; label = @14
                                block ;; label = @15
                                  local.get 11
                                  local.get 18
                                  i32.add
                                  i32.load8_u
                                  i32.const 223
                                  i32.and
                                  i32.const -65
                                  i32.add
                                  i32.const 255
                                  i32.and
                                  i32.const 26
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  local.get 18
                                  local.set 9
                                  local.get 18
                                  local.set 14
                                  br 2 (;@13;)
                                end
                                local.get 15
                                local.get 18
                                i32.const 1
                                i32.add
                                local.tee 18
                                i32.ne
                                br_if 0 (;@14;)
                              end
                              local.get 15
                              local.set 9
                              local.get 15
                              local.set 14
                            end
                            local.get 1
                            local.get 9
                            i32.store offset=148
                          end
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                local.get 14
                                local.get 13
                                i32.lt_u
                                br_if 0 (;@14;)
                                local.get 14
                                local.get 15
                                i32.gt_u
                                br_if 0 (;@14;)
                                local.get 11
                                local.get 13
                                i32.add
                                local.set 18
                                local.get 14
                                local.get 13
                                i32.sub
                                i32.const -4
                                i32.add
                                br_table 1 (;@13;) 2 (;@12;) 7 (;@7;)
                              end
                              local.get 13
                              local.get 14
                              local.get 15
                              i32.const 1050192
                              call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                              unreachable
                            end
                            i32.const 0
                            local.set 17
                            block ;; label = @13
                              local.get 18
                              i32.load align=1
                              i32.const 1702195828
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 8
                              local.set 16
                              local.get 14
                              local.set 13
                              br 4 (;@9;)
                            end
                            local.get 18
                            i32.load align=1
                            i32.const 1819047278
                            i32.ne
                            br_if 5 (;@7;)
                            i32.const 10
                            local.set 16
                            local.get 14
                            local.set 13
                            br 3 (;@9;)
                          end
                          local.get 18
                          i32.load align=1
                          i32.const 1936482662
                          i32.xor
                          local.get 18
                          i32.const 4
                          i32.add
                          i32.load8_u
                          i32.const 101
                          i32.xor
                          i32.or
                          br_if 4 (;@7;)
                          i32.const 9
                          local.set 16
                          i32.const 0
                          local.set 17
                          local.get 14
                          local.set 13
                          br 2 (;@9;)
                        end
                        i32.const 0
                        local.set 17
                      end
                      local.get 9
                      local.set 13
                      local.get 9
                      local.set 14
                    end
                    block ;; label = @9
                      local.get 12
                      local.get 1
                      i32.load offset=168
                      i32.ne
                      br_if 0 (;@9;)
                      local.get 1
                      i32.const 168
                      i32.add
                      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecNtCskpYeTZolUQS_4json5TokenE8grow_oneBO_
                    end
                    local.get 1
                    i32.load offset=172
                    local.get 12
                    i32.const 4
                    i32.shl
                    i32.add
                    local.tee 18
                    local.get 19
                    i32.store offset=12
                    local.get 18
                    local.get 21
                    i32.store offset=8
                    local.get 18
                    local.get 20
                    i32.store offset=4
                    local.get 18
                    local.get 16
                    i32.store
                    local.get 1
                    local.get 12
                    i32.const 1
                    i32.add
                    local.tee 12
                    i32.store offset=176
                    local.get 17
                    i32.eqz
                    br_if 0 (;@8;)
                  end
                  local.get 1
                  i32.load offset=168
                  local.tee 9
                  i32.const -1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 1
                  i64.load offset=172 align=4
                  local.set 0
                  local.get 1
                  local.get 9
                  i32.store offset=152
                  local.get 1
                  local.get 0
                  i64.store offset=156 align=4
                  block ;; label = @8
                    local.get 0
                    i64.const 4294967295
                    i64.gt_u
                    br_if 0 (;@8;)
                    block ;; label = @9
                      block ;; label = @10
                        local.get 9
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        i32.wrap_i64
                        local.set 9
                        br 1 (;@9;)
                      end
                      local.get 1
                      i32.const 152
                      i32.add
                      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecNtCskpYeTZolUQS_4json5TokenE8grow_oneBO_
                      local.get 1
                      i32.load offset=156
                      local.set 9
                    end
                    local.get 9
                    i32.const 11
                    i32.store
                    local.get 1
                    i32.const 1
                    i32.store offset=160
                  end
                  local.get 1
                  local.get 1
                  i32.load offset=160
                  i32.store offset=176
                  local.get 1
                  local.get 1
                  i64.load offset=152 align=4
                  i64.store offset=168
                  local.get 1
                  i32.const 0
                  i32.store offset=180
                  local.get 1
                  i32.const 32
                  i32.add
                  local.get 1
                  i32.const 168
                  i32.add
                  call $_RNvMs0_CskpYeTZolUQS_4jsonNtB5_6Parser11parse_value
                  local.get 1
                  i32.load offset=32
                  i32.const -1
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 1
                  i64.load offset=40
                  i64.store offset=192
                  local.get 1
                  local.get 1
                  i64.load offset=32
                  i64.store offset=184
                  local.get 1
                  i32.load offset=180
                  local.tee 14
                  local.get 1
                  i32.load offset=176
                  local.tee 9
                  i32.const -1
                  i32.add
                  local.tee 15
                  local.get 14
                  local.get 15
                  i32.lt_u
                  select
                  local.tee 14
                  local.get 9
                  i32.ge_u
                  br_if 3 (;@4;)
                  local.get 1
                  i32.load offset=172
                  local.get 14
                  i32.const 4
                  i32.shl
                  i32.add
                  i32.load
                  i32.const 11
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 1
                  i32.load offset=184
                  local.tee 9
                  i32.const -2147483648
                  i32.xor
                  i32.const 5
                  local.get 9
                  i32.const 0
                  i32.lt_s
                  select
                  local.tee 9
                  i32.const 4
                  i32.lt_u
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 9
                    i32.const 4
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 184
                    i32.add
                    call $_RNvXsp_NtCseFzb18zECBI_5alloc3vecINtB5_3VecTNtNtB7_6string6StringNtCskpYeTZolUQS_4json5ValueEENtNtNtCs8euEcNjxCEf_4core3ops4drop4Drop4dropB13_
                    i32.const -2147483648
                    local.set 19
                    br 3 (;@5;)
                  end
                  local.get 1
                  i32.load offset=196
                  local.tee 13
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  i32.load offset=192
                  local.set 18
                  i32.const 0
                  local.set 15
                  loop ;; label = @8
                    block ;; label = @9
                      local.get 18
                      local.get 15
                      i32.const 4
                      i32.shl
                      i32.add
                      local.tee 11
                      i32.load
                      local.tee 9
                      i32.const -2147483648
                      i32.xor
                      i32.const 5
                      local.get 9
                      i32.const 0
                      i32.lt_s
                      select
                      local.tee 9
                      i32.const 4
                      i32.lt_u
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 9
                        i32.const 4
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 11
                        i32.load offset=8
                        local.tee 14
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 11
                        i32.load offset=4
                        local.set 9
                        loop ;; label = @11
                          local.get 9
                          call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueTNtNtCseFzb18zECBI_5alloc6string6StringNtCskpYeTZolUQS_4json5ValueEEB1g_.llvm.12285963892569529774
                          local.get 9
                          i32.const 32
                          i32.add
                          local.set 9
                          local.get 14
                          i32.const -1
                          i32.add
                          local.tee 14
                          br_if 0 (;@11;)
                          br 2 (;@9;)
                        end
                      end
                      local.get 11
                      i32.const 4
                      i32.add
                      call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueINtNtCseFzb18zECBI_5alloc3vec3VecNtCskpYeTZolUQS_4json5ValueEEB1a_.llvm.12285963892569529774
                    end
                    local.get 13
                    local.get 15
                    i32.const 1
                    i32.add
                    local.tee 15
                    i32.ne
                    br_if 0 (;@8;)
                  end
                end
                i32.const -2147483648
                local.set 19
                br 1 (;@5;)
              end
              local.get 1
              local.get 6
              i64.load align=4
              i64.store offset=32
              local.get 1
              local.get 6
              i32.load offset=8
              i32.store offset=40
              block ;; label = @6
                local.get 1
                i32.load offset=184
                local.tee 19
                i32.const -1
                i32.ne
                br_if 0 (;@6;)
                i32.const -2147483648
                local.set 19
                br 1 (;@5;)
              end
              local.get 5
              local.get 1
              i32.load offset=40
              i32.store offset=8
              local.get 5
              local.get 1
              i64.load offset=32
              i64.store align=4
            end
            local.get 1
            local.get 19
            i32.store offset=16
            local.get 4
            i64.const 0
            i64.store
            local.get 4
            i64.const 0
            i64.store offset=8
            local.get 1
            i64.const 0
            i64.store offset=88
            local.get 1
            i64.const 0
            i64.store offset=80
            local.get 1
            i64.const 0
            i64.store offset=72
            local.get 1
            i64.const 0
            i64.store offset=64
            local.get 1
            i64.const 0
            i64.store offset=56
            local.get 1
            i64.const 0
            i64.store offset=48
            local.get 1
            i64.const 0
            i64.store offset=40
            local.get 1
            i64.const 0
            i64.store offset=32
            local.get 1
            i64.const -9223372036854775808
            i64.store offset=104
            local.get 1
            i64.const 9223372036854775807
            i64.store offset=96
            local.get 1
            i64.const -3750763034362895579
            i64.store offset=128
            local.get 1
            i32.const 16
            i32.add
            i64.const 0
            local.get 1
            i32.const 32
            i32.add
            call $_RNvCskpYeTZolUQS_4json4walk
            local.get 1
            i32.const 0
            i32.store offset=176
            local.get 1
            i64.const 34359738368
            i64.store offset=168 align=4
            local.get 1
            i32.const 0
            i32.store offset=192
            local.get 1
            i64.const 4294967296
            i64.store offset=184 align=4
            local.get 1
            i32.const 16
            i32.add
            local.get 1
            i32.const 184
            i32.add
            local.get 1
            i32.const 168
            i32.add
            local.get 3
            call $_RNvCskpYeTZolUQS_4json13collect_paths
            block ;; label = @5
              local.get 1
              i32.load offset=176
              local.tee 9
              i32.const 2
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              i32.load offset=172
              local.set 15
              block ;; label = @6
                local.get 9
                i32.const 21
                i32.lt_u
                br_if 0 (;@6;)
                local.get 15
                local.get 9
                local.get 1
                i32.const 207
                i32.add
                call $_RINvNtNtNtCs8euEcNjxCEf_4core5slice4sort6stable14driftsort_mainTNtNtCseFzb18zECBI_5alloc6string6StringxENvYBZ_NtNtB8_3cmp10PartialOrd2ltINtNtB14_3vec3VecBZ_EECskpYeTZolUQS_4json
                br 1 (;@5;)
              end
              i32.const 24
              local.set 14
              local.get 9
              i32.const 24
              i32.mul
              local.set 9
              loop ;; label = @6
                local.get 15
                local.get 15
                local.get 14
                i32.add
                call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort6shared9smallsort11insert_tailTNtNtCseFzb18zECBI_5alloc6string6StringxENvYB18_NtNtBa_3cmp10PartialOrd2ltECskpYeTZolUQS_4json.llvm.12285963892569529774
                local.get 9
                local.get 14
                i32.const 24
                i32.add
                local.tee 14
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 1
            local.get 10
            i64.extend_i32_u
            i64.store offset=152
            i32.const 0
            i32.const 16384
            i32.const 0
            i32.load offset=1067256
            local.tee 14
            i32.sub
            local.tee 9
            local.get 9
            i32.const 16384
            i32.gt_u
            select
            local.tee 9
            i32.const 8
            local.get 9
            i32.const 8
            i32.lt_u
            select
            local.tee 15
            local.get 14
            i32.add
            local.tee 9
            local.get 14
            i32.lt_u
            br_if 1 (;@3;)
            local.get 9
            i32.const 16385
            i32.ge_u
            br_if 1 (;@3;)
            block ;; label = @5
              local.get 15
              i32.eqz
              br_if 0 (;@5;)
              local.get 14
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 15
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=32
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=40
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=48
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=56
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=64
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=72
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=80
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=88
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=96
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=104
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=112
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=120
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i64.load offset=128
            local.tee 0
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            local.get 1
            local.get 1
            i32.load offset=176
            local.tee 18
            i32.const 12
            local.get 18
            i32.const 12
            i32.lt_u
            select
            local.tee 12
            i64.extend_i32_u
            i64.store offset=152
            block ;; label = @5
              i32.const 16384
              local.get 14
              local.get 9
              i32.add
              local.tee 9
              i32.sub
              local.tee 14
              i32.const 8
              local.get 14
              i32.const 8
              i32.lt_u
              select
              local.tee 14
              i32.eqz
              br_if 0 (;@5;)
              local.get 9
              i32.const 1050872
              i32.add
              local.get 1
              i32.const 152
              i32.add
              local.get 14
              memory.copy
            end
            i32.const 0
            local.get 14
            local.get 9
            i32.add
            local.tee 15
            i32.store offset=1067256
            local.get 18
            i32.eqz
            br_if 2 (;@2;)
            local.get 1
            i32.load offset=172
            i32.const 16
            i32.add
            local.set 9
            i32.const 0
            local.set 11
            loop ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 18
                  local.get 11
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 9
                  i32.const -8
                  i32.add
                  i32.load
                  local.tee 14
                  i32.const 0
                  i32.const 16384
                  local.get 15
                  i32.sub
                  local.tee 13
                  local.get 13
                  i32.const 16384
                  i32.gt_u
                  select
                  local.tee 13
                  local.get 14
                  local.get 13
                  i32.lt_u
                  select
                  local.tee 13
                  local.get 15
                  i32.add
                  local.tee 14
                  i32.const 16385
                  i32.lt_u
                  br_if 1 (;@6;)
                  local.get 15
                  local.get 14
                  i32.const 16384
                  i32.const 1049548
                  call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                  unreachable
                end
                local.get 18
                local.get 18
                i32.const 1050160
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              block ;; label = @6
                local.get 13
                i32.eqz
                br_if 0 (;@6;)
                local.get 15
                i32.const 1050872
                i32.add
                local.get 9
                i32.const -12
                i32.add
                i32.load
                local.get 13
                memory.copy
              end
              i32.const 0
              local.get 14
              i32.store offset=1067256
              local.get 1
              local.get 9
              i64.load
              i64.store offset=152
              block ;; label = @6
                i32.const 16384
                local.get 14
                i32.sub
                local.tee 15
                i32.const 8
                local.get 15
                i32.const 8
                i32.lt_u
                select
                local.tee 15
                i32.eqz
                br_if 0 (;@6;)
                local.get 14
                i32.const 1050872
                i32.add
                local.get 1
                i32.const 152
                i32.add
                local.get 15
                memory.copy
              end
              i32.const 0
              local.get 15
              local.get 14
              i32.add
              local.tee 15
              i32.store offset=1067256
              local.get 9
              i32.const 24
              i32.add
              local.set 9
              local.get 12
              local.get 11
              i32.const 1
              i32.add
              local.tee 11
              i32.ne
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          local.get 14
          local.get 9
          i32.const 1050256
          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
          unreachable
        end
        local.get 14
        local.get 9
        i32.const 16384
        i32.const 1049548
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      local.get 2
      i64.const 11
      i64.rotl
      local.get 18
      i64.extend_i32_u
      i64.const -7046029254386353131
      i64.mul
      i64.xor
      local.set 2
      block ;; label = @2
        local.get 19
        i32.const -2147483648
        i32.xor
        i32.const 5
        local.get 19
        i32.const 0
        i32.lt_s
        select
        local.tee 9
        i32.const 4
        i32.lt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 9
          i32.const 4
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.const 16
          i32.add
          call $_RNvXsp_NtCseFzb18zECBI_5alloc3vecINtB5_3VecTNtNtB7_6string6StringNtCskpYeTZolUQS_4json5ValueEENtNtNtCs8euEcNjxCEf_4core3ops4drop4Drop4dropB13_
          br 1 (;@2;)
        end
        local.get 1
        i32.load offset=28
        local.tee 13
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=24
        local.set 18
        i32.const 0
        local.set 15
        loop ;; label = @3
          block ;; label = @4
            local.get 18
            local.get 15
            i32.const 4
            i32.shl
            i32.add
            local.tee 11
            i32.load
            local.tee 9
            i32.const -2147483648
            i32.xor
            i32.const 5
            local.get 9
            i32.const 0
            i32.lt_s
            select
            local.tee 9
            i32.const 4
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 9
              i32.const 4
              i32.eq
              br_if 0 (;@5;)
              local.get 11
              i32.load offset=8
              local.tee 14
              i32.eqz
              br_if 1 (;@4;)
              local.get 11
              i32.load offset=4
              local.set 9
              loop ;; label = @6
                local.get 9
                call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueTNtNtCseFzb18zECBI_5alloc6string6StringNtCskpYeTZolUQS_4json5ValueEEB1g_.llvm.12285963892569529774
                local.get 9
                i32.const 32
                i32.add
                local.set 9
                local.get 14
                i32.const -1
                i32.add
                local.tee 14
                br_if 0 (;@6;)
                br 2 (;@4;)
              end
            end
            local.get 11
            i32.const 4
            i32.add
            call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueINtNtCseFzb18zECBI_5alloc3vec3VecNtCskpYeTZolUQS_4json5ValueEEB1a_.llvm.12285963892569529774
          end
          local.get 13
          local.get 15
          i32.const 1
          i32.add
          local.tee 15
          i32.ne
          br_if 0 (;@3;)
        end
      end
      local.get 2
      local.get 0
      i64.xor
      local.set 2
      local.get 8
      i32.const 1
      i32.add
      local.tee 8
      i32.const 4
      i32.ne
      br_if 0 (;@1;)
    end
    local.get 1
    local.get 2
    i64.store offset=32
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1067256
        local.tee 9
        i32.sub
        local.tee 14
        local.get 14
        i32.const 16384
        i32.gt_u
        select
        local.tee 14
        i32.const 8
        local.get 14
        i32.const 8
        i32.lt_u
        select
        local.tee 15
        local.get 9
        i32.add
        local.tee 14
        local.get 9
        i32.lt_u
        br_if 0 (;@2;)
        local.get 14
        i32.const 16385
        i32.lt_u
        br_if 1 (;@1;)
      end
      local.get 9
      local.get 14
      i32.const 16384
      i32.const 1049548
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    block ;; label = @1
      local.get 15
      i32.eqz
      br_if 0 (;@1;)
      local.get 9
      i32.const 1050872
      i32.add
      local.get 1
      i32.const 32
      i32.add
      local.get 15
      memory.copy
    end
    i32.const 0
    local.get 14
    i32.store offset=1067256
    local.get 1
    i32.const 208
    i32.add
    global.set $__stack_pointer
    local.get 2
  )

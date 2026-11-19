(func $_RNvMs_CsbB8S1oLEkkY_4exprNtB4_6Parser10parse_expr (;10;) (type 3) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 112
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
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
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=4
            local.get 6
            i32.const 4
            i32.shl
            i32.add
            local.tee 7
            i32.load
            local.tee 6
            i32.const -2
            i32.add
            i32.const 11
            i32.lt_u
            br_if 3 (;@1;)
            local.get 6
            br_table 1 (;@3;) 2 (;@2;) 1 (;@3;)
          end
          local.get 6
          local.get 5
          i32.const 1049592
          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
          unreachable
        end
        local.get 3
        local.get 7
        i64.load offset=8
        i64.store offset=56
        br 1 (;@1;)
      end
      local.get 3
      i32.const 48
      i32.add
      i32.const 4
      i32.or
      local.get 7
      i32.const 4
      i32.add
      call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
    end
    block ;; label = @1
      local.get 4
      i32.const 1
      i32.add
      local.tee 4
      local.get 5
      i32.ge_u
      br_if 0 (;@1;)
      local.get 1
      local.get 4
      i32.store offset=12
    end
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
                            local.get 6
                            br_table 3 (;@9;) 0 (;@12;) 10 (;@2;) 10 (;@2;) 1 (;@11;) 10 (;@2;) 10 (;@2;) 10 (;@2;) 2 (;@10;) 10 (;@2;)
                          end
                          i32.const 1
                          local.set 4
                          local.get 3
                          i64.load offset=56
                          local.set 8
                          local.get 3
                          i32.load offset=52
                          local.set 5
                          br 3 (;@8;)
                        end
                        local.get 3
                        i32.const 96
                        i32.add
                        local.get 1
                        i32.const 30
                        call $_RNvMs_CsbB8S1oLEkkY_4exprNtB4_6Parser10parse_expr
                        local.get 3
                        i32.load8_u offset=96
                        i32.const 255
                        i32.eq
                        br_if 8 (;@2;)
                        local.get 3
                        local.get 3
                        i64.load offset=104
                        i64.store offset=72
                        local.get 3
                        local.get 3
                        i64.load offset=96
                        i64.store offset=64
                        call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load8_u offset=1066512
                            br_if 0 (;@12;)
                            block ;; label = @13
                              memory.size
                              i32.const 16
                              i32.shl
                              local.tee 4
                              br_if 0 (;@13;)
                              i32.const 65536
                              local.set 4
                              i32.const 1
                              memory.grow
                              i32.const -1
                              i32.eq
                              br_if 6 (;@7;)
                            end
                            i32.const 0
                            local.get 4
                            i32.store offset=1066508
                            i32.const 0
                            local.get 4
                            i32.store offset=1066504
                            i32.const 0
                            i32.const 1
                            i32.store8 offset=1066512
                            local.get 4
                            i32.const 16
                            i32.or
                            local.set 6
                            local.get 4
                            local.set 5
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i32.load offset=1066504
                          local.tee 5
                          i32.const -24
                          i32.gt_u
                          br_if 4 (;@7;)
                          local.get 5
                          i32.const 7
                          i32.add
                          i32.const -8
                          i32.and
                          local.tee 5
                          i32.const 16
                          i32.add
                          local.set 6
                          i32.const 0
                          i32.load offset=1066508
                          local.set 4
                        end
                        block ;; label = @11
                          local.get 6
                          local.get 4
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 6
                          local.get 4
                          i32.sub
                          local.tee 4
                          i32.const 16
                          i32.shr_u
                          local.get 4
                          i32.const 65535
                          i32.and
                          i32.const 0
                          i32.ne
                          i32.add
                          local.tee 4
                          memory.grow
                          local.tee 7
                          i32.const -1
                          i32.eq
                          br_if 4 (;@7;)
                          i32.const 0
                          local.get 7
                          local.get 4
                          i32.add
                          i32.const 16
                          i32.shl
                          i32.store offset=1066508
                        end
                        i32.const 0
                        local.get 6
                        i32.store offset=1066504
                        local.get 5
                        i32.eqz
                        br_if 3 (;@7;)
                        local.get 5
                        local.get 3
                        i64.load offset=72
                        i64.store offset=8
                        local.get 5
                        local.get 3
                        i64.load offset=64
                        i64.store
                        i32.const 2
                        local.set 4
                        br 2 (;@8;)
                      end
                      local.get 3
                      i32.const 96
                      i32.add
                      local.get 1
                      i32.const 0
                      call $_RNvMs_CsbB8S1oLEkkY_4exprNtB4_6Parser10parse_expr
                      local.get 3
                      i32.load8_u offset=96
                      i32.const 255
                      i32.eq
                      br_if 7 (;@2;)
                      local.get 3
                      local.get 3
                      i64.load offset=104
                      i64.store offset=88
                      local.get 3
                      local.get 3
                      i64.load offset=96
                      i64.store offset=80
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
                      br_if 3 (;@6;)
                      block ;; label = @10
                        local.get 1
                        i32.load offset=4
                        local.get 6
                        i32.const 4
                        i32.shl
                        i32.add
                        local.tee 7
                        i32.load
                        local.tee 6
                        i32.const 1
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 96
                        i32.add
                        i32.const 4
                        i32.or
                        local.get 7
                        i32.const 4
                        i32.add
                        call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                      end
                      block ;; label = @10
                        local.get 4
                        i32.const 1
                        i32.add
                        local.tee 4
                        local.get 5
                        i32.ge_u
                        br_if 0 (;@10;)
                        local.get 1
                        local.get 4
                        i32.store offset=12
                      end
                      block ;; label = @10
                        local.get 6
                        i32.const 9
                        i32.eq
                        br_if 0 (;@10;)
                        local.get 3
                        i32.const 80
                        i32.add
                        call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueNtCsbB8S1oLEkkY_4expr4ExprEBD_
                        br 8 (;@2;)
                      end
                      local.get 3
                      local.get 3
                      i32.load16_u offset=81 align=1
                      i32.store16 offset=12
                      local.get 3
                      local.get 3
                      i32.load8_u offset=83
                      i32.store8 offset=14
                      local.get 3
                      i32.load8_u offset=80
                      local.tee 4
                      i32.const 255
                      i32.eq
                      br_if 7 (;@2;)
                      local.get 3
                      i32.load offset=84
                      local.set 5
                      local.get 3
                      i64.load offset=88
                      local.set 8
                      br 1 (;@8;)
                    end
                    i32.const 0
                    local.set 4
                    local.get 3
                    i64.load offset=56
                    local.set 8
                  end
                  local.get 3
                  local.get 3
                  i32.load8_u offset=14
                  i32.store8 offset=35
                  local.get 3
                  local.get 3
                  i32.load16_u offset=12
                  i32.store16 offset=33 align=1
                  local.get 3
                  local.get 8
                  i64.store offset=40
                  local.get 3
                  local.get 5
                  i32.store offset=36
                  local.get 3
                  local.get 4
                  i32.store8 offset=32
                  block ;; label = @8
                    block ;; label = @9
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
                      br_if 0 (;@9;)
                      local.get 3
                      i32.const 96
                      i32.add
                      i32.const 4
                      i32.or
                      local.set 9
                      local.get 2
                      i32.const 255
                      i32.and
                      local.set 10
                      loop ;; label = @10
                        local.get 1
                        i32.load offset=4
                        local.tee 2
                        local.get 6
                        i32.const 4
                        i32.shl
                        i32.add
                        i32.load
                        i32.const -3
                        i32.add
                        local.tee 7
                        i32.const 5
                        i32.ge_u
                        br_if 2 (;@8;)
                        local.get 10
                        i64.const 86236203530
                        local.get 7
                        i32.const 3
                        i32.shl
                        i64.extend_i32_u
                        local.tee 8
                        i64.shr_u
                        i32.wrap_i64
                        i32.const 30
                        i32.and
                        i32.gt_u
                        br_if 2 (;@8;)
                        local.get 4
                        local.get 5
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
                        br_if 5 (;@5;)
                        i64.const 90548013835
                        local.get 8
                        i64.shr_u
                        local.set 8
                        block ;; label = @11
                          local.get 2
                          local.get 6
                          i32.const 4
                          i32.shl
                          i32.add
                          local.tee 6
                          i32.load
                          i32.const 1
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 9
                          local.get 6
                          i32.const 4
                          i32.add
                          call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
                        end
                        local.get 8
                        i32.wrap_i64
                        local.set 6
                        block ;; label = @11
                          local.get 4
                          i32.const 1
                          i32.add
                          local.tee 4
                          local.get 5
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 1
                          local.get 4
                          i32.store offset=12
                        end
                        local.get 3
                        i32.const 96
                        i32.add
                        local.get 1
                        local.get 6
                        call $_RNvMs_CsbB8S1oLEkkY_4exprNtB4_6Parser10parse_expr
                        block ;; label = @11
                          local.get 3
                          i32.load8_u offset=96
                          i32.const 255
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 0
                          i32.const 255
                          i32.store8
                          local.get 3
                          i32.const 32
                          i32.add
                          call $_RINvNtCs8euEcNjxCEf_4core3ptr9drop_glueNtCsbB8S1oLEkkY_4expr4ExprEBD_
                          br 10 (;@1;)
                        end
                        local.get 3
                        local.get 3
                        i64.load offset=104
                        i64.store offset=24
                        local.get 3
                        local.get 3
                        i64.load offset=96
                        i64.store offset=16
                        call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load8_u offset=1066512
                            br_if 0 (;@12;)
                            block ;; label = @13
                              memory.size
                              i32.const 16
                              i32.shl
                              local.tee 4
                              br_if 0 (;@13;)
                              i32.const 65536
                              local.set 4
                              i32.const 1
                              memory.grow
                              i32.const -1
                              i32.eq
                              br_if 9 (;@4;)
                            end
                            i32.const 0
                            local.get 4
                            i32.store offset=1066508
                            i32.const 0
                            local.get 4
                            i32.store offset=1066504
                            i32.const 0
                            i32.const 1
                            i32.store8 offset=1066512
                            local.get 4
                            i32.const 16
                            i32.or
                            local.set 6
                            local.get 4
                            local.set 5
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i32.load offset=1066504
                          local.tee 5
                          i32.const -24
                          i32.gt_u
                          br_if 7 (;@4;)
                          local.get 5
                          i32.const 7
                          i32.add
                          i32.const -8
                          i32.and
                          local.tee 5
                          i32.const 16
                          i32.add
                          local.set 6
                          i32.const 0
                          i32.load offset=1066508
                          local.set 4
                        end
                        block ;; label = @11
                          local.get 6
                          local.get 4
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 6
                          local.get 4
                          i32.sub
                          local.tee 4
                          i32.const 16
                          i32.shr_u
                          local.get 4
                          i32.const 65535
                          i32.and
                          i32.const 0
                          i32.ne
                          i32.add
                          local.tee 4
                          memory.grow
                          local.tee 2
                          i32.const -1
                          i32.eq
                          br_if 7 (;@4;)
                          i32.const 0
                          local.get 2
                          local.get 4
                          i32.add
                          i32.const 16
                          i32.shl
                          i32.store offset=1066508
                        end
                        i32.const 0
                        local.get 6
                        i32.store offset=1066504
                        local.get 5
                        i32.eqz
                        br_if 6 (;@4;)
                        local.get 5
                        local.get 3
                        i64.load offset=40
                        i64.store offset=8
                        local.get 5
                        local.get 3
                        i64.load offset=32
                        i64.store
                        call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load8_u offset=1066512
                            br_if 0 (;@12;)
                            block ;; label = @13
                              memory.size
                              i32.const 16
                              i32.shl
                              local.tee 6
                              br_if 0 (;@13;)
                              i32.const 65536
                              local.set 6
                              i32.const 1
                              memory.grow
                              i32.const -1
                              i32.eq
                              br_if 10 (;@3;)
                            end
                            i32.const 0
                            local.get 6
                            i32.store offset=1066508
                            i32.const 0
                            local.get 6
                            i32.store offset=1066504
                            i32.const 0
                            i32.const 1
                            i32.store8 offset=1066512
                            local.get 6
                            i32.const 16
                            i32.or
                            local.set 2
                            local.get 6
                            local.set 4
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i32.load offset=1066504
                          local.tee 4
                          i32.const -24
                          i32.gt_u
                          br_if 8 (;@3;)
                          local.get 4
                          i32.const 7
                          i32.add
                          i32.const -8
                          i32.and
                          local.tee 4
                          i32.const 16
                          i32.add
                          local.set 2
                          i32.const 0
                          i32.load offset=1066508
                          local.set 6
                        end
                        block ;; label = @11
                          local.get 2
                          local.get 6
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 2
                          local.get 6
                          i32.sub
                          local.tee 6
                          i32.const 16
                          i32.shr_u
                          local.get 6
                          i32.const 65535
                          i32.and
                          i32.const 0
                          i32.ne
                          i32.add
                          local.tee 6
                          memory.grow
                          local.tee 11
                          i32.const -1
                          i32.eq
                          br_if 8 (;@3;)
                          i32.const 0
                          local.get 11
                          local.get 6
                          i32.add
                          i32.const 16
                          i32.shl
                          i32.store offset=1066508
                        end
                        i32.const 0
                        local.get 2
                        i32.store offset=1066504
                        local.get 4
                        i32.eqz
                        br_if 7 (;@3;)
                        local.get 4
                        local.get 3
                        i64.load offset=24
                        i64.store offset=8
                        local.get 4
                        local.get 3
                        i64.load offset=16
                        i64.store
                        local.get 3
                        local.get 4
                        i32.store offset=40
                        local.get 3
                        local.get 5
                        i32.store offset=36
                        local.get 3
                        local.get 7
                        i32.store8 offset=33
                        local.get 3
                        i32.const 3
                        i32.store8 offset=32
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
                        br_if 0 (;@10;)
                      end
                    end
                    local.get 6
                    local.get 5
                    i32.const 1049592
                    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                    unreachable
                  end
                  local.get 0
                  local.get 3
                  i64.load offset=40
                  i64.store offset=8
                  local.get 0
                  local.get 3
                  i64.load offset=32
                  i64.store
                  br 6 (;@1;)
                end
                i32.const 8
                i32.const 16
                call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
                unreachable
              end
              local.get 6
              local.get 5
              i32.const 1049592
              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
              unreachable
            end
            local.get 6
            local.get 5
            i32.const 1049592
            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
            unreachable
          end
          i32.const 8
          i32.const 16
          call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
          unreachable
        end
        i32.const 8
        i32.const 16
        call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
        unreachable
      end
      local.get 0
      i32.const 255
      i32.store8
    end
    local.get 3
    i32.const 112
    i32.add
    global.set $__stack_pointer
  )

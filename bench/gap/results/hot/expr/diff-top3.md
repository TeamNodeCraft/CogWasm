# <expr::Parser>::parse_expr

- ours: 169,867 executed instructions (6.3% of the program)
- LLVM -O2: 159,040 executed instructions (23.8%)

## Ours

```wat
(func $_RNvMs_CsizV4RGem7kw_4exprNtB4_6Parser10parse_expr (;52;) (type 3) (param i32 i32 i32)
    (local i32 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 256
    i32.sub
    local.tee 33
    global.set $__stack_pointer
    local.get 2
    i32.const 255
    i32.and
    local.set 2
    local.get 33
    i32.const 32
    i32.add
    local.tee 3
    local.get 1
    call $_RNvMs_CsizV4RGem7kw_4exprNtB4_6Parser12parse_prefix
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i32.load8_u
              i32.const 255
              i32.eq
              br_table 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            br 3 (;@1;)
          end
          local.get 3
          i64.load
          local.set 4
          local.get 3
          i64.load offset=8
          local.set 5
          local.get 33
          i32.const 192
          i32.add
          local.tee 3
          local.get 4
          i64.store
          local.get 3
          local.get 5
          i64.store offset=8
          local.get 33
          i32.const 16
          i32.add
          local.tee 6
          local.get 3
          i64.load
          i64.store
          local.get 6
          local.get 5
          i64.store offset=8
          local.get 33
          i32.const 48
          i32.add
          local.tee 3
          local.get 6
          i64.load
          i64.store
          local.get 3
          local.get 5
          i64.store offset=8
          local.get 33
          local.tee 6
          local.get 3
          i64.load
          i64.store
          local.get 6
          local.get 5
          i64.store offset=8
          local.get 33
          i32.const 216
          i32.add
          local.set 3
          local.get 33
          i32.const 208
          i32.add
          local.set 7
          local.get 33
          i32.const 212
          i32.add
          local.set 8
          local.get 33
          i32.const 241
          i32.add
          local.set 9
          local.get 33
          i32.const 240
          i32.add
          local.set 10
          local.get 33
          i32.const 242
          i32.add
          local.set 11
          local.get 33
          i32.const 243
          i32.add
          local.set 12
          local.get 33
          i32.const 244
          i32.add
          local.set 13
          local.get 33
          i32.const 245
          i32.add
          local.set 14
          local.get 33
          i32.const 64
          i32.add
          local.set 15
          local.get 33
          i32.const 96
          i32.add
          local.set 16
          local.get 33
          i32.const 224
          i32.add
          local.set 17
          local.get 33
          i32.const 80
          i32.add
          local.set 18
          local.get 33
          i32.const 112
          i32.add
          local.set 19
          local.get 33
          i32.const 152
          i32.add
          local.set 20
          i32.const 1066408
          local.set 21
          local.get 33
          i32.const 144
          i32.add
          local.set 22
          i32.const 1066408
          local.set 23
          local.get 33
          i32.const 168
          i32.add
          local.set 24
          local.get 33
          i32.const 129
          i32.add
          local.set 25
          local.get 33
          i32.const 132
          i32.add
          local.set 26
          local.get 33
          i32.const 136
          i32.add
          local.set 27
          local.get 33
          i32.const 128
          i32.add
          local.set 28
          loop ;; label = @4
            local.get 1
            i32.load offset=8 align=1
            local.set 29
            local.get 1
            i32.load offset=12 align=1
            local.set 30
            local.get 1
            i32.load offset=4 align=1
            local.set 31
            local.get 3
            local.get 31
            i32.store align=1
            local.get 1
            i32.load offset=8 align=1
            local.set 32
            local.get 7
            local.get 31
            i32.store align=1
            local.get 8
            local.get 32
            i32.store align=1
            local.get 30
            local.tee 34
            local.get 29
            i32.const 1
            i32.sub
            local.tee 35
            local.get 34
            local.get 35
            i32.lt_u
            select
            local.tee 30
            local.get 32
            i32.lt_u
            if ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 7
                  i32.load align=1
                  local.get 30
                  i32.const 4
                  i32.shl
                  i32.add
                  i32.load align=1
                  local.tee 30
                  i32.const 3
                  i32.ge_u
                  if ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 30
                                  i32.const -3
                                  i32.add
                                  br_table 1 (;@14;) 2 (;@13;) 3 (;@12;) 4 (;@11;) 5 (;@10;) 0 (;@15;)
                                end
                                br 7 (;@7;)
                              end
                              local.get 14
                              i32.const 0
                              i32.store8
                              local.get 10
                              i32.const 0
                              i32.store8
                              i32.const 10
                              i32.const 11
                              local.set 32
                              local.set 30
                              br 4 (;@9;)
                            end
                            local.get 13
                            i32.const 1
                            i32.store8
                            local.get 10
                            i32.const 1
                            i32.store8
                            i32.const 10
                            i32.const 11
                            local.set 32
                            local.set 30
                            br 3 (;@9;)
                          end
                          local.get 12
                          i32.const 2
                          i32.store8
                          local.get 10
                          i32.const 2
                          i32.store8
                          i32.const 20
                          i32.const 21
                          local.set 32
                          local.set 30
                          br 2 (;@9;)
                        end
                        local.get 11
                        i32.const 3
                        i32.store8
                        local.get 10
                        i32.const 3
                        i32.store8
                        i32.const 20
                        i32.const 21
                        local.set 32
                        local.set 30
                        br 1 (;@9;)
                      end
                      local.get 9
                      i32.const 4
                      i32.store8
                      local.get 10
                      i32.const 4
                      i32.store8
                      i32.const 20
                      i32.const 21
                      local.set 32
                      local.set 30
                      br 0 (;@9;)
                    end
                    local.get 30
                    local.get 2
                    i32.lt_u
                    if ;; label = @9
                      br 3 (;@6;)
                    else
                      local.get 15
                      local.get 1
                      call $_RNvMs_CsizV4RGem7kw_4exprNtB4_6Parser4bump
                      local.get 15
                      call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueNtCsizV4RGem7kw_4expr5TokenEBD_
                      local.get 16
                      local.get 1
                      local.get 32
                      call $_RNvMs_CsizV4RGem7kw_4exprNtB4_6Parser10parse_expr
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 16
                            i32.load8_u
                            i32.const 255
                            i32.eq
                            br_table 1 (;@11;) 2 (;@10;) 0 (;@12;)
                          end
                          br 10 (;@1;)
                        end
                        local.get 16
                        i64.load
                        local.set 5
                        local.get 16
                        i64.load offset=8
                        local.set 4
                        local.get 17
                        local.get 5
                        i64.store
                        local.get 17
                        local.get 4
                        i64.store offset=8
                        local.get 18
                        local.get 17
                        i64.load
                        i64.store
                        local.get 18
                        local.get 4
                        i64.store offset=8
                        local.get 19
                        local.get 18
                        i64.load
                        i64.store
                        local.get 19
                        local.get 4
                        i64.store offset=8
                        local.get 6
                        i64.load
                        local.set 4
                        local.get 6
                        i64.load offset=8
                        local.set 5
                        local.get 20
                        local.get 4
                        i64.store
                        local.get 20
                        local.get 5
                        i64.store offset=8
                        local.get 21
                        i32.load align=1
                        local.get 21
                        i32.load offset=4 align=1
                        call $_RNvNtCs45LlgTF0LZI_5alloc5boxed14box_new_uninit
                        local.set 32
                        local.get 20
                        i64.load
                        local.set 5
                        local.get 20
                        i64.load offset=8
                        local.set 4
                        local.get 32
                        local.get 5
                        i64.store
                        local.get 32
                        local.get 4
                        i64.store offset=8
                        local.get 22
                        local.get 32
                        i32.store align=1
                        local.get 23
                        i32.load align=1
                        local.get 23
                        i32.load offset=4 align=1
                        call $_RNvNtCs45LlgTF0LZI_5alloc5boxed14box_new_uninit
                        local.set 32
                        local.get 19
                        i64.load
                        local.set 4
                        local.get 19
                        i64.load offset=8
                        local.set 5
                        local.get 32
                        local.get 4
                        i64.store
                        local.get 32
                        local.get 5
                        i64.store offset=8
                        local.get 24
                        local.get 32
                        i32.store align=1
                        local.get 25
                        local.get 10
                        i32.load8_u
                        i32.store8
                        local.get 26
                        local.get 22
                        i32.load align=1
                        i32.store align=1
                        local.get 27
                        local.get 24
                        i32.load align=1
                        i32.store align=1
                        local.get 28
                        i32.const 3
                        i32.store8
                        local.get 28
                        i64.load
                        local.set 5
                        local.get 28
                        i64.load offset=8
                        local.set 4
                        local.get 6
                        local.get 5
                        i64.store
                        local.get 6
                        local.get 4
                        i64.store offset=8
                        br 6 (;@4;)
                      end
                      local.get 0
                      i32.const 255
                      i32.store8
                      local.get 6
                      call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueNtCsizV4RGem7kw_4expr4ExprEBD_
                      br 7 (;@2;)
                    end
                  else
                    br 1 (;@7;)
                  end
                end
                br 0 (;@6;)
              end
              local.get 6
              i64.load
              local.set 4
              local.get 6
              i64.load offset=8
              local.set 5
              local.get 33
              i32.const 176
              i32.add
              local.tee 6
              local.get 4
              i64.store
              local.get 6
              local.get 5
              i64.store offset=8
              local.get 0
              local.get 6
              i64.load
              i64.store
              local.get 0
              local.get 5
              i64.store offset=8
              local.get 33
              i32.const 256
              i32.add
              global.set $__stack_pointer
              return
            else
              local.get 30
              local.get 32
              i32.const 1066416
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          end
        end
        local.get 0
        i32.const 255
        i32.store8
        br 0 (;@2;)
      end
      local.get 33
      i32.const 256
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
    unreachable
  )
```

## LLVM -O2

```wat
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
```

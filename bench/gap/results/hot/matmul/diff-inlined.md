# matmul::matmul vs LLVM's inlined `run`

LLVM -O2 inlined the whole program into `run`; this is the hot function that survives as a separate body in our module.

## Ours (355,806,232 instructions, 93.5%)

```wat
(func $_RNvCshS8UloB9tPT_6matmul6matmul (;10;) (type 7) (param i32 i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 f64 i32 i32 f64)
    i32.const 0
    local.set 7
    loop ;; label = @1
      local.get 7
      local.get 6
      i32.lt_u
      if ;; label = @2
        local.get 6
        local.get 7
        i32.mul
        local.set 8
        i32.const 0
        local.set 9
        loop ;; label = @3
          local.get 9
          local.get 6
          i32.lt_u
          if ;; label = @4
            local.get 8
            local.get 9
            i32.add
            local.tee 12
            local.get 5
            i32.lt_u
            if ;; label = @5
              local.get 4
              local.get 12
              i32.const 3
              i32.shl
              i32.add
              f64.const 0x0p+0 (;=0;)
              f64.store align=1
              local.get 9
              i32.const 1
              i32.add
              local.set 9
              br 2 (;@3;)
            else
              local.get 12
              local.get 5
              i32.const 1065460
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          else
            i32.const 0
            local.set 9
            loop ;; label = @5
              local.get 9
              local.get 6
              i32.lt_u
              if ;; label = @6
                local.get 8
                local.get 9
                i32.add
                local.tee 10
                local.get 1
                i32.lt_u
                if ;; label = @7
                  local.get 0
                  local.get 10
                  i32.const 3
                  i32.shl
                  i32.add
                  f64.load align=1
                  local.set 11
                  local.get 6
                  local.get 9
                  i32.mul
                  local.set 10
                  i32.const 0
                  local.set 12
                  loop ;; label = @8
                    local.get 12
                    local.get 6
                    i32.lt_u
                    if ;; label = @9
                      local.get 10
                      local.get 12
                      i32.add
                      local.tee 13
                      local.get 3
                      i32.lt_u
                      if ;; label = @10
                        local.get 2
                        local.get 13
                        i32.const 3
                        i32.shl
                        i32.add
                        f64.load align=1
                        local.set 14
                        local.get 8
                        local.get 12
                        i32.add
                        local.tee 13
                        local.get 5
                        i32.lt_u
                        if ;; label = @11
                          local.get 4
                          local.get 13
                          i32.const 3
                          i32.shl
                          i32.add
                          local.tee 13
                          local.get 13
                          f64.load align=1
                          local.get 11
                          local.get 14
                          f64.mul
                          f64.add
                          f64.store align=1
                          local.get 12
                          i32.const 1
                          i32.add
                          local.set 12
                          br 3 (;@8;)
                        else
                          local.get 13
                          local.get 5
                          i32.const 1065304
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        local.get 13
                        local.get 3
                        i32.const 1065428
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 9
                      i32.const 1
                      i32.add
                      local.set 9
                      br 4 (;@5;)
                    end
                  end
                else
                  local.get 10
                  local.get 1
                  i32.const 1065444
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 7
                i32.const 1
                i32.add
                local.set 7
                br 5 (;@1;)
              end
            end
          end
        end
      else
        return
      end
    end
    unreachable
  )
```

## LLVM -O2 `run` (346,913,888 instructions, 100.0%)

```wat
(func $run (;7;) (type 6) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i32 i64 f64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      i32.const 0
                      i32.load8_u offset=1065904
                      br_if 0 (;@9;)
                      block ;; label = @10
                        memory.size
                        i32.const 16
                        i32.shl
                        local.tee 2
                        br_if 0 (;@10;)
                        i32.const 65536
                        local.set 2
                        i32.const 1
                        memory.grow
                        i32.const -1
                        i32.eq
                        br_if 3 (;@7;)
                      end
                      i32.const 0
                      local.get 2
                      i32.store offset=1065900
                      i32.const 0
                      local.get 2
                      i32.store offset=1065896
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1065904
                      br 1 (;@8;)
                    end
                    i32.const 0
                    i32.load offset=1065896
                    local.tee 2
                    i32.const -8
                    i32.gt_u
                    br_if 1 (;@7;)
                  end
                  local.get 2
                  i32.const -131080
                  i32.gt_u
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 2
                    i32.const 7
                    i32.add
                    i32.const -8
                    i32.and
                    local.tee 3
                    i32.const 131072
                    i32.add
                    local.tee 2
                    i32.const 0
                    i32.load offset=1065900
                    local.tee 4
                    i32.le_u
                    br_if 0 (;@8;)
                    local.get 2
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
                    local.tee 5
                    i32.const -1
                    i32.eq
                    br_if 1 (;@7;)
                    i32.const 0
                    local.get 5
                    local.get 4
                    i32.add
                    i32.const 16
                    i32.shl
                    i32.store offset=1065900
                  end
                  i32.const 0
                  local.get 2
                  i32.store offset=1065896
                  local.get 3
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  local.get 3
                  i32.store offset=8
                  local.get 1
                  i32.const 16384
                  i32.store offset=4
                  local.get 1
                  i32.const 0
                  i32.store offset=12
                  call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                  i32.const 0
                  i32.load8_u offset=1065904
                  br_if 1 (;@6;)
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 2
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 2
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 4 (;@4;)
                  end
                  i32.const 0
                  local.get 2
                  i32.store offset=1065900
                  i32.const 0
                  local.get 2
                  i32.store offset=1065896
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1065904
                  br 2 (;@5;)
                end
                i32.const 8
                i32.const 131072
                call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
                unreachable
              end
              i32.const 0
              i32.load offset=1065896
              local.tee 2
              i32.const -8
              i32.gt_u
              br_if 1 (;@4;)
            end
            local.get 2
            i32.const -131080
            i32.gt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 2
              i32.const 7
              i32.add
              i32.const -8
              i32.and
              local.tee 2
              i32.const 131072
              i32.add
              local.tee 4
              i32.const 0
              i32.load offset=1065900
              local.tee 5
              i32.le_u
              br_if 0 (;@5;)
              local.get 4
              local.get 5
              i32.sub
              local.tee 5
              i32.const 16
              i32.shr_u
              local.get 5
              i32.const 65535
              i32.and
              i32.const 0
              i32.ne
              i32.add
              local.tee 5
              memory.grow
              local.tee 6
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              i32.const 0
              local.get 6
              local.get 5
              i32.add
              i32.const 16
              i32.shl
              i32.store offset=1065900
            end
            i32.const 0
            local.get 4
            i32.store offset=1065896
            local.get 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 2
            i32.store offset=20
            local.get 1
            i32.const 16384
            i32.store offset=16
            local.get 1
            i32.const 0
            i32.store offset=24
            call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            i32.const 0
            i32.load8_u offset=1065904
            br_if 1 (;@3;)
            block ;; label = @5
              memory.size
              i32.const 16
              i32.shl
              local.tee 2
              br_if 0 (;@5;)
              i32.const 65536
              local.set 2
              i32.const 1
              memory.grow
              i32.const -1
              i32.eq
              br_if 4 (;@1;)
            end
            i32.const 0
            local.get 2
            i32.store offset=1065900
            i32.const 0
            local.get 2
            i32.store offset=1065896
            i32.const 0
            i32.const 1
            i32.store8 offset=1065904
            br 2 (;@2;)
          end
          i32.const 8
          i32.const 131072
          call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
          unreachable
        end
        i32.const 0
        i32.load offset=1065896
        local.tee 2
        i32.const -8
        i32.gt_u
        br_if 1 (;@1;)
      end
      local.get 2
      i32.const -131080
      i32.gt_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 2
        i32.const 7
        i32.add
        i32.const -8
        i32.and
        local.tee 2
        i32.const 131072
        i32.add
        local.tee 4
        i32.const 0
        i32.load offset=1065900
        local.tee 5
        i32.le_u
        br_if 0 (;@2;)
        local.get 4
        local.get 5
        i32.sub
        local.tee 5
        i32.const 16
        i32.shr_u
        local.get 5
        i32.const 65535
        i32.and
        i32.const 0
        i32.ne
        i32.add
        local.tee 5
        memory.grow
        local.tee 6
        i32.const -1
        i32.eq
        br_if 1 (;@1;)
        i32.const 0
        local.get 6
        local.get 5
        i32.add
        i32.const 16
        i32.shl
        i32.store offset=1065900
      end
      i32.const 0
      local.get 4
      i32.store offset=1065896
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 4
      local.get 1
      i32.const 0
      i32.store offset=36
      local.get 1
      local.get 2
      i32.store offset=32
      local.get 1
      i32.const 16384
      i32.store offset=28
      local.get 0
      i64.const -7046029254386353131
      i64.xor
      i64.const -7046029254386353131
      i64.add
      local.tee 0
      i64.const 30
      i64.shr_u
      local.get 0
      i64.xor
      i64.const -4658895280553007687
      i64.mul
      local.tee 0
      i64.const 27
      i64.shr_u
      local.get 0
      i64.xor
      i64.const -7723592293110705685
      i64.mul
      local.tee 0
      i64.const 31
      i64.shr_u
      local.get 0
      i64.xor
      i64.const -2691343689449507777
      i64.add
      local.set 0
      i32.const 16384
      local.set 6
      i32.const 0
      local.set 2
      loop ;; label = @2
        local.get 0
        i64.const 7046029254386353131
        i64.add
        local.tee 7
        i64.const 30
        i64.shr_u
        local.get 7
        i64.xor
        i64.const -4658895280553007687
        i64.mul
        local.tee 7
        i64.const 27
        i64.shr_u
        local.get 7
        i64.xor
        i64.const -7723592293110705685
        i64.mul
        local.tee 7
        i64.const 31
        i64.shr_u
        local.get 7
        i64.xor
        i64.const 2000001
        i64.rem_u
        i64.const -1000000
        i64.add
        f64.convert_i64_s
        f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
        f64.mul
        local.set 8
        block ;; label = @3
          local.get 2
          local.get 6
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 4
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs7NmtmrQhIpY_6matmul
          local.get 1
          i32.load offset=8
          local.set 3
          local.get 1
          i32.load offset=4
          local.set 6
        end
        local.get 3
        local.get 4
        i32.add
        local.get 8
        f64.store
        local.get 1
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        i32.store offset=12
        local.get 0
        i64.const 30
        i64.shr_u
        local.get 0
        i64.xor
        i64.const -4658895280553007687
        i64.mul
        local.tee 7
        i64.const 27
        i64.shr_u
        local.get 7
        i64.xor
        i64.const -7723592293110705685
        i64.mul
        local.tee 7
        i64.const 31
        i64.shr_u
        local.get 7
        i64.xor
        i64.const 2000001
        i64.rem_u
        i64.const -1000000
        i64.add
        f64.convert_i64_s
        f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
        f64.mul
        local.set 8
        block ;; label = @3
          local.get 1
          i32.load offset=24
          local.tee 5
          local.get 1
          i32.load offset=16
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 16
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs7NmtmrQhIpY_6matmul
        end
        local.get 1
        i32.load offset=20
        local.tee 9
        local.get 5
        i32.const 3
        i32.shl
        i32.add
        local.get 8
        f64.store
        local.get 1
        local.get 5
        i32.const 1
        i32.add
        local.tee 10
        i32.store offset=24
        block ;; label = @3
          local.get 1
          i32.load offset=36
          local.tee 5
          local.get 1
          i32.load offset=28
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 28
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCs7NmtmrQhIpY_6matmul
        end
        local.get 1
        i32.load offset=32
        local.tee 11
        local.get 5
        i32.const 3
        i32.shl
        i32.add
        i64.const 0
        i64.store
        local.get 1
        local.get 5
        i32.const 1
        i32.add
        local.tee 5
        i32.store offset=36
        local.get 0
        i64.const 4354685564936845354
        i64.add
        local.set 0
        local.get 4
        i32.const 8
        i32.add
        local.set 4
        local.get 2
        i32.const 16384
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 12
      local.get 1
      i32.load offset=8
      local.set 13
      local.get 11
      local.set 14
      i32.const 0
      local.set 3
      i32.const 0
      local.set 15
      loop ;; label = @2
        local.get 15
        i32.const 7
        i32.shl
        local.set 16
        local.get 12
        local.set 6
        i32.const 0
        local.set 2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  loop ;; label = @8
                    local.get 3
                    local.get 2
                    i32.add
                    local.tee 4
                    local.get 5
                    i32.ge_u
                    br_if 4 (;@4;)
                    local.get 11
                    local.get 6
                    i32.add
                    local.tee 4
                    i64.const 0
                    i64.store
                    local.get 3
                    local.get 2
                    i32.add
                    local.tee 17
                    i32.const 1
                    i32.add
                    local.get 5
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 4
                    i32.const 8
                    i32.add
                    i64.const 0
                    i64.store
                    local.get 17
                    i32.const 2
                    i32.add
                    local.get 5
                    i32.ge_u
                    br_if 2 (;@6;)
                    local.get 4
                    i32.const 16
                    i32.add
                    i64.const 0
                    i64.store
                    local.get 17
                    i32.const 3
                    i32.add
                    local.get 5
                    i32.ge_u
                    br_if 3 (;@5;)
                    local.get 4
                    i32.const 24
                    i32.add
                    i64.const 0
                    i64.store
                    local.get 6
                    i32.const 32
                    i32.add
                    local.set 6
                    local.get 2
                    i32.const 4
                    i32.add
                    local.tee 2
                    i32.const 128
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 13
                  local.get 16
                  i32.const 3
                  i32.shl
                  i32.add
                  local.set 18
                  i32.const 0
                  local.set 6
                  local.get 9
                  local.set 19
                  i32.const 0
                  local.set 20
                  block ;; label = @8
                    loop ;; label = @9
                      local.get 18
                      local.get 20
                      i32.const 3
                      i32.shl
                      i32.add
                      f64.load
                      local.set 8
                      i32.const -1024
                      local.set 4
                      i32.const 0
                      local.set 2
                      block ;; label = @10
                        loop ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              local.get 6
                              local.get 2
                              i32.add
                              local.tee 17
                              local.get 10
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 3
                              local.get 2
                              i32.add
                              local.tee 17
                              local.get 5
                              i32.ge_u
                              br_if 5 (;@8;)
                              local.get 14
                              local.get 4
                              i32.add
                              local.tee 16
                              i32.const 1024
                              i32.add
                              local.tee 17
                              local.get 17
                              f64.load
                              local.get 8
                              local.get 19
                              local.get 4
                              i32.add
                              local.tee 17
                              i32.const 1024
                              i32.add
                              f64.load
                              f64.mul
                              f64.add
                              f64.store
                              local.get 6
                              local.get 2
                              i32.add
                              local.tee 21
                              i32.const 1
                              i32.add
                              local.get 10
                              i32.lt_u
                              br_if 1 (;@12;)
                              local.get 21
                              i32.const 1
                              i32.add
                              local.set 17
                            end
                            local.get 17
                            local.get 10
                            i32.const 1049208
                            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                            unreachable
                          end
                          local.get 3
                          local.get 2
                          i32.add
                          local.tee 21
                          i32.const 1
                          i32.add
                          local.get 5
                          i32.ge_u
                          br_if 1 (;@10;)
                          local.get 16
                          i32.const 1032
                          i32.add
                          local.tee 16
                          local.get 16
                          f64.load
                          local.get 8
                          local.get 17
                          i32.const 1032
                          i32.add
                          f64.load
                          f64.mul
                          f64.add
                          f64.store
                          local.get 2
                          i32.const 2
                          i32.add
                          local.set 2
                          local.get 4
                          i32.const 16
                          i32.add
                          local.tee 4
                          br_if 0 (;@11;)
                        end
                        local.get 6
                        i32.const 128
                        i32.add
                        local.set 6
                        local.get 19
                        i32.const 1024
                        i32.add
                        local.set 19
                        local.get 20
                        i32.const 1
                        i32.add
                        local.tee 20
                        i32.const 128
                        i32.eq
                        br_if 7 (;@3;)
                        br 1 (;@9;)
                      end
                    end
                    local.get 21
                    i32.const 1
                    i32.add
                    local.set 17
                  end
                  local.get 17
                  local.get 5
                  i32.const 1049224
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 17
                i32.const 1
                i32.add
                local.set 4
                br 2 (;@4;)
              end
              local.get 17
              i32.const 2
              i32.add
              local.set 4
              br 1 (;@4;)
            end
            local.get 17
            i32.const 3
            i32.add
            local.set 4
          end
          local.get 4
          local.get 5
          i32.const 1049240
          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
          unreachable
        end
        local.get 14
        i32.const 1024
        i32.add
        local.set 14
        local.get 3
        i32.const 128
        i32.add
        local.set 3
        local.get 12
        i32.const 1024
        i32.add
        local.set 12
        local.get 15
        i32.const 1
        i32.add
        local.tee 15
        i32.const 128
        i32.ne
        br_if 0 (;@2;)
      end
      i64.const 0
      local.set 0
      f64.const 0x0p+0 (;=0;)
      local.set 22
      i32.const 0
      local.set 6
      local.get 11
      local.set 17
      f64.const 0x0p+0 (;=0;)
      local.set 23
      f64.const 0x0p+0 (;=0;)
      local.set 24
      i32.const 0
      local.set 10
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            local.get 17
            local.set 4
            i32.const 0
            local.set 2
            loop ;; label = @5
              local.get 6
              local.get 2
              i32.add
              local.tee 3
              local.get 5
              i32.ge_u
              br_if 2 (;@3;)
              local.get 23
              local.get 4
              f64.load
              local.tee 8
              f64.add
              local.get 23
              local.get 10
              local.get 2
              i32.eq
              select
              local.set 23
              local.get 4
              i32.const 8
              i32.add
              local.set 4
              local.get 22
              local.get 8
              f64.add
              local.set 22
              local.get 0
              i64.const 3
              i64.rotl
              local.get 8
              i64.reinterpret_f64
              i64.xor
              local.set 0
              local.get 24
              local.get 8
              local.get 3
              f64.convert_i32_u
              f64.const 0x1.a36e2eb1c432dp-14 (;=0.0001;)
              f64.mul
              f64.mul
              f64.add
              local.set 24
              local.get 2
              i32.const 1
              i32.add
              local.tee 2
              i32.const 128
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 17
            i32.const 1024
            i32.add
            local.set 17
            local.get 6
            i32.const 128
            i32.add
            local.set 6
            local.get 10
            i32.const 1
            i32.add
            local.tee 10
            i32.const 128
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 1
          i64.const 128
          i64.store offset=40
          i32.const 0
          i32.const 16384
          i32.const 0
          i32.load offset=1065892
          local.tee 4
          i32.sub
          local.tee 2
          local.get 2
          i32.const 16384
          i32.gt_u
          select
          local.tee 2
          i32.const 8
          local.get 2
          i32.const 8
          i32.lt_u
          select
          local.tee 5
          local.get 4
          i32.add
          local.tee 2
          local.get 4
          i32.lt_u
          br_if 1 (;@2;)
          local.get 2
          i32.const 16385
          i32.ge_u
          br_if 1 (;@2;)
          block ;; label = @4
            local.get 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 5
            memory.copy
          end
          local.get 1
          local.get 22
          f64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          local.get 1
          local.get 23
          f64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 4
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          local.get 1
          local.get 24
          f64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 4
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          local.get 1
          local.get 0
          i64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 4
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          i32.const 0
          local.get 4
          local.get 2
          i32.add
          local.tee 2
          i32.store offset=1065892
          local.get 1
          local.get 11
          f64.load
          f64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          local.get 1
          local.get 11
          f64.load offset=66048
          f64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 4
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          local.get 1
          local.get 11
          f64.load offset=131064
          f64.store offset=40
          block ;; label = @4
            i32.const 16384
            local.get 4
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 4
            i32.const 8
            local.get 4
            i32.const 8
            i32.lt_u
            select
            local.tee 4
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049508
            i32.add
            local.get 1
            i32.const 40
            i32.add
            local.get 4
            memory.copy
          end
          i32.const 0
          local.get 4
          local.get 2
          i32.add
          i32.store offset=1065892
          local.get 1
          i32.const 48
          i32.add
          global.set $__stack_pointer
          local.get 22
          i64.reinterpret_f64
          i64.const 11
          i64.rotl
          local.get 23
          i64.reinterpret_f64
          i64.const 31
          i64.rotl
          i64.xor
          local.get 24
          i64.reinterpret_f64
          i64.const 47
          i64.rotl
          i64.xor
          local.get 0
          i64.xor
          return
        end
        local.get 3
        local.get 5
        i32.const 1049192
        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
        unreachable
      end
      local.get 4
      local.get 2
      i32.const 16384
      i32.const 1049176
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    i32.const 8
    i32.const 131072
    call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
    unreachable
  )
```

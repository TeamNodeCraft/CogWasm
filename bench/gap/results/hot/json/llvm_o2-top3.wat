(func $_RNvCskpYeTZolUQS_4json13collect_paths (;20;) (type 8) (param i32 i32 i32 i64)
    (local i32 i32 i32 i32 i64 i32 i64 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 0
            i32.load
            local.tee 5
            i32.const -2147483648
            i32.xor
            i32.const 5
            local.get 5
            i32.const 0
            i32.lt_s
            select
            i32.const -2
            i32.add
            br_table 1 (;@3;) 3 (;@1;) 0 (;@4;) 2 (;@2;) 3 (;@1;)
          end
          local.get 0
          i32.load offset=12
          local.tee 5
          i32.eqz
          br_if 2 (;@1;)
          local.get 4
          i32.const 19
          i32.add
          local.set 6
          local.get 1
          i32.load offset=8
          local.set 7
          local.get 5
          i64.extend_i32_u
          local.set 8
          local.get 0
          i32.load offset=8
          local.set 9
          i64.const 0
          local.set 10
          loop ;; label = @4
            block ;; label = @5
              local.get 1
              i32.load
              local.get 7
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              local.get 7
              i32.const 1
              call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
            end
            local.get 1
            i32.load offset=4
            local.get 7
            i32.add
            i32.const 91
            i32.store8
            local.get 1
            local.get 7
            i32.const 1
            i32.add
            local.tee 0
            i32.store offset=8
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 10
                  i64.const 0
                  i64.ne
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 1
                    i32.load
                    local.get 0
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 1
                    local.get 0
                    i32.const 1
                    call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
                  end
                  local.get 1
                  i32.load offset=4
                  local.get 0
                  i32.add
                  i32.const 48
                  i32.store8
                  local.get 7
                  i32.const 2
                  i32.add
                  local.set 0
                  br 1 (;@6;)
                end
                i32.const 0
                local.set 0
                local.get 4
                i32.const 0
                i32.store offset=16
                local.get 4
                i64.const 0
                i64.store offset=8
                local.get 4
                i64.const 0
                i64.store
                local.get 10
                local.set 11
                block ;; label = @7
                  loop ;; label = @8
                    local.get 6
                    local.get 0
                    i32.add
                    local.tee 5
                    local.get 11
                    local.get 11
                    i64.const 10
                    i64.div_u
                    local.tee 12
                    i64.const 10
                    i64.mul
                    i64.sub
                    i32.wrap_i64
                    i32.const 48
                    i32.or
                    i32.store8
                    local.get 11
                    i64.const 10
                    i64.lt_u
                    br_if 1 (;@7;)
                    local.get 0
                    i32.const 18
                    i32.add
                    local.set 5
                    local.get 0
                    i32.const -1
                    i32.add
                    local.set 0
                    local.get 12
                    local.set 11
                    local.get 5
                    i32.const 20
                    i32.lt_u
                    br_if 0 (;@8;)
                  end
                  i32.const -1
                  i32.const 20
                  i32.const 1050176
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 4
                i32.const 20
                i32.add
                local.get 5
                i32.const 1
                local.get 0
                i32.sub
                call $_RNvNtNtCs8euEcNjxCEf_4core3str8converts9from_utf8
                block ;; label = @7
                  local.get 4
                  i32.load offset=20
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 1
                  i32.load offset=8
                  local.set 0
                  br 2 (;@5;)
                end
                local.get 4
                i32.load offset=24
                local.set 13
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 4
                      i32.load offset=28
                      local.tee 0
                      local.get 1
                      i32.load
                      local.get 1
                      i32.load offset=8
                      local.tee 5
                      i32.sub
                      i32.le_u
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 5
                      local.get 0
                      call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
                      local.get 1
                      i32.load offset=8
                      local.set 5
                      br 1 (;@8;)
                    end
                    local.get 0
                    i32.eqz
                    br_if 1 (;@7;)
                  end
                  block ;; label = @8
                    local.get 0
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load offset=4
                    local.get 5
                    i32.add
                    local.get 13
                    local.get 0
                    memory.copy
                  end
                  local.get 1
                  i32.load offset=8
                  local.set 5
                end
                local.get 5
                local.get 0
                i32.add
                local.set 0
              end
              local.get 1
              local.get 0
              i32.store offset=8
            end
            block ;; label = @5
              local.get 1
              i32.load
              local.get 0
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              local.get 0
              i32.const 1
              call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
            end
            local.get 1
            i32.load offset=4
            local.get 0
            i32.add
            i32.const 93
            i32.store8
            local.get 1
            local.get 0
            i32.const 1
            i32.add
            i32.store offset=8
            local.get 9
            local.get 10
            i32.wrap_i64
            i32.const 4
            i32.shl
            i32.add
            local.get 1
            local.get 2
            local.get 3
            call $_RNvCskpYeTZolUQS_4json13collect_paths
            block ;; label = @5
              block ;; label = @6
                local.get 7
                local.get 1
                i32.load offset=8
                local.tee 0
                i32.le_u
                br_if 0 (;@6;)
                local.get 0
                local.set 7
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                local.get 0
                i32.ge_u
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=4
                local.get 7
                i32.add
                i32.load8_s
                i32.const -65
                i32.gt_s
                br_if 0 (;@6;)
                i32.const 1050208
                i32.const 48
                i32.const 1049564
                call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
                unreachable
              end
              local.get 1
              local.get 7
              i32.store offset=8
            end
            local.get 10
            i64.const 1
            i64.add
            local.tee 10
            local.get 8
            i64.ne
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 0
        i64.load offset=8
        local.tee 11
        local.get 3
        i64.lt_s
        br_if 1 (;@1;)
        local.get 4
        local.get 1
        call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
        block ;; label = @3
          local.get 2
          i32.load offset=8
          local.tee 0
          local.get 2
          i32.load
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecTNtNtB7_6string6StringxEE8grow_oneCskpYeTZolUQS_4json
        end
        local.get 2
        i32.load offset=4
        local.get 0
        i32.const 24
        i32.mul
        i32.add
        local.tee 5
        local.get 4
        i64.load offset=8
        i64.store offset=8
        local.get 5
        local.get 4
        i64.load
        i64.store
        local.get 5
        local.get 11
        i64.store offset=16
        local.get 2
        local.get 0
        i32.const 1
        i32.add
        i32.store offset=8
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=8
      local.tee 5
      i32.eqz
      br_if 0 (;@1;)
      local.get 5
      i32.const 5
      i32.shl
      local.set 9
      local.get 0
      i32.load offset=4
      i32.const 8
      i32.add
      local.set 5
      local.get 1
      i32.load offset=8
      local.set 0
      loop ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load
          local.get 0
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          local.get 0
          i32.const 1
          call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
        end
        local.get 1
        i32.load offset=4
        local.get 0
        i32.add
        i32.const 46
        i32.store8
        local.get 1
        local.get 0
        i32.const 1
        i32.add
        local.tee 6
        i32.store offset=8
        local.get 5
        i32.const -4
        i32.add
        i32.load
        local.set 13
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 5
              i32.load
              local.tee 7
              local.get 1
              i32.load
              local.get 6
              i32.sub
              i32.le_u
              br_if 0 (;@5;)
              local.get 1
              local.get 6
              local.get 7
              call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECskpYeTZolUQS_4json.llvm.12285963892569529774
              local.get 1
              i32.load offset=8
              local.set 6
              br 1 (;@4;)
            end
            local.get 7
            i32.eqz
            br_if 1 (;@3;)
          end
          block ;; label = @4
            local.get 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=4
            local.get 6
            i32.add
            local.get 13
            local.get 7
            memory.copy
          end
          local.get 1
          i32.load offset=8
          local.set 6
        end
        local.get 1
        local.get 6
        local.get 7
        i32.add
        i32.store offset=8
        local.get 5
        i32.const 8
        i32.add
        local.get 1
        local.get 2
        local.get 3
        call $_RNvCskpYeTZolUQS_4json13collect_paths
        block ;; label = @3
          block ;; label = @4
            local.get 0
            local.get 1
            i32.load offset=8
            local.tee 6
            i32.le_u
            br_if 0 (;@4;)
            local.get 6
            local.set 0
            br 1 (;@3;)
          end
          block ;; label = @4
            local.get 0
            i32.eqz
            br_if 0 (;@4;)
            local.get 0
            local.get 6
            i32.ge_u
            br_if 0 (;@4;)
            local.get 1
            i32.load offset=4
            local.get 0
            i32.add
            i32.load8_s
            i32.const -65
            i32.gt_s
            br_if 0 (;@4;)
            i32.const 1050208
            i32.const 48
            i32.const 1049580
            call $_RNvNtCs8euEcNjxCEf_4core9panicking5panic
            unreachable
          end
          local.get 1
          local.get 0
          i32.store offset=8
        end
        local.get 5
        i32.const 32
        i32.add
        local.set 5
        local.get 9
        i32.const -32
        i32.add
        local.tee 9
        br_if 0 (;@2;)
      end
    end
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )

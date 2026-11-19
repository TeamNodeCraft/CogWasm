(func $_RNvNvNtCseFzb18zECBI_5alloc3fmt6format12format_inner (;20;) (type 4) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 1
                  i32.shr_u
                  local.set 4
                  br 1 (;@6;)
                end
                local.get 1
                i32.load8_u
                local.tee 4
                i32.eqz
                br_if 1 (;@5;)
                i32.const 0
                local.set 5
                local.get 1
                local.set 6
                i32.const 0
                local.set 7
                loop ;; label = @7
                  local.get 6
                  i32.const 1
                  i32.add
                  local.set 6
                  block ;; label = @8
                    block ;; label = @9
                      local.get 4
                      i32.extend8_s
                      i32.const -1
                      i32.gt_s
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 4
                        i32.const 255
                        i32.and
                        i32.const 128
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 5
                        local.get 6
                        i32.load16_u align=1
                        local.tee 4
                        i32.add
                        local.set 5
                        local.get 6
                        local.get 4
                        i32.add
                        i32.const 2
                        i32.add
                        local.set 6
                        br 2 (;@8;)
                      end
                      local.get 6
                      local.get 4
                      i32.const 3
                      i32.and
                      i32.const 8
                      i32.rotr
                      local.tee 8
                      i32.const 5
                      i32.shl
                      i32.const 1073741824
                      i32.and
                      local.get 8
                      i32.const 7
                      i32.shl
                      i32.or
                      i32.const 29
                      i32.shr_u
                      i32.add
                      local.get 4
                      i32.const 1
                      i32.shr_u
                      i32.const 2
                      i32.and
                      i32.add
                      local.get 4
                      i32.const 2
                      i32.shr_u
                      i32.const 2
                      i32.and
                      i32.add
                      local.set 6
                      local.get 5
                      i32.eqz
                      local.get 7
                      i32.or
                      local.set 7
                      br 1 (;@8;)
                    end
                    local.get 6
                    local.get 4
                    i32.const 255
                    i32.and
                    local.tee 4
                    i32.add
                    local.set 6
                    local.get 5
                    local.get 4
                    i32.add
                    local.set 5
                  end
                  local.get 6
                  i32.load8_u
                  local.tee 4
                  br_if 0 (;@7;)
                end
                i32.const 0
                local.set 4
                local.get 7
                local.get 5
                i32.const 16
                i32.lt_u
                i32.and
                br_if 0 (;@6;)
                i32.const 0
                local.set 7
                local.get 5
                i32.const 1
                i32.shl
                local.tee 4
                i32.const 0
                i32.lt_s
                br_if 4 (;@2;)
              end
              local.get 4
              br_if 1 (;@4;)
            end
            i32.const 1
            local.set 6
            i32.const 0
            local.set 4
            br 1 (;@3;)
          end
          call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
          i32.const 1
          local.set 7
          local.get 4
          i32.const 1
          call $_RNvCs7c4fZWoqHJ1_7___rustc12___rust_alloc
          local.tee 6
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 3
        i32.const 0
        i32.store offset=12
        local.get 3
        local.get 6
        i32.store offset=8
        local.get 3
        local.get 4
        i32.store offset=4
        local.get 3
        i32.const 4
        i32.add
        i32.const 1049216
        local.get 1
        local.get 2
        call $_RNvNtCs8euEcNjxCEf_4core3fmt5write
        i32.eqz
        br_if 1 (;@1;)
        call $_RINvNtCs8euEcNjxCEf_4core6result13unwrap_failedNtNtB4_3fmt5ErrorECseFzb18zECBI_5alloc
        unreachable
      end
      local.get 7
      local.get 4
      call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 0
    local.get 3
    i32.load offset=12
    i32.store offset=8
    local.get 0
    local.get 3
    i64.load offset=4 align=4
    i64.store align=4
    local.get 3
    i32.const 16
    i32.add
    global.set $__stack_pointer
  )

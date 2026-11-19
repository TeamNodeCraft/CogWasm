(func $_RNvMs8_NtCseFzb18zECBI_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs9hsE6Fh6sWE_7strings (;1;) (type 3) (param i32 i32 i32 i32 i32)
    (local i32 i32 i64 i32 i32 i32 i32)
    i32.const 1
    local.set 5
    i32.const 4
    local.set 6
    block ;; label = @1
      block ;; label = @2
        local.get 4
        i64.extend_i32_u
        local.get 2
        i64.extend_i32_u
        i64.mul
        local.tee 7
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        br_if 0 (;@2;)
        local.get 7
        i32.wrap_i64
        local.tee 2
        i32.const -2147483648
        local.get 3
        i32.sub
        i32.gt_u
        br_if 0 (;@2;)
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 1
                  i32.load
                  local.tee 8
                  i32.eqz
                  br_if 0 (;@7;)
                  local.get 1
                  i32.load offset=4
                  local.set 9
                  block ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1066376
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.load offset=1066368
                    local.set 6
                    br 2 (;@6;)
                  end
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 6
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 6
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 4 (;@4;)
                  end
                  i32.const 0
                  local.get 6
                  i32.store offset=1066372
                  i32.const 0
                  local.get 6
                  i32.store offset=1066368
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1066376
                  br 1 (;@6;)
                end
                block ;; label = @7
                  local.get 2
                  br_if 0 (;@7;)
                  local.get 3
                  local.set 6
                  br 4 (;@3;)
                end
                call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                block ;; label = @7
                  block ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1066376
                    i32.eqz
                    br_if 0 (;@8;)
                    i32.const 0
                    i32.load offset=1066368
                    local.set 6
                    br 1 (;@7;)
                  end
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 6
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 6
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 4 (;@4;)
                  end
                  i32.const 0
                  local.get 6
                  i32.store offset=1066372
                  i32.const 0
                  local.get 6
                  i32.store offset=1066368
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1066376
                end
                local.get 3
                i32.const -1
                i32.add
                local.tee 4
                local.get 6
                i32.add
                local.tee 6
                local.get 4
                i32.lt_u
                br_if 2 (;@4;)
                local.get 6
                i32.const 0
                local.get 3
                i32.sub
                i32.and
                local.tee 6
                local.get 2
                i32.add
                local.tee 4
                local.get 6
                i32.lt_u
                br_if 2 (;@4;)
                block ;; label = @7
                  local.get 4
                  i32.const 0
                  i32.load offset=1066372
                  local.tee 1
                  i32.le_u
                  br_if 0 (;@7;)
                  local.get 4
                  local.get 1
                  i32.sub
                  local.tee 1
                  i32.const 16
                  i32.shr_u
                  local.get 1
                  i32.const 65535
                  i32.and
                  i32.const 0
                  i32.ne
                  i32.add
                  local.tee 1
                  memory.grow
                  local.tee 8
                  i32.const -1
                  i32.eq
                  br_if 3 (;@4;)
                  i32.const 0
                  local.get 8
                  local.get 1
                  i32.add
                  i32.const 16
                  i32.shl
                  i32.store offset=1066372
                end
                i32.const 0
                local.get 4
                i32.store offset=1066368
                br 1 (;@5;)
              end
              local.get 3
              i32.const -1
              i32.add
              local.tee 1
              local.get 6
              i32.add
              local.tee 6
              local.get 1
              i32.lt_u
              br_if 1 (;@4;)
              local.get 6
              i32.const 0
              local.get 3
              i32.sub
              i32.and
              local.tee 6
              local.get 2
              i32.add
              local.tee 1
              local.get 6
              i32.lt_u
              br_if 1 (;@4;)
              block ;; label = @6
                local.get 1
                i32.const 0
                i32.load offset=1066372
                local.tee 10
                i32.le_u
                br_if 0 (;@6;)
                local.get 1
                local.get 10
                i32.sub
                local.tee 10
                i32.const 16
                i32.shr_u
                local.get 10
                i32.const 65535
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 10
                memory.grow
                local.tee 11
                i32.const -1
                i32.eq
                br_if 2 (;@4;)
                i32.const 0
                local.get 11
                local.get 10
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1066372
              end
              i32.const 0
              local.get 1
              i32.store offset=1066368
              local.get 6
              i32.eqz
              br_if 1 (;@4;)
              local.get 8
              local.get 4
              i32.mul
              local.tee 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 6
              local.get 9
              local.get 4
              memory.copy
            end
            local.get 6
            br_if 1 (;@3;)
          end
          local.get 0
          local.get 3
          i32.store offset=4
          i32.const 8
          local.set 6
          br 2 (;@1;)
        end
        local.get 0
        local.get 6
        i32.store offset=4
        i32.const 0
        local.set 5
        i32.const 8
        local.set 6
        br 1 (;@1;)
      end
      i32.const 0
      local.set 2
    end
    local.get 0
    local.get 6
    i32.add
    local.get 2
    i32.store
    local.get 0
    local.get 5
    i32.store
  )

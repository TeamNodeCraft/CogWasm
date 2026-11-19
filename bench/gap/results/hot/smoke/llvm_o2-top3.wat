(func $_RNvMs8_NtCseFzb18zECBI_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs1KmEyLwV42l_5smoke (;7;) (type 6) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    i32.const 1
    local.set 3
    i32.const 4
    local.set 4
    block ;; label = @1
      block ;; label = @2
        local.get 2
        i32.const 536870911
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block ;; label = @2
        local.get 2
        i32.const 3
        i32.shl
        local.tee 2
        i32.const 2147483640
        i32.le_u
        br_if 0 (;@2;)
        i32.const 0
        local.set 2
        br 1 (;@1;)
      end
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 1
                      i32.load
                      local.tee 5
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      i32.load offset=4
                      local.set 6
                      i32.const 0
                      i32.load8_u offset=1065872
                      br_if 1 (;@8;)
                      block ;; label = @10
                        memory.size
                        i32.const 16
                        i32.shl
                        local.tee 4
                        br_if 0 (;@10;)
                        i32.const 65536
                        local.set 4
                        i32.const 1
                        memory.grow
                        i32.const -1
                        i32.eq
                        br_if 7 (;@3;)
                      end
                      i32.const 0
                      local.get 4
                      i32.store offset=1065868
                      i32.const 0
                      local.get 4
                      i32.store offset=1065864
                      i32.const 0
                      i32.const 1
                      i32.store8 offset=1065872
                      br 4 (;@5;)
                    end
                    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                    i32.const 0
                    i32.load8_u offset=1065872
                    br_if 1 (;@7;)
                    block ;; label = @9
                      memory.size
                      i32.const 16
                      i32.shl
                      local.tee 4
                      br_if 0 (;@9;)
                      i32.const 65536
                      local.set 4
                      i32.const 1
                      memory.grow
                      i32.const -1
                      i32.eq
                      br_if 6 (;@3;)
                    end
                    i32.const 0
                    local.get 4
                    i32.store offset=1065868
                    i32.const 0
                    local.get 4
                    i32.store offset=1065864
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1065872
                    br 2 (;@6;)
                  end
                  i32.const 0
                  i32.load offset=1065864
                  local.tee 4
                  i32.const -8
                  i32.le_u
                  br_if 2 (;@5;)
                  br 4 (;@3;)
                end
                i32.const 0
                i32.load offset=1065864
                local.tee 4
                i32.const -8
                i32.gt_u
                br_if 3 (;@3;)
              end
              local.get 4
              i32.const 7
              i32.add
              i32.const -8
              i32.and
              local.tee 4
              local.get 2
              i32.add
              local.tee 1
              local.get 4
              i32.lt_u
              br_if 2 (;@3;)
              block ;; label = @6
                local.get 1
                i32.const 0
                i32.load offset=1065868
                local.tee 5
                i32.le_u
                br_if 0 (;@6;)
                local.get 1
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
                local.tee 7
                i32.const -1
                i32.eq
                br_if 3 (;@3;)
                i32.const 0
                local.get 7
                local.get 5
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1065868
              end
              i32.const 0
              local.get 1
              i32.store offset=1065864
              br 1 (;@4;)
            end
            local.get 4
            i32.const 7
            i32.add
            i32.const -8
            i32.and
            local.tee 4
            local.get 2
            i32.add
            local.tee 1
            local.get 4
            i32.lt_u
            br_if 1 (;@3;)
            block ;; label = @5
              local.get 1
              i32.const 0
              i32.load offset=1065868
              local.tee 7
              i32.le_u
              br_if 0 (;@5;)
              local.get 1
              local.get 7
              i32.sub
              local.tee 7
              i32.const 16
              i32.shr_u
              local.get 7
              i32.const 65535
              i32.and
              i32.const 0
              i32.ne
              i32.add
              local.tee 7
              memory.grow
              local.tee 8
              i32.const -1
              i32.eq
              br_if 2 (;@3;)
              i32.const 0
              local.get 8
              local.get 7
              i32.add
              i32.const 16
              i32.shl
              i32.store offset=1065868
            end
            i32.const 0
            local.get 1
            i32.store offset=1065864
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            i32.const 3
            i32.shl
            local.tee 1
            i32.eqz
            br_if 0 (;@4;)
            local.get 4
            local.get 6
            local.get 1
            memory.copy
          end
          local.get 4
          br_if 1 (;@2;)
        end
        i32.const 8
        local.set 4
        local.get 0
        i32.const 8
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 0
      local.get 4
      i32.store offset=4
      i32.const 0
      local.set 3
      i32.const 8
      local.set 4
    end
    local.get 0
    local.get 4
    i32.add
    local.get 2
    i32.store
    local.get 0
    local.get 3
    i32.store
  )

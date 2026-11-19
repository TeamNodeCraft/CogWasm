(func $_RNvMsi_NtNtNtCseFzb18zECBI_5alloc11collections5btree3mapINtB5_8BTreeMapyNtNtB7_7set_val9SetValZSTE6insertCsbgrmSDC215G_11collections (;9;) (type 8) (param i32 i64) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 2
    global.set $__stack_pointer
    i32.const 0
    local.set 3
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.load
          local.tee 4
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          br 1 (;@2;)
        end
        local.get 0
        i32.load offset=4
        local.set 5
        block ;; label = @3
          loop ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 4
                i32.load16_u offset=94
                local.tee 6
                br_if 0 (;@6;)
                local.get 6
                local.set 7
                br 1 (;@5;)
              end
              local.get 6
              i32.const 3
              i32.shl
              local.set 8
              i32.const 0
              local.set 7
              local.get 4
              local.set 9
              block ;; label = @6
                loop ;; label = @7
                  local.get 1
                  local.get 9
                  i64.load
                  local.tee 10
                  i64.gt_u
                  local.get 1
                  local.get 10
                  i64.lt_u
                  i32.sub
                  i32.const 255
                  i32.and
                  local.tee 11
                  i32.const 1
                  i32.ne
                  br_if 1 (;@6;)
                  local.get 7
                  i32.const 1
                  i32.add
                  local.set 7
                  local.get 9
                  i32.const 8
                  i32.add
                  local.set 9
                  local.get 8
                  i32.const -8
                  i32.add
                  local.tee 8
                  br_if 0 (;@7;)
                end
                local.get 6
                local.set 7
                br 1 (;@5;)
              end
              local.get 11
              i32.eqz
              br_if 2 (;@3;)
            end
            local.get 5
            i32.eqz
            br_if 2 (;@2;)
            local.get 5
            i32.const -1
            i32.add
            local.set 5
            local.get 4
            local.get 7
            i32.const 2
            i32.shl
            i32.add
            i32.load offset=96
            local.set 4
            br 0 (;@4;)
          end
        end
        i32.const 1
        local.set 3
        br 1 (;@1;)
      end
      local.get 2
      local.get 0
      i32.store offset=28
      local.get 2
      local.get 7
      i32.store offset=24
      local.get 2
      i32.const 0
      i32.store offset=20
      local.get 2
      local.get 4
      i32.store offset=16
      local.get 2
      local.get 1
      i64.store offset=8
      local.get 2
      i32.const 8
      i32.add
      call $_RNvMs4_NtNtNtNtCseFzb18zECBI_5alloc11collections5btree3map5entryINtB5_11VacantEntryyNtNtB9_7set_val9SetValZSTE6insertCsbgrmSDC215G_11collections
      drop
    end
    local.get 2
    i32.const 32
    i32.add
    global.set $__stack_pointer
    local.get 3
  )

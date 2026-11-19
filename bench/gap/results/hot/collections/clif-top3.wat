(func $_RINvMs0_NtNtNtCs45LlgTF0LZI_5alloc11collections5btree6searchINtNtB8_4node7NodeRefNtNtBZ_6marker3MutyNtNtB8_7set_val9SetValZSTNtB1j_14LeafOrInternalE14find_key_indexyECsaeq2Ur5fVa2_11collections (;233;) (type 4) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 16
    global.set $__stack_pointer
    local.get 1
    i32.load align=1
    local.set 1
    local.get 16
    i32.const 20
    i32.add
    local.get 1
    i32.store align=1
    local.get 16
    i32.const 32
    i32.add
    local.tee 4
    local.get 1
    i32.store align=1
    local.get 16
    i32.const 24
    i32.add
    local.tee 5
    local.get 1
    i32.store align=1
    local.get 16
    i32.const 28
    i32.add
    i32.const 11
    i32.store align=1
    local.get 4
    i32.load align=1
    i32.load16_u offset=94 align=1
    local.set 4
    local.get 5
    i32.load align=1
    local.set 5
    local.get 16
    i32.const 36
    i32.add
    local.tee 1
    local.get 5
    i32.store align=1
    local.get 4
    local.set 4
    local.get 16
    i32.const 40
    i32.add
    local.get 4
    i32.store align=1
    local.get 1
    i32.load align=1
    local.set 1
    local.get 16
    i32.const 44
    i32.add
    local.tee 5
    local.get 1
    local.get 3
    i32.const 3
    i32.shl
    i32.add
    i32.store align=1
    local.get 4
    local.get 3
    i32.sub
    local.set 1
    local.get 16
    i32.const 48
    i32.add
    local.get 1
    i32.store align=1
    local.get 16
    i32.const 52
    i32.add
    local.tee 6
    local.get 5
    i32.load align=1
    i32.store align=1
    local.get 16
    i32.const 56
    i32.add
    local.get 1
    i32.store align=1
    local.get 6
    i32.load align=1
    local.set 6
    local.get 16
    i32.const 72
    i32.add
    local.tee 5
    local.get 6
    i32.store align=1
    local.get 16
    i32.const 64
    i32.add
    local.set 7
    local.get 6
    local.get 1
    i32.const 3
    i32.shl
    i32.add
    local.set 6
    local.get 16
    i32.const 68
    i32.add
    local.set 1
    local.get 16
    i32.const 76
    i32.add
    local.set 8
    local.get 16
    i32.const 60
    i32.add
    local.set 9
    local.get 16
    i32.const 8
    i32.add
    local.set 10
    local.get 16
    i32.const 12
    i32.add
    local.set 11
    local.get 16
    i32.const 16
    i32.add
    local.set 12
    i32.const 0
    local.set 13
    loop ;; label = @1
      local.get 7
      local.get 5
      i32.load align=1
      i32.store align=1
      local.get 1
      local.get 6
      i32.store align=1
      local.get 7
      i32.load align=1
      local.tee 14
      local.get 6
      i32.eq
      if ;; label = @2
        local.get 16
        i32.const 4
        i32.add
        local.tee 3
        local.get 4
        i32.store align=1
        local.get 16
        i32.const 1
        i32.store align=1
        local.get 3
        i32.load align=1
        local.set 3
        local.get 0
        i32.const 1
        i32.store
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 16
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      else
        local.get 5
        local.get 14
        i32.const 8
        i32.add
        i32.store align=1
        local.get 7
        i32.load align=1
        local.set 14
        local.get 8
        local.get 14
        i32.store align=1
        local.get 9
        local.get 14
        i32.store align=1
        local.get 10
        local.get 13
        i32.store align=1
        local.get 11
        local.get 14
        i32.store align=1
        local.get 10
        i32.load align=1
        local.set 15
        local.get 2
        local.get 14
        call $"#func316 _RNvXs13_NtNtCsk7Mwp4gGzxp_4core3cmp5implsyNtB8_3Ord3cmp"
        i32.const 255
        i32.and
        local.set 14
        local.get 12
        local.get 14
        i32.store8
        block ;; label = @3
          local.get 14
          i32.const 255
          i32.eq
          if ;; label = @4
            local.get 16
            i32.const 4
            i32.add
            local.get 3
            local.get 15
            i32.add
            i32.store align=1
            local.get 16
            i32.const 1
            i32.store align=1
            br 1 (;@3;)
          else
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 14
                  br_table 1 (;@6;) 2 (;@5;) 0 (;@7;)
                end
                unreachable
              end
              local.get 16
              i32.const 4
              i32.add
              local.get 3
              local.get 15
              i32.add
              i32.store align=1
              local.get 16
              i32.const 0
              i32.store align=1
              br 2 (;@3;)
            end
            local.get 13
            i32.const 1
            i32.add
            local.set 13
            br 3 (;@1;)
          end
        end
        local.get 16
        i32.load align=1
        local.set 15
        local.get 16
        i32.const 4
        i32.add
        i32.load align=1
        local.set 3
        local.get 0
        local.get 15
        i32.store
        local.get 0
        local.get 3
        i32.store offset=4
        local.get 16
        i32.const 80
        i32.add
        global.set $__stack_pointer
        return
      end
    end
    unreachable
  )

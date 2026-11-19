(func $_RNvXs3_NtNtCsk7Mwp4gGzxp_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next (;47;) (type 3) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 8
    global.set $__stack_pointer
    local.get 8
    i32.const 8
    i32.add
    local.tee 2
    local.get 1
    i32.store align=1
    local.get 8
    i32.const 16
    i32.add
    local.tee 3
    local.get 1
    i32.load offset=4 align=1
    i32.store align=1
    local.get 2
    i32.load align=1
    i32.load align=1
    local.set 1
    local.get 8
    i32.const 20
    i32.add
    local.get 1
    i32.store align=1
    local.get 3
    i32.load align=1
    local.set 3
    local.get 8
    i32.const 48
    i32.add
    local.tee 4
    local.get 2
    i32.load align=1
    call $_RINvNtNtCsk7Mwp4gGzxp_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECshJOoaGw8lTO_7strings
    local.get 4
    i32.load
    local.set 5
    local.get 4
    i32.load offset=4
    local.set 4
    local.get 8
    i32.const 24
    i32.add
    local.tee 6
    local.get 5
    i32.store align=1
    local.get 8
    i32.const 28
    i32.add
    local.get 4
    i32.store align=1
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 6
          i32.load align=1
          br_table 1 (;@2;) 2 (;@1;) 0 (;@3;)
        end
        unreachable
      end
      i32.const 1067140
      local.tee 4
      i32.load align=1
      local.set 7
      local.get 4
      i32.load offset=4 align=1
      local.set 4
      local.get 8
      local.tee 5
      local.get 7
      i32.store align=1
      local.get 8
      i32.const 4
      i32.add
      local.get 4
      i32.store align=1
      local.get 0
      local.get 5
      i32.load align=1
      i32.store
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 8
      i32.const 64
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 8
    i32.const 12
    i32.add
    local.get 4
    i32.store align=1
    local.get 2
    i32.load align=1
    local.tee 6
    i32.load offset=8 align=1
    local.set 5
    local.get 8
    i32.const 32
    i32.add
    local.tee 7
    local.get 6
    i32.load offset=4 align=1
    i32.store align=1
    local.get 2
    i32.load align=1
    i32.load align=1
    local.set 6
    local.get 8
    i32.const 36
    i32.add
    local.get 6
    i32.store align=1
    local.get 7
    i32.load align=1
    local.set 7
    local.get 2
    i32.load align=1
    local.tee 2
    local.get 2
    i32.load offset=8 align=1
    local.get 3
    local.get 1
    i32.sub
    local.get 7
    local.get 6
    i32.sub
    i32.sub
    i32.add
    i32.store offset=8 align=1
    local.get 8
    local.tee 7
    local.get 5
    i32.store align=1
    local.get 8
    i32.const 4
    i32.add
    local.get 4
    i32.store align=1
    local.get 0
    local.get 7
    i32.load align=1
    i32.store
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 8
    i32.const 64
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

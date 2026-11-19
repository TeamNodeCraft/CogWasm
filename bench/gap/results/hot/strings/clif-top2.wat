(func $_RNvXs8_NtNtCsk7Mwp4gGzxp_4core3str7patternINtB5_19MultiCharEqSearcherNtB7_12IsWhitespaceENtB5_8Searcher4nextCshJOoaGw8lTO_7strings (;51;) (type 3) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 8
    global.set $__stack_pointer
    local.get 8
    i32.const 0
    i32.add
    local.tee 2
    local.get 1
    i32.store align=1
    local.get 1
    i32.const 8
    i32.add
    local.set 3
    local.get 8
    i32.const 12
    i32.add
    local.tee 4
    local.get 3
    i32.load offset=4 align=1
    i32.store align=1
    local.get 3
    i32.load align=1
    local.set 3
    local.get 8
    i32.const 16
    i32.add
    local.get 3
    i32.store align=1
    local.get 4
    i32.load align=1
    local.set 4
    local.get 8
    i32.const 32
    i32.add
    local.tee 5
    local.get 1
    i32.const 8
    i32.add
    call $_RNvXs3_NtNtCsk7Mwp4gGzxp_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next
    local.get 5
    i32.load
    local.set 6
    local.get 5
    i32.load offset=4
    local.set 5
    local.get 8
    i32.const 4
    i32.add
    local.tee 7
    local.get 6
    i32.store align=1
    local.get 8
    i32.const 8
    i32.add
    local.get 5
    i32.store align=1
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 5
          i32.const -1
          i32.ne
          br_table 1 (;@2;) 2 (;@1;) 0 (;@3;)
        end
        unreachable
      end
      local.get 0
      i32.const 2
      i32.store align=1
      local.get 8
      i32.const 48
      i32.add
      global.set $__stack_pointer
      return
    end
    local.get 7
    i32.load align=1
    local.set 7
    local.get 1
    i32.const 8
    i32.add
    local.set 1
    local.get 8
    i32.const 20
    i32.add
    local.tee 6
    local.get 1
    i32.load offset=4 align=1
    i32.store align=1
    local.get 1
    i32.load align=1
    local.set 1
    local.get 8
    i32.const 24
    i32.add
    local.get 1
    i32.store align=1
    local.get 6
    i32.load align=1
    local.set 6
    local.get 2
    i32.load align=1
    i32.const 20
    i32.add
    local.get 5
    call $_RNvXs3_NtNtCsk7Mwp4gGzxp_4core3str7patternNtB7_12IsWhitespaceNtB5_11MultiCharEq7matchesCshJOoaGw8lTO_7strings
    i32.const 255
    i32.and
    local.set 5
    block ;; label = @1
      local.get 5
      if ;; label = @2
        local.get 0
        local.get 7
        i32.store offset=4 align=1
        local.get 0
        local.get 7
        local.get 4
        local.get 3
        i32.sub
        local.get 6
        local.get 1
        i32.sub
        i32.sub
        i32.add
        i32.store offset=8 align=1
        local.get 0
        i32.const 0
        i32.store align=1
        br 1 (;@1;)
      else
        local.get 0
        local.get 7
        i32.store offset=4 align=1
        local.get 0
        local.get 7
        local.get 4
        local.get 3
        i32.sub
        local.get 6
        local.get 1
        i32.sub
        i32.sub
        i32.add
        i32.store offset=8 align=1
        local.get 0
        i32.const 1
        i32.store align=1
        br 1 (;@1;)
      end
    end
    local.get 8
    i32.const 48
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

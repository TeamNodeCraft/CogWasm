(func $_RNvXsp_NtCs45LlgTF0LZI_5alloc3vecINtB5_3VecAdj3_ENtNtNtCsk7Mwp4gGzxp_4core3ops4drop4Drop4dropCs4GhUpX7rSt1_5nbody (;28;) (type 1) (param i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 3
    global.set $__stack_pointer
    local.get 0
    i32.load offset=4 align=1
    local.set 1
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    i32.store align=1
    local.get 0
    i32.load offset=8 align=1
    local.set 0
    local.get 3
    local.tee 2
    local.get 1
    i32.store align=1
    local.get 3
    i32.const 4
    i32.add
    local.get 0
    i32.store align=1
    local.get 3
    i32.const 12
    i32.add
    local.get 2
    i32.load align=1
    i32.store align=1
    local.get 3
    i32.const 16
    i32.add
    local.get 0
    i32.store align=1
    i32.const 0
    local.set 2
    loop ;; label = @1
      local.get 0
      local.get 2
      i32.eq
      if ;; label = @2
        local.get 3
        i32.const 32
        i32.add
        global.set $__stack_pointer
        return
      else
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        br 1 (;@1;)
      end
    end
    unreachable
  )

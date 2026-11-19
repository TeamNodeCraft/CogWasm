(func $_RNvCs6DQIhS9wjfV_9float_sum5kahan (;6;) (type 6) (param i32 i32) (result f64)
    (local i32 i32 i32 i32 i32 f64 f64 i32 i32 f64 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 12
    global.set $__stack_pointer
    local.get 12
    local.tee 2
    local.get 0
    i32.store align=1
    local.get 12
    i32.const 4
    i32.add
    local.get 1
    i32.store align=1
    local.get 12
    i32.const 24
    i32.add
    local.tee 0
    local.get 2
    i32.load align=1
    i32.store align=1
    local.get 12
    i32.const 28
    i32.add
    local.get 1
    i32.store align=1
    local.get 0
    i32.load align=1
    local.set 0
    local.get 12
    i32.const 20
    i32.add
    local.get 0
    i32.store align=1
    local.get 12
    i32.const 8
    i32.add
    local.tee 2
    local.get 0
    i32.store align=1
    local.get 12
    i32.const 12
    i32.add
    local.tee 3
    local.get 0
    local.get 1
    i32.const 3
    i32.shl
    i32.add
    i32.store align=1
    local.get 12
    i32.const 32
    i32.add
    local.set 1
    local.get 12
    i32.const 36
    i32.add
    local.set 0
    local.get 12
    i32.const 40
    i32.add
    local.set 4
    local.get 12
    i32.const 44
    i32.add
    local.set 5
    local.get 12
    i32.const 16
    i32.add
    local.set 6
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    local.set 8
    local.set 7
    loop ;; label = @1
      local.get 1
      local.get 2
      i32.load align=1
      i32.store align=1
      local.get 3
      i32.load align=1
      local.set 9
      local.get 0
      local.get 9
      i32.store align=1
      local.get 1
      i32.load align=1
      local.tee 10
      local.get 9
      i32.eq
      if ;; label = @2
        local.get 7
        local.get 12
        i32.const 48
        i32.add
        global.set $__stack_pointer
        return
      else
        local.get 10
        i32.const 8
        i32.add
        local.set 10
        local.get 4
        local.get 10
        i32.store align=1
        local.get 2
        local.get 10
        i32.store align=1
        local.get 1
        i32.load align=1
        local.set 10
        local.get 5
        local.get 10
        i32.store align=1
        local.get 6
        local.get 10
        i32.store align=1
        local.get 10
        f64.load align=1
        local.get 8
        f64.sub
        local.set 8
        local.get 7
        local.get 8
        f64.add
        local.tee 11
        local.get 11
        local.get 7
        f64.sub
        local.get 8
        f64.sub
        local.set 8
        local.set 7
        br 1 (;@1;)
      end
    end
    unreachable
  )

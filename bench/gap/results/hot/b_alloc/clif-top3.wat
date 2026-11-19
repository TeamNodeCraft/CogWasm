(func $_RNvXs3_NtNtCsk7Mwp4gGzxp_4core4iter5rangeINtNtNtB9_3ops5range5RangelENtB5_17RangeIteratorImpl9spec_nextCs6R0IlkGltaz_7b_alloc (;49;) (type 2) (param i32 i32)
    (local i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 4
    i32.const 8
    i32.add
    local.tee 2
    local.get 1
    i32.store align=1
    local.get 1
    local.get 1
    i32.const 4
    i32.add
    call $_RNvXs1c_NtNtCsk7Mwp4gGzxp_4core3cmp5implslNtB8_10PartialOrd2lt
    i32.const 255
    i32.and
    local.tee 1
    if ;; label = @1
      local.get 2
      i32.load align=1
      i32.load align=1
      local.tee 1
      i32.const 1
      call $_RNvXsI_NtNtCsk7Mwp4gGzxp_4core4iter5rangelNtB5_4Step17forward_unchecked
      local.set 3
      local.get 2
      i32.load align=1
      local.get 3
      i32.store align=1
      local.get 4
      i32.const 4
      i32.add
      local.tee 3
      local.get 1
      i32.store align=1
      local.get 4
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
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    else
      local.get 4
      i32.const 0
      i32.store align=1
      local.get 4
      i32.const 4
      i32.add
      i32.load align=1
      local.set 1
      local.get 0
      i32.const 0
      i32.store
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 4
      i32.const 16
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
  )

(func $_RINvXs2J_NtNtCsk7Mwp4gGzxp_4core5slice4iterINtB7_4IterdENtNtNtNtBb_4iter6traits8iterator8Iterator4folddNCINvXs28_NtBW_5accumdINtB1N_3SumRdE3sumBF_E0ECskuAql209qb2_8c_plugin (;2;) (type 1) (param i32 i32 f64) (result f64)
    (local i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 8
    global.set $__stack_pointer
    local.get 8
    i32.const 0
    i32.add
    local.tee 3
    local.get 0
    i32.store align=1
    local.get 8
    i32.const 4
    i32.add
    local.tee 0
    local.get 1
    i32.store align=1
    local.get 8
    i32.const 8
    i32.add
    local.tee 4
    local.get 1
    i32.store align=1
    local.get 3
    i32.load align=1
    local.set 1
    local.get 8
    i32.const 32
    i32.add
    local.get 1
    i32.store align=1
    local.get 4
    i32.load align=1
    local.set 4
    block ;; label = @1
      local.get 1
      local.get 4
      i32.eq
      if ;; label = @2
        br 1 (;@1;)
      else
        local.get 8
        i32.const 12
        i32.add
        local.tee 4
        local.get 0
        i32.load align=1
        i32.store align=1
        local.get 3
        i32.load align=1
        local.set 0
        local.get 8
        i32.const 16
        i32.add
        local.get 0
        i32.store align=1
        local.get 4
        i32.load align=1
        local.set 4
        local.get 8
        i32.const 28
        i32.add
        local.set 1
        local.get 8
        i32.const 24
        i32.add
        local.set 5
        local.get 8
        i32.const 20
        i32.add
        local.set 6
        local.get 4
        local.get 0
        i32.sub
        i32.const 3
        i32.shr_u
        local.set 4
        i32.const 0
        local.set 0
        loop ;; label = @3
          local.get 3
          i32.load align=1
          local.set 7
          local.get 1
          local.get 7
          i32.store align=1
          local.get 7
          local.get 0
          i32.const 3
          i32.shl
          i32.add
          local.set 7
          local.get 5
          local.get 7
          i32.store align=1
          local.get 6
          local.get 7
          i32.store align=1
          i32.const 1
          local.get 2
          local.get 7
          call $_RNCINvXs28_NtNtNtCsk7Mwp4gGzxp_4core4iter6traits5accumdINtB9_3SumRdE3sumINtNtNtBf_5slice4iter4IterdEE0CskuAql209qb2_8c_plugin
          local.set 2
          local.get 0
          i32.const 1
          i32.add
          local.tee 0
          local.get 4
          i32.eq
          if ;; label = @4
            br 3 (;@1;)
          else
            br 1 (;@3;)
          end
        end
      end
    end
    local.get 2
    local.get 8
    i32.const 48
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

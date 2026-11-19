(func $_RNvCskuAql209qb2_8c_plugin4eval (;7;) (type 6) (param i32 i32) (result f64)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 f64 i32)
    global.get $__stack_pointer
    i32.const 128
    i32.sub
    local.tee 15
    global.set $__stack_pointer
    local.get 15
    local.tee 2
    local.get 0
    i32.store align=1
    local.get 15
    i32.const 4
    i32.add
    local.tee 0
    local.get 1
    i32.store align=1
    local.get 15
    i32.const 8
    i32.add
    local.set 1
    i32.const 0
    local.set 3
    loop ;; label = @1
      local.get 3
      i32.const 4
      i32.eq
      if ;; label = @2
        local.get 2
        i32.load align=1
        local.set 2
        local.get 0
        i32.load align=1
        local.set 0
        local.get 15
        i32.const 88
        i32.add
        local.tee 3
        local.get 2
        i32.store align=1
        local.get 15
        i32.const 92
        i32.add
        local.get 0
        i32.store align=1
        local.get 3
        i32.load align=1
        local.set 3
        local.get 15
        i32.const 84
        i32.add
        local.get 3
        i32.store align=1
        local.get 15
        i32.const 52
        i32.add
        local.tee 2
        local.get 3
        i32.store align=1
        local.get 3
        local.get 0
        i32.const 3
        i32.shl
        i32.add
        local.set 3
        local.get 15
        i32.const 56
        i32.add
        local.get 3
        i32.store align=1
        local.get 15
        i32.const 40
        i32.add
        local.tee 4
        local.get 2
        i32.load align=1
        i32.store align=1
        local.get 15
        i32.const 44
        i32.add
        local.tee 2
        local.get 3
        i32.store align=1
        local.get 15
        i32.const 48
        i32.add
        local.tee 3
        i32.const 0
        i32.store align=1
        local.get 15
        i32.const 100
        i32.add
        local.set 5
        local.get 15
        i32.const 104
        i32.add
        local.set 6
        local.get 15
        i32.const 108
        i32.add
        local.set 7
        local.get 15
        i32.const 112
        i32.add
        local.set 8
        local.get 15
        i32.const 96
        i32.add
        local.set 9
        local.get 15
        i32.const 60
        i32.add
        local.set 10
        local.get 15
        i32.const 64
        i32.add
        local.set 11
        loop ;; label = @3
          local.get 5
          local.get 4
          i32.load align=1
          i32.store align=1
          local.get 2
          i32.load align=1
          local.set 12
          local.get 6
          local.get 12
          i32.store align=1
          local.get 5
          i32.load align=1
          local.tee 13
          local.get 12
          i32.eq
          if ;; label = @4
            local.get 15
            i32.const 76
            i32.add
            local.tee 11
            local.get 1
            i32.store align=1
            local.get 15
            i32.const 80
            i32.add
            i32.const 4
            i32.store align=1
            local.get 15
            i32.const 120
            i32.add
            local.tee 1
            local.get 11
            i32.load align=1
            i32.store align=1
            local.get 15
            i32.const 124
            i32.add
            i32.const 4
            i32.store align=1
            local.get 1
            i32.load align=1
            local.set 1
            local.get 15
            i32.const 116
            i32.add
            local.get 1
            i32.store align=1
            local.get 15
            i32.const 68
            i32.add
            local.tee 11
            local.get 1
            i32.store align=1
            local.get 1
            i32.const 32
            i32.add
            local.set 1
            local.get 15
            i32.const 72
            i32.add
            local.get 1
            i32.store align=1
            local.get 11
            i32.load align=1
            local.get 1
            f64.const -0x0p+0 (;=-0;)
            call $_RINvXs2J_NtNtCsk7Mwp4gGzxp_4core5slice4iterINtB7_4IterdENtNtNtNtBb_4iter6traits8iterator8Iterator4folddNCINvXs28_NtBW_5accumdINtB1N_3SumRdE3sumBF_E0ECskuAql209qb2_8c_plugin
            local.tee 14
            local.get 0
            f64.convert_i32_u
            f64.div
            local.get 15
            i32.const 128
            i32.add
            global.set $__stack_pointer
            return
          else
            local.get 13
            i32.const 8
            i32.add
            local.set 13
            local.get 7
            local.get 13
            i32.store align=1
            local.get 4
            local.get 13
            i32.store align=1
            local.get 5
            i32.load align=1
            local.set 13
            local.get 8
            local.get 13
            i32.store align=1
            local.get 9
            local.get 13
            i32.store align=1
            local.get 3
            i32.load align=1
            local.set 12
            local.get 3
            local.get 12
            i32.const 1
            i32.add
            i32.store align=1
            local.get 10
            local.get 12
            i32.store align=1
            local.get 11
            local.get 13
            i32.store align=1
            local.get 10
            i32.load align=1
            local.set 12
            local.get 13
            f64.load align=1
            call $host_bias
            local.set 14
            local.get 12
            i32.const 4
            i32.lt_u
            if ;; label = @5
              local.get 1
              local.get 12
              i32.const 3
              i32.shl
              i32.add
              local.get 14
              f64.store align=1
              br 2 (;@3;)
            else
              local.get 12
              i32.const 4
              i32.const 1065112
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          end
        end
      else
        local.get 1
        local.get 3
        i32.const 3
        i32.shl
        i32.add
        f64.const 0x0p+0 (;=0;)
        f64.store align=1
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        br 1 (;@1;)
      end
    end
    unreachable
  )

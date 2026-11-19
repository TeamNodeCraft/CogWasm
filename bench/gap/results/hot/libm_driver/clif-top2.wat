(func $_RINvNtNtNtCseievaMsdffa_4libm4math7generic4sqrt11goldschmidtdyEB8_ (;44;) (type 22) (param i32 i64 i64 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 64
    i32.sub
    local.tee 17
    global.set $__stack_pointer
    local.get 4
    i32.const 255
    i32.and
    local.set 4
    local.get 17
    i32.const 0
    i32.add
    local.tee 5
    local.get 2
    i64.store align=1
    local.get 17
    i32.const 12
    i32.add
    local.set 6
    local.get 17
    i32.const 8
    i32.add
    local.set 7
    local.get 17
    i32.const 48
    i32.add
    local.set 8
    i32.const 1067132
    local.set 9
    local.get 17
    i32.const 16
    i32.add
    local.set 10
    local.get 17
    i32.const 32
    i32.add
    local.set 11
    i32.const 0
    local.get 1
    local.set 2
    local.set 12
    loop ;; label = @1
      local.get 12
      local.get 3
      i32.lt_u
      if ;; label = @2
        local.get 6
        local.get 12
        i32.store align=1
        local.get 7
        i32.const 1
        i32.store align=1
        local.get 6
        i32.load align=1
        local.set 13
        local.get 8
        local.get 5
        i64.load align=1
        local.get 1
        call $_RNvXsy_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyNtB5_4HInt9widen_mul
        local.get 8
        i64.load align=1
        local.set 1
        local.get 8
        i64.load offset=8 align=1
        local.set 14
        block ;; label = @3
          i32.const 64
          i64.extend_i32_u
          i64.const 127
          i64.and
          local.tee 15
          i64.const 63
          i64.and
          local.set 16
          local.get 5
          local.get 14
          local.get 16
          i64.shr_u
          local.get 1
          local.get 16
          i64.shr_u
          local.get 14
          i64.const 1
          i64.shl
          i64.const 63
          local.get 16
          i64.sub
          i64.shl
          i64.or
          local.get 15
          i64.const 64
          i64.ge_u
          select
          i64.store align=1
          local.get 13
          if ;; label = @4
            block ;; label = @5
              local.get 4
              if ;; label = @6
                local.get 13
                i32.const 1
                i32.add
                local.get 3
                i32.lt_u
                if ;; label = @7
                  br 2 (;@5;)
                else
                  br 4 (;@3;)
                end
              else
                br 1 (;@5;)
              end
            end
            local.get 5
            i32.const 1
            local.get 9
            call $_RNvXsob_NtNtCsk7Mwp4gGzxp_4core3ops3bityINtB6_9ShlAssignlE10shl_assign
            br 1 (;@3;)
          else
            br 1 (;@3;)
          end
        end
        local.get 10
        local.get 5
        i64.load align=1
        local.get 2
        call $_RNvXsy_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyNtB5_4HInt9widen_mul
        local.get 10
        i64.load align=1
        local.set 16
        local.get 10
        i64.load offset=8 align=1
        local.set 15
        i32.const 64
        i64.extend_i32_u
        i64.const 127
        i64.and
        local.tee 14
        i64.const 63
        i64.and
        local.set 1
        i64.const -4611686018427387904
        local.get 15
        local.get 1
        i64.shr_u
        local.get 16
        local.get 1
        i64.shr_u
        local.get 15
        i64.const 1
        i64.shl
        i64.const 63
        local.get 1
        i64.sub
        i64.shl
        i64.or
        local.get 14
        i64.const 64
        i64.ge_u
        select
        i64.sub
        local.set 1
        local.get 11
        local.get 2
        local.get 1
        call $_RNvXsy_NtNtNtCseievaMsdffa_4libm4math7support10int_traitsyNtB5_4HInt9widen_mul
        local.get 11
        i64.load align=1
        local.set 16
        local.get 11
        i64.load offset=8 align=1
        local.set 15
        i32.const 64
        i64.extend_i32_u
        i64.const 127
        i64.and
        local.tee 14
        i64.const 63
        i64.and
        local.set 2
        local.get 12
        i32.const 1
        i32.add
        local.get 15
        local.get 2
        i64.shr_u
        local.get 16
        local.get 2
        i64.shr_u
        local.get 15
        i64.const 1
        i64.shl
        i64.const 63
        local.get 2
        i64.sub
        i64.shl
        i64.or
        local.get 14
        i64.const 64
        i64.ge_u
        select
        i32.const 1
        i64.extend_i32_u
        i64.shl
        local.set 2
        local.set 12
        br 1 (;@1;)
      else
        local.get 5
        i64.load align=1
        local.set 1
        local.get 0
        local.get 2
        i64.store align=1
        local.get 0
        local.get 1
        i64.store offset=8 align=1
        local.get 17
        i32.const 64
        i32.add
        global.set $__stack_pointer
        return
      end
    end
    unreachable
  )

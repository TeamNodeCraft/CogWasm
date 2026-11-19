(func $_RNvCs9103VBb1EJO_6sha2568compress (;5;) (type 7) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 528
    i32.sub
    local.tee 17
    global.set $__stack_pointer
    local.get 17
    i32.const 0
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop ;; label = @1
      local.get 4
      i32.const 64
      i32.eq
      if ;; label = @2
        local.get 17
        i32.const 256
        i32.add
        local.tee 4
        i32.const 1
        i32.add
        local.set 5
        local.get 4
        i32.const 2
        i32.add
        local.set 6
        i32.const 0
        local.set 7
        loop ;; label = @3
          local.get 7
          i32.const 16
          i32.lt_u
          if ;; label = @4
            local.get 7
            i32.const 2
            i32.shl
            local.tee 13
            local.get 2
            i32.lt_u
            if ;; label = @5
              local.get 1
              local.get 13
              i32.add
              i32.load8_u
              i32.const 255
              i32.and
              local.set 12
              local.get 13
              i32.const 1
              i32.add
              local.tee 11
              local.get 2
              i32.lt_u
              if ;; label = @6
                local.get 1
                local.get 11
                i32.add
                i32.load8_u
                i32.const 255
                i32.and
                local.set 11
                local.get 13
                i32.const 2
                i32.add
                local.tee 10
                local.get 2
                i32.lt_u
                if ;; label = @7
                  local.get 1
                  local.get 10
                  i32.add
                  i32.load8_u
                  i32.const 255
                  i32.and
                  local.set 10
                  local.get 13
                  i32.const 3
                  i32.add
                  local.get 2
                  i32.lt_u
                  if ;; label = @8
                    local.get 1
                    local.get 13
                    i32.const 3
                    i32.add
                    i32.add
                    i32.load8_u
                    i32.const 255
                    i32.and
                    local.set 9
                    local.get 4
                    local.get 12
                    i32.store8
                    local.get 5
                    local.get 11
                    i32.store8
                    local.get 6
                    local.get 10
                    i32.store8
                    local.get 4
                    i32.const 3
                    i32.add
                    local.get 9
                    i32.store8
                    local.get 4
                    i32.load align=1
                    local.set 9
                    local.get 7
                    i32.const 64
                    i32.lt_u
                    if ;; label = @9
                      local.get 3
                      local.get 13
                      i32.add
                      local.get 9
                      local.tee 18
                      i32.const 16711935
                      i32.and
                      i32.const 8
                      i32.rotr
                      local.get 18
                      i32.const -16711936
                      i32.and
                      i32.const 8
                      i32.rotl
                      i32.or
                      i32.store align=1
                      local.get 7
                      i32.const 1
                      i32.add
                      local.set 7
                      br 6 (;@3;)
                    else
                      local.get 7
                      i32.const 64
                      i32.const 1065748
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  else
                    local.get 13
                    i32.const 3
                    i32.add
                    local.get 2
                    i32.const 1065844
                    call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                    unreachable
                  end
                else
                  local.get 10
                  local.get 2
                  i32.const 1065860
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 11
                local.get 2
                i32.const 1065876
                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                unreachable
              end
            else
              local.get 13
              local.get 2
              i32.const 1065892
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          else
            loop ;; label = @5
              local.get 7
              i32.const 64
              i32.lt_u
              local.tee 6
              if ;; label = @6
                local.get 7
                i32.const 15
                i32.sub
                local.tee 13
                i32.const 64
                i32.lt_u
                if ;; label = @7
                  local.get 3
                  local.get 13
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load align=1
                  local.set 13
                  local.get 7
                  i32.const 2
                  i32.sub
                  local.tee 12
                  i32.const 64
                  i32.lt_u
                  if ;; label = @8
                    local.get 3
                    local.get 12
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load align=1
                    local.set 12
                    local.get 7
                    i32.const 16
                    i32.sub
                    local.tee 11
                    i32.const 64
                    i32.lt_u
                    if ;; label = @9
                      local.get 3
                      local.get 11
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load align=1
                      local.set 11
                      local.get 7
                      i32.const 7
                      i32.sub
                      local.tee 10
                      i32.const 64
                      i32.lt_u
                      if ;; label = @10
                        local.get 3
                        local.get 10
                        i32.const 2
                        i32.shl
                        i32.add
                        i32.load align=1
                        local.set 10
                        local.get 6
                        if ;; label = @11
                          local.get 3
                          local.get 7
                          i32.const 2
                          i32.shl
                          i32.add
                          local.get 12
                          i32.const 17
                          i32.rotr
                          local.get 12
                          i32.const 19
                          i32.rotr
                          i32.xor
                          local.get 12
                          i32.const 10
                          i32.shr_u
                          i32.xor
                          local.get 13
                          i32.const 7
                          i32.rotr
                          local.get 13
                          i32.const 18
                          i32.rotr
                          i32.xor
                          local.get 13
                          i32.const 3
                          i32.shr_u
                          i32.xor
                          local.get 11
                          i32.add
                          local.get 10
                          i32.add
                          i32.add
                          i32.store align=1
                          local.get 7
                          i32.const 1
                          i32.add
                          local.set 7
                          br 6 (;@5;)
                        else
                          local.get 7
                          i32.const 64
                          i32.const 1065780
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        local.get 10
                        i32.const 64
                        i32.const 1065796
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 11
                      i32.const 64
                      i32.const 1065732
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  else
                    local.get 12
                    i32.const 64
                    i32.const 1065812
                    call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                    unreachable
                  end
                else
                  local.get 13
                  i32.const 64
                  i32.const 1065828
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 0
                i32.load align=1
                local.set 7
                local.get 0
                i32.const 4
                i32.add
                i32.load align=1
                local.set 6
                local.get 0
                i32.const 8
                i32.add
                i32.load align=1
                local.set 5
                local.get 0
                i32.const 12
                i32.add
                i32.load align=1
                local.set 4
                local.get 0
                i32.const 16
                i32.add
                local.set 2
                local.get 17
                i32.const 260
                i32.add
                local.set 1
                i32.const 1065476
                local.set 8
                i32.const 0
                local.get 2
                i32.load align=1
                local.get 7
                local.get 6
                local.get 5
                local.get 4
                local.get 0
                i32.const 20
                i32.add
                i32.load align=1
                local.get 0
                i32.const 24
                i32.add
                i32.load align=1
                local.get 0
                i32.const 28
                i32.add
                i32.load align=1
                local.set 13
                local.set 12
                local.set 11
                local.set 10
                local.set 9
                local.set 4
                local.set 5
                local.set 6
                local.set 7
                loop ;; label = @7
                  local.get 7
                  i32.const 64
                  i32.lt_u
                  local.tee 14
                  if ;; label = @8
                    local.get 1
                    local.get 8
                    i32.const 256
                    call $memcpy
                    drop
                    local.get 14
                    if ;; label = @9
                      local.get 7
                      i32.const 2
                      i32.shl
                      local.set 15
                      local.get 1
                      local.get 15
                      i32.add
                      i32.load align=1
                      local.set 16
                      local.get 14
                      if ;; label = @10
                        local.get 3
                        local.get 15
                        i32.add
                        i32.load align=1
                        local.set 15
                        local.get 6
                        local.get 12
                        i32.and
                        local.get 6
                        local.get 11
                        i32.and
                        local.get 12
                        i32.xor
                        i32.xor
                        local.get 6
                        i32.const 6
                        i32.rotr
                        local.get 6
                        i32.const 11
                        i32.rotr
                        i32.xor
                        local.get 6
                        i32.const 25
                        i32.rotr
                        i32.xor
                        local.get 13
                        i32.add
                        i32.add
                        local.get 16
                        i32.add
                        local.get 15
                        i32.add
                        local.set 16
                        local.get 7
                        i32.const 1
                        i32.add
                        local.get 16
                        local.get 10
                        i32.add
                        local.get 16
                        local.get 5
                        i32.const 2
                        i32.rotr
                        local.get 5
                        i32.const 13
                        i32.rotr
                        i32.xor
                        local.get 5
                        i32.const 22
                        i32.rotr
                        i32.xor
                        local.get 5
                        local.get 4
                        local.get 9
                        i32.xor
                        i32.and
                        local.get 4
                        local.get 9
                        i32.and
                        i32.xor
                        i32.add
                        i32.add
                        local.get 5
                        local.get 4
                        local.get 9
                        local.get 6
                        local.get 11
                        local.get 12
                        local.set 13
                        local.set 12
                        local.set 11
                        local.set 10
                        local.set 9
                        local.set 4
                        local.set 5
                        local.set 6
                        local.set 7
                        br 3 (;@7;)
                      else
                        local.get 7
                        i32.const 64
                        i32.const 1065764
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 7
                      i32.const 64
                      i32.const 1065352
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  else
                    local.get 0
                    local.get 0
                    i32.load align=1
                    local.get 5
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 4
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 4
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 8
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 9
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 12
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 10
                    i32.add
                    i32.store align=1
                    local.get 2
                    local.get 2
                    i32.load align=1
                    local.get 6
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 20
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 11
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 24
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 12
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 28
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 13
                    i32.add
                    i32.store align=1
                    local.get 17
                    i32.const 528
                    i32.add
                    global.set $__stack_pointer
                    return
                  end
                end
              end
            end
          end
        end
      else
        local.get 3
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        i32.const 0
        i32.store align=1
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        br 1 (;@1;)
      end
    end
    unreachable
  )

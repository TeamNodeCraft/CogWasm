(func $_RNvCshS8UloB9tPT_6matmul6matmul (;10;) (type 7) (param i32 i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 f64 i32 i32 f64)
    i32.const 0
    local.set 7
    loop ;; label = @1
      local.get 7
      local.get 6
      i32.lt_u
      if ;; label = @2
        local.get 6
        local.get 7
        i32.mul
        local.set 8
        i32.const 0
        local.set 9
        loop ;; label = @3
          local.get 9
          local.get 6
          i32.lt_u
          if ;; label = @4
            local.get 8
            local.get 9
            i32.add
            local.tee 12
            local.get 5
            i32.lt_u
            if ;; label = @5
              local.get 4
              local.get 12
              i32.const 3
              i32.shl
              i32.add
              f64.const 0x0p+0 (;=0;)
              f64.store align=1
              local.get 9
              i32.const 1
              i32.add
              local.set 9
              br 2 (;@3;)
            else
              local.get 12
              local.get 5
              i32.const 1065460
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          else
            i32.const 0
            local.set 9
            loop ;; label = @5
              local.get 9
              local.get 6
              i32.lt_u
              if ;; label = @6
                local.get 8
                local.get 9
                i32.add
                local.tee 10
                local.get 1
                i32.lt_u
                if ;; label = @7
                  local.get 0
                  local.get 10
                  i32.const 3
                  i32.shl
                  i32.add
                  f64.load align=1
                  local.set 11
                  local.get 6
                  local.get 9
                  i32.mul
                  local.set 10
                  i32.const 0
                  local.set 12
                  loop ;; label = @8
                    local.get 12
                    local.get 6
                    i32.lt_u
                    if ;; label = @9
                      local.get 10
                      local.get 12
                      i32.add
                      local.tee 13
                      local.get 3
                      i32.lt_u
                      if ;; label = @10
                        local.get 2
                        local.get 13
                        i32.const 3
                        i32.shl
                        i32.add
                        f64.load align=1
                        local.set 14
                        local.get 8
                        local.get 12
                        i32.add
                        local.tee 13
                        local.get 5
                        i32.lt_u
                        if ;; label = @11
                          local.get 4
                          local.get 13
                          i32.const 3
                          i32.shl
                          i32.add
                          local.tee 13
                          local.get 13
                          f64.load align=1
                          local.get 11
                          local.get 14
                          f64.mul
                          f64.add
                          f64.store align=1
                          local.get 12
                          i32.const 1
                          i32.add
                          local.set 12
                          br 3 (;@8;)
                        else
                          local.get 13
                          local.get 5
                          i32.const 1065304
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        local.get 13
                        local.get 3
                        i32.const 1065428
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 9
                      i32.const 1
                      i32.add
                      local.set 9
                      br 4 (;@5;)
                    end
                  end
                else
                  local.get 10
                  local.get 1
                  i32.const 1065444
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 7
                i32.const 1
                i32.add
                local.set 7
                br 5 (;@1;)
              end
            end
          end
        end
      else
        return
      end
    end
    unreachable
  )

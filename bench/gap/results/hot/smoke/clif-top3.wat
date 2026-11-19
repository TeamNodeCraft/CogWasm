(func $_RNvNtNtCs1xoxxhqgeCT_17compiler_builtins3mem5impls13copy_backward (;121;) (type 6) (param i32 i32 i32)
    (local i32 i32 i32)
    block ;; label = @1
      local.get 2
      i32.const 16
      i32.ge_u
      if ;; label = @2
        local.get 0
        local.get 2
        i32.add
        local.set 0
        local.get 1
        local.get 2
        i32.add
        local.set 1
        local.get 0
        i32.const 3
        i32.and
        local.set 3
        local.get 0
        local.get 3
        i32.sub
        local.set 4
        local.get 2
        local.get 3
        i32.sub
        local.set 3
        local.get 0
        local.get 1
        local.set 5
        local.set 2
        loop ;; label = @3
          local.get 4
          local.get 2
          i32.lt_u
          if ;; label = @4
            local.get 5
            i32.const -1
            i32.add
            local.set 5
            local.get 2
            i32.const -1
            i32.add
            local.tee 2
            local.get 5
            i32.load8_u
            i32.const 255
            i32.and
            i32.store8
            br 1 (;@3;)
          else
            block ;; label = @5
              local.get 1
              local.get 0
              i32.const 3
              i32.and
              i32.sub
              local.tee 5
              i32.const 3
              i32.and
              if ;; label = @6
                call $_RNvNtNtNtCs1xoxxhqgeCT_17compiler_builtins4math9libm_math7support9cold_path
                local.get 4
                local.get 5
                local.get 3
                i32.const -4
                i32.and
                call $_RNvNvNtNtCs1xoxxhqgeCT_17compiler_builtins3mem5impls13copy_backward30copy_backward_misaligned_words
                br 1 (;@5;)
              else
                local.get 4
                local.get 3
                i32.const -4
                i32.and
                i32.sub
                local.set 2
                local.get 4
                local.get 5
                local.set 0
                local.set 1
                loop ;; label = @7
                  local.get 2
                  local.get 1
                  i32.lt_u
                  if ;; label = @8
                    local.get 0
                    i32.const -4
                    i32.add
                    local.set 0
                    local.get 1
                    i32.const -4
                    i32.add
                    local.tee 1
                    local.get 0
                    i32.load align=1
                    i32.store align=1
                    br 1 (;@7;)
                  else
                    br 3 (;@5;)
                  end
                end
              end
            end
            local.get 3
            i32.const -4
            i32.and
            local.set 2
            local.get 4
            local.get 2
            i32.sub
            local.get 3
            local.get 2
            i32.sub
            local.get 5
            local.get 2
            i32.sub
            local.set 0
            local.set 1
            local.set 2
            br 3 (;@1;)
          end
        end
      else
        local.get 0
        local.get 2
        i32.add
        local.get 2
        local.get 1
        local.get 2
        i32.add
        local.set 0
        local.set 1
        local.set 2
        br 1 (;@1;)
      end
    end
    local.get 2
    local.get 1
    i32.sub
    local.set 5
    loop ;; label = @1
      local.get 5
      local.get 2
      i32.lt_u
      if ;; label = @2
        local.get 0
        i32.const -1
        i32.add
        local.set 0
        local.get 2
        i32.const -1
        i32.add
        local.tee 2
        local.get 0
        i32.load8_u
        i32.const 255
        i32.and
        i32.store8
        br 1 (;@1;)
      else
        return
      end
    end
    unreachable
  )

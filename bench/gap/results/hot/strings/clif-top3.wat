(func $_RINvNtNtCsk7Mwp4gGzxp_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECshJOoaGw8lTO_7strings (;6;) (type 3) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 6
    global.set $__stack_pointer
    local.get 1
    call $_RNvXs2J_NtNtCsk7Mwp4gGzxp_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCshJOoaGw8lTO_7strings
    local.set 2
    local.get 6
    i32.const 12
    i32.add
    local.get 2
    i32.store align=1
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 1
              i32.const 0
              local.get 2
              select
              br_table 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            br 3 (;@1;)
          end
          i32.const 1048784
          local.tee 2
          i32.load align=1
          local.set 4
          local.get 2
          i32.load offset=4 align=1
          local.set 2
          local.get 6
          local.get 4
          i32.store align=1
          local.get 6
          i32.const 4
          i32.add
          local.get 2
          i32.store align=1
          br 1 (;@2;)
        end
        local.get 6
        i32.const 8
        i32.add
        local.get 2
        i32.store align=1
        local.get 2
        i32.load8_u
        local.tee 2
        i32.const 128
        i32.lt_u
        if ;; label = @3
          local.get 6
          i32.const 4
          i32.add
          local.get 2
          i32.store align=1
          local.get 6
          i32.const 1
          i32.store align=1
          br 1 (;@2;)
        else
          local.get 1
          call $_RNvXs2J_NtNtCsk7Mwp4gGzxp_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCshJOoaGw8lTO_7strings
          local.set 3
          local.get 6
          i32.const 16
          i32.add
          local.get 3
          i32.store align=1
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                i32.const 1
                i32.const 0
                local.get 3
                select
                br_table 1 (;@5;) 2 (;@4;) 0 (;@6;)
              end
              br 4 (;@1;)
            end
            i32.const 1048768
            call $_RNvNvNtCsk7Mwp4gGzxp_4core4hint21unreachable_unchecked18precondition_check
            br 3 (;@1;)
          end
          block ;; label = @4
            local.get 2
            i32.const 31
            i32.and
            local.set 4
            local.get 3
            i32.load8_u
            i32.const 63
            i32.and
            local.set 3
            local.get 2
            i32.const 224
            i32.ge_u
            if ;; label = @5
              local.get 1
              call $_RNvXs2J_NtNtCsk7Mwp4gGzxp_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCshJOoaGw8lTO_7strings
              local.set 5
              local.get 6
              i32.const 20
              i32.add
              local.get 5
              i32.store align=1
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    i32.const 1
                    i32.const 0
                    local.get 5
                    select
                    br_table 1 (;@7;) 2 (;@6;) 0 (;@8;)
                  end
                  br 6 (;@1;)
                end
                i32.const 1048752
                call $_RNvNvNtCsk7Mwp4gGzxp_4core4hint21unreachable_unchecked18precondition_check
                br 5 (;@1;)
              end
              block ;; label = @6
                local.get 3
                i32.const 6
                i32.shl
                local.get 5
                i32.load8_u
                i32.const 63
                i32.and
                i32.or
                local.set 3
                local.get 2
                i32.const 240
                i32.ge_u
                if ;; label = @7
                  local.get 1
                  call $_RNvXs2J_NtNtCsk7Mwp4gGzxp_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCshJOoaGw8lTO_7strings
                  local.set 1
                  local.get 6
                  i32.const 24
                  i32.add
                  local.get 1
                  i32.store align=1
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        i32.const 1
                        i32.const 0
                        local.get 1
                        select
                        br_table 1 (;@9;) 2 (;@8;) 0 (;@10;)
                      end
                      br 8 (;@1;)
                    end
                    i32.const 1048576
                    call $_RNvNvNtCsk7Mwp4gGzxp_4core4hint21unreachable_unchecked18precondition_check
                    br 7 (;@1;)
                  end
                  local.get 4
                  i32.const 7
                  i32.and
                  i32.const 18
                  i32.shl
                  local.get 3
                  i32.const 6
                  i32.shl
                  local.get 1
                  i32.load8_u
                  i32.const 63
                  i32.and
                  i32.or
                  i32.or
                  local.set 5
                  br 1 (;@6;)
                else
                  local.get 3
                  local.get 4
                  i32.const 12
                  i32.shl
                  i32.or
                  local.set 5
                  br 1 (;@6;)
                end
              end
              br 1 (;@4;)
            else
              local.get 4
              i32.const 6
              i32.shl
              local.get 3
              i32.or
              local.set 5
              br 1 (;@4;)
            end
          end
          local.get 6
          i32.const 4
          i32.add
          local.tee 4
          local.get 5
          i32.store align=1
          local.get 6
          i32.const 1
          i32.store align=1
          local.get 4
          i32.load align=1
          local.set 4
          local.get 0
          i32.const 1
          i32.store
          local.get 0
          local.get 4
          i32.store offset=4
          local.get 6
          i32.const 32
          i32.add
          global.set $__stack_pointer
          return
        end
      end
      local.get 6
      i32.load align=1
      local.set 2
      local.get 6
      i32.const 4
      i32.add
      i32.load align=1
      local.set 4
      local.get 0
      local.get 2
      i32.store
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 6
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
    unreachable
  )

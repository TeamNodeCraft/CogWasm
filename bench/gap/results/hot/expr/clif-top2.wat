(func $_RNvMCsizV4RGem7kw_4exprNtB2_5Lexer10next_token (;35;) (type 5) (param i32 i32)
    (local i32 i32 i32 i32 i32 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 192
    i32.sub
    local.tee 9
    global.set $__stack_pointer
    local.get 9
    i32.const 176
    i32.add
    local.set 2
    local.get 9
    i32.const 180
    i32.add
    local.set 3
    loop ;; label = @1
      block ;; label = @2
        local.get 1
        i32.load align=1
        local.set 4
        local.get 1
        i32.load offset=4 align=1
        local.set 5
        local.get 2
        local.get 4
        i32.store align=1
        local.get 3
        local.get 5
        i32.store align=1
        local.get 1
        i32.load offset=8 align=1
        local.set 4
        local.get 2
        i32.load align=1
        local.set 6
        local.get 4
        local.get 5
        i32.lt_u
        if ;; label = @3
          block ;; label = @4
            local.get 6
            local.get 4
            i32.add
            i32.load8_u
            local.tee 6
            i32.const 32
            i32.eq
            if ;; label = @5
              br 1 (;@4;)
            else
              local.get 6
              i32.const 13
              i32.eq
              if ;; label = @6
                br 2 (;@4;)
              else
                block ;; label = @7
                  local.get 6
                  i32.const 9
                  i32.ge_u
                  if ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        local.get 6
                        i32.const 247
                        i32.add
                        i32.const 255
                        i32.and
                        br_table 1 (;@9;) 1 (;@9;) 0 (;@10;)
                      end
                      br 2 (;@7;)
                    end
                    br 4 (;@4;)
                  else
                    br 1 (;@7;)
                  end
                end
                br 4 (;@2;)
              end
            end
          end
          local.get 1
          local.get 4
          i32.const 1
          i32.add
          i32.store offset=8 align=1
          br 2 (;@1;)
        else
          br 1 (;@2;)
        end
      end
      local.get 1
      i32.load align=1
      local.set 6
      local.get 1
      i32.load offset=4 align=1
      local.set 5
      local.get 9
      i32.const 168
      i32.add
      local.tee 4
      local.get 6
      i32.store align=1
      local.get 9
      i32.const 172
      i32.add
      local.get 5
      i32.store align=1
      local.get 1
      i32.load offset=8 align=1
      local.set 6
      local.get 4
      i32.load align=1
      local.set 4
      local.get 6
      local.get 5
      i32.lt_u
      if ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 4
              local.get 6
              i32.add
              local.set 6
              local.get 9
              i32.const 16
              i32.add
              local.get 6
              i32.store align=1
              local.get 6
              i32.load8_u
              local.tee 6
              i32.const 47
              i32.ge_u
              if ;; label = @6
                local.get 6
                i32.const 61
                i32.ge_u
                if ;; label = @7
                  local.get 6
                  i32.const 95
                  i32.eq
                  if ;; label = @8
                    br 4 (;@4;)
                  else
                    local.get 6
                    i32.const 61
                    i32.eq
                    if ;; label = @9
                      local.get 1
                      local.get 1
                      i32.load offset=8 align=1
                      i32.const 1
                      i32.add
                      i32.store offset=8 align=1
                      local.get 9
                      i32.const 136
                      i32.add
                      local.tee 6
                      i32.const 10
                      i32.store align=1
                      local.get 6
                      i64.load
                      local.set 8
                      local.get 6
                      i64.load offset=8
                      local.set 7
                      local.get 0
                      local.get 8
                      i64.store
                      local.get 0
                      local.get 7
                      i64.store offset=8
                      br 6 (;@3;)
                    else
                      br 4 (;@5;)
                    end
                  end
                else
                  local.get 6
                  i32.const 59
                  i32.eq
                  if ;; label = @8
                    local.get 1
                    local.get 1
                    i32.load offset=8 align=1
                    i32.const 1
                    i32.add
                    i32.store offset=8 align=1
                    local.get 9
                    i32.const 152
                    i32.add
                    local.tee 5
                    i32.const 11
                    i32.store align=1
                    local.get 5
                    i64.load
                    local.set 7
                    local.get 5
                    i64.load offset=8
                    local.set 8
                    local.get 0
                    local.get 7
                    i64.store
                    local.get 0
                    local.get 8
                    i64.store offset=8
                    br 5 (;@3;)
                  else
                    local.get 6
                    i32.const 47
                    i32.eq
                    if ;; label = @9
                      local.get 1
                      local.get 1
                      i32.load offset=8 align=1
                      i32.const 1
                      i32.add
                      i32.store offset=8 align=1
                      local.get 9
                      i32.const 72
                      i32.add
                      local.tee 5
                      i32.const 6
                      i32.store align=1
                      local.get 5
                      i64.load
                      local.set 8
                      local.get 5
                      i64.load offset=8
                      local.set 7
                      local.get 0
                      local.get 8
                      i64.store
                      local.get 0
                      local.get 7
                      i64.store offset=8
                      br 6 (;@3;)
                    else
                      br 4 (;@5;)
                    end
                  end
                end
              else
                local.get 6
                i32.const 45
                i32.eq
                if ;; label = @7
                  local.get 1
                  local.get 1
                  i32.load offset=8 align=1
                  i32.const 1
                  i32.add
                  i32.store offset=8 align=1
                  local.get 9
                  i32.const 40
                  i32.add
                  local.tee 5
                  i32.const 4
                  i32.store align=1
                  local.get 5
                  i64.load
                  local.set 7
                  local.get 5
                  i64.load offset=8
                  local.set 8
                  local.get 0
                  local.get 7
                  i64.store
                  local.get 0
                  local.get 8
                  i64.store offset=8
                  br 4 (;@3;)
                else
                  local.get 6
                  i32.const 40
                  i32.ge_u
                  if ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            block ;; label = @13
                              local.get 6
                              i32.const 216
                              i32.add
                              i32.const 255
                              i32.and
                              br_table 1 (;@12;) 2 (;@11;) 3 (;@10;) 4 (;@9;) 0 (;@13;)
                            end
                            br 7 (;@5;)
                          end
                          local.get 1
                          local.get 1
                          i32.load offset=8 align=1
                          i32.const 1
                          i32.add
                          i32.store offset=8 align=1
                          local.get 9
                          i32.const 104
                          i32.add
                          local.tee 5
                          i32.const 8
                          i32.store align=1
                          local.get 5
                          i64.load
                          local.set 8
                          local.get 5
                          i64.load offset=8
                          local.set 7
                          local.get 0
                          local.get 8
                          i64.store
                          local.get 0
                          local.get 7
                          i64.store offset=8
                          br 8 (;@3;)
                        end
                        local.get 1
                        local.get 1
                        i32.load offset=8 align=1
                        i32.const 1
                        i32.add
                        i32.store offset=8 align=1
                        local.get 9
                        i32.const 120
                        i32.add
                        local.tee 5
                        i32.const 9
                        i32.store align=1
                        local.get 5
                        i64.load
                        local.set 7
                        local.get 5
                        i64.load offset=8
                        local.set 8
                        local.get 0
                        local.get 7
                        i64.store
                        local.get 0
                        local.get 8
                        i64.store offset=8
                        br 7 (;@3;)
                      end
                      local.get 1
                      local.get 1
                      i32.load offset=8 align=1
                      i32.const 1
                      i32.add
                      i32.store offset=8 align=1
                      local.get 9
                      i32.const 56
                      i32.add
                      local.tee 5
                      i32.const 5
                      i32.store align=1
                      local.get 5
                      i64.load
                      local.set 8
                      local.get 5
                      i64.load offset=8
                      local.set 7
                      local.get 0
                      local.get 8
                      i64.store
                      local.get 0
                      local.get 7
                      i64.store offset=8
                      br 6 (;@3;)
                    end
                    local.get 1
                    local.get 1
                    i32.load offset=8 align=1
                    i32.const 1
                    i32.add
                    i32.store offset=8 align=1
                    local.get 9
                    i32.const 24
                    i32.add
                    local.tee 5
                    i32.const 3
                    i32.store align=1
                    local.get 5
                    i64.load
                    local.set 7
                    local.get 5
                    i64.load offset=8
                    local.set 8
                    local.get 0
                    local.get 7
                    i64.store
                    local.get 0
                    local.get 8
                    i64.store offset=8
                    br 5 (;@3;)
                  else
                    local.get 6
                    i32.const 37
                    i32.eq
                    if ;; label = @9
                      local.get 1
                      local.get 1
                      i32.load offset=8 align=1
                      i32.const 1
                      i32.add
                      i32.store offset=8 align=1
                      local.get 9
                      i32.const 88
                      i32.add
                      local.tee 5
                      i32.const 7
                      i32.store align=1
                      local.get 5
                      i64.load
                      local.set 8
                      local.get 5
                      i64.load offset=8
                      local.set 7
                      local.get 0
                      local.get 8
                      i64.store
                      local.get 0
                      local.get 7
                      i64.store offset=8
                      br 6 (;@3;)
                    else
                      br 4 (;@5;)
                    end
                  end
                end
              end
            end
            block ;; label = @5
              local.get 6
              i32.const 48
              i32.ge_u
              if ;; label = @6
                local.get 6
                i32.const 57
                i32.le_u
                if ;; label = @7
                  local.get 0
                  local.get 1
                  call $_RNvMCsizV4RGem7kw_4exprNtB2_5Lexer6number
                  br 4 (;@3;)
                else
                  br 2 (;@5;)
                end
              else
                br 1 (;@5;)
              end
            end
            block ;; label = @5
              local.get 6
              i32.const 97
              i32.ge_u
              if ;; label = @6
                local.get 6
                i32.const 122
                i32.le_u
                if ;; label = @7
                  br 3 (;@4;)
                else
                  br 2 (;@5;)
                end
              else
                br 1 (;@5;)
              end
            end
            block ;; label = @5
              local.get 6
              i32.const 65
              i32.ge_u
              if ;; label = @6
                local.get 6
                i32.const 90
                i32.le_u
                if ;; label = @7
                  br 3 (;@4;)
                else
                  br 2 (;@5;)
                end
              else
                br 1 (;@5;)
              end
            end
            local.get 0
            i32.const -1
            i32.store align=1
            br 1 (;@3;)
          end
          local.get 0
          local.get 1
          call $_RNvMCsizV4RGem7kw_4exprNtB2_5Lexer5ident
          br 0 (;@3;)
        end
        local.get 9
        i32.const 192
        i32.add
        global.set $__stack_pointer
        return
      else
        local.get 9
        local.tee 5
        i32.const 12
        i32.store align=1
        local.get 5
        i64.load
        local.set 7
        local.get 5
        i64.load offset=8
        local.set 8
        local.get 0
        local.get 7
        i64.store
        local.get 0
        local.get 8
        i64.store offset=8
        local.get 9
        i32.const 192
        i32.add
        global.set $__stack_pointer
        return
      end
    end
    unreachable
  )

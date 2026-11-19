(func $_RNvCsfRjVcpQQBS9_5scene3dfs (;40;) (type 11) (param i32 i32) (result i64)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 144
    i32.sub
    local.tee 40
    global.set $__stack_pointer
    local.get 0
    i32.load offset=32 align=1
    local.tee 2
    if ;; label = @1
      i32.const 1065996
      local.tee 3
      i32.load align=1
      local.set 4
      local.get 3
      i32.load offset=4 align=1
      local.set 3
      local.get 40
      i32.const 0
      i32.add
      local.tee 5
      local.get 4
      i32.store align=1
      local.get 40
      i32.const 4
      i32.add
      local.tee 4
      local.get 3
      i32.store align=1
      local.get 40
      i32.const 8
      i32.add
      local.tee 3
      i32.const 0
      i32.store align=1
      local.get 5
      local.get 2
      i32.const 0
      call $_RNvMs1_NtCs45LlgTF0LZI_5alloc3vecINtB5_3VecbE6resizeCsfRjVcpQQBS9_5scene
      block ;; label = @2
        i32.const 1065988
        local.tee 2
        i32.load align=1
        local.set 6
        local.get 2
        i32.load offset=4 align=1
        local.set 2
        local.get 40
        i32.const 12
        i32.add
        local.tee 7
        local.get 6
        i32.store align=1
        local.get 40
        i32.const 16
        i32.add
        local.tee 6
        local.get 2
        i32.store align=1
        local.get 40
        i32.const 20
        i32.add
        local.tee 2
        i32.const 0
        i32.store align=1
        local.get 7
        i32.load align=1
        local.set 8
        local.get 40
        i32.const 32
        i32.add
        local.get 8
        i32.store align=1
        local.get 8
        i32.const 0
        i32.eq
        if ;; label = @3
          local.get 7
          call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCsfRjVcpQQBS9_5scene
          br 1 (;@2;)
        else
          br 1 (;@2;)
        end
      end
      local.get 6
      i32.load align=1
      local.set 8
      local.get 40
      i32.const 36
      i32.add
      local.get 8
      i32.store align=1
      local.get 8
      local.get 1
      i32.store align=1
      local.get 2
      i32.const 1
      i32.store align=1
      local.get 40
      i32.const 40
      i32.add
      local.set 8
      local.get 40
      i32.const 44
      i32.add
      local.set 1
      local.get 40
      i32.const 28
      i32.add
      local.set 9
      local.get 40
      i32.const 24
      i32.add
      local.set 10
      local.get 40
      i32.const 56
      i32.add
      local.set 11
      local.get 40
      i32.const 48
      i32.add
      local.set 12
      local.get 40
      i32.const 52
      i32.add
      local.set 13
      local.get 40
      i32.const 76
      i32.add
      local.set 14
      local.get 40
      i32.const 68
      i32.add
      local.set 15
      local.get 40
      i32.const 72
      i32.add
      local.set 16
      local.get 40
      i32.const 60
      i32.add
      local.set 17
      local.get 40
      i32.const 64
      i32.add
      local.set 18
      local.get 40
      i32.const 88
      i32.add
      local.set 19
      local.get 40
      i32.const 80
      i32.add
      local.set 20
      local.get 40
      i32.const 84
      i32.add
      local.set 21
      i64.const 0
      local.set 22
      loop ;; label = @2
        local.get 2
        i32.load align=1
        local.tee 23
        if ;; label = @3
          local.get 2
          local.get 23
          i32.const 1
          i32.sub
          i32.store align=1
          local.get 8
          local.get 7
          i32.load align=1
          i32.store align=1
          local.get 6
          i32.load align=1
          local.set 23
          local.get 1
          local.get 23
          i32.store align=1
          local.get 9
          local.get 23
          local.get 2
          i32.load align=1
          i32.const 2
          i32.shl
          i32.add
          i32.load align=1
          i32.store align=1
          local.get 10
          i32.const 1
          i32.store align=1
          local.get 9
          i32.load align=1
          local.set 23
          local.get 4
          i32.load align=1
          local.set 24
          local.get 11
          local.get 24
          i32.store align=1
          local.get 3
          i32.load align=1
          local.set 25
          local.get 12
          local.get 24
          i32.store align=1
          local.get 13
          local.get 25
          i32.store align=1
          local.get 23
          local.get 25
          i32.lt_u
          if ;; label = @4
            local.get 12
            i32.load align=1
            local.get 23
            i32.add
            i32.load8_u
            i32.const 255
            i32.and
            if ;; label = @5
              br 3 (;@2;)
            else
              local.get 4
              i32.load align=1
              local.set 25
              local.get 14
              local.get 25
              i32.store align=1
              local.get 3
              i32.load align=1
              local.set 24
              local.get 15
              local.get 25
              i32.store align=1
              local.get 16
              local.get 24
              i32.store align=1
              local.get 17
              local.get 15
              i32.load align=1
              i32.store align=1
              local.get 18
              local.get 24
              i32.store align=1
              local.get 23
              local.get 24
              i32.lt_u
              if ;; label = @6
                local.get 17
                i32.load align=1
                local.get 23
                i32.add
                i32.const 1
                i32.store8
                local.get 0
                i32.load offset=28 align=1
                local.set 24
                local.get 19
                local.get 24
                i32.store align=1
                local.get 0
                i32.load offset=32 align=1
                local.set 25
                local.get 20
                local.get 24
                i32.store align=1
                local.get 21
                local.get 25
                i32.store align=1
                local.get 23
                local.get 25
                i32.lt_u
                if ;; label = @7
                  local.get 23
                  i32.const 12
                  i32.mul
                  local.set 25
                  local.get 40
                  i32.const 100
                  i32.add
                  local.set 24
                  local.get 40
                  i32.const 92
                  i32.add
                  local.set 26
                  local.get 40
                  i32.const 96
                  i32.add
                  local.set 27
                  local.get 40
                  i32.const 112
                  i32.add
                  local.set 28
                  local.get 40
                  i32.const 104
                  i32.add
                  local.set 29
                  local.get 40
                  i32.const 108
                  i32.add
                  local.set 30
                  local.get 40
                  i32.const 124
                  i32.add
                  local.set 31
                  local.get 40
                  i32.const 116
                  i32.add
                  local.set 32
                  local.get 40
                  i32.const 120
                  i32.add
                  local.set 33
                  local.get 40
                  i32.const 128
                  i32.add
                  local.set 34
                  local.get 40
                  i32.const 132
                  i32.add
                  local.set 35
                  local.get 20
                  i32.load align=1
                  local.get 25
                  i32.add
                  i32.load offset=8 align=1
                  local.set 36
                  loop ;; label = @8
                    local.get 36
                    if ;; label = @9
                      local.get 0
                      i32.load offset=28 align=1
                      local.set 37
                      local.get 24
                      local.get 37
                      i32.store align=1
                      local.get 0
                      i32.load offset=32 align=1
                      local.set 38
                      local.get 26
                      local.get 37
                      i32.store align=1
                      local.get 27
                      local.get 38
                      i32.store align=1
                      local.get 23
                      local.get 38
                      i32.lt_u
                      if ;; label = @10
                        local.get 26
                        i32.load align=1
                        local.get 25
                        i32.add
                        local.tee 38
                        i32.load offset=4 align=1
                        local.set 37
                        local.get 28
                        local.get 37
                        i32.store align=1
                        local.get 38
                        i32.load offset=8 align=1
                        local.set 38
                        local.get 29
                        local.get 37
                        i32.store align=1
                        local.get 30
                        local.get 38
                        i32.store align=1
                        local.get 36
                        i32.const 1
                        i32.sub
                        local.tee 36
                        local.get 38
                        i32.lt_u
                        if ;; label = @11
                          local.get 29
                          i32.load align=1
                          local.get 36
                          i32.const 2
                          i32.shl
                          i32.add
                          i32.load align=1
                          local.set 38
                          local.get 4
                          i32.load align=1
                          local.set 37
                          local.get 31
                          local.get 37
                          i32.store align=1
                          local.get 3
                          i32.load align=1
                          local.set 39
                          local.get 32
                          local.get 37
                          i32.store align=1
                          local.get 33
                          local.get 39
                          i32.store align=1
                          local.get 38
                          local.get 39
                          i32.lt_u
                          if ;; label = @12
                            block ;; label = @13
                              local.get 32
                              i32.load align=1
                              local.get 38
                              i32.add
                              i32.load8_u
                              i32.const 255
                              i32.and
                              if ;; label = @14
                                br 1 (;@13;)
                              else
                                block ;; label = @15
                                  local.get 2
                                  i32.load align=1
                                  local.set 39
                                  local.get 7
                                  i32.load align=1
                                  local.set 37
                                  local.get 34
                                  local.get 37
                                  i32.store align=1
                                  local.get 39
                                  local.get 37
                                  i32.eq
                                  if ;; label = @16
                                    local.get 7
                                    call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCsfRjVcpQQBS9_5scene
                                    br 1 (;@15;)
                                  else
                                    br 1 (;@15;)
                                  end
                                end
                                local.get 6
                                i32.load align=1
                                local.set 37
                                local.get 35
                                local.get 37
                                i32.store align=1
                                local.get 37
                                local.get 39
                                i32.const 2
                                i32.shl
                                i32.add
                                local.get 38
                                i32.store align=1
                                local.get 2
                                local.get 39
                                i32.const 1
                                i32.add
                                i32.store align=1
                                br 1 (;@13;)
                              end
                            end
                            br 4 (;@8;)
                          else
                            local.get 38
                            local.get 39
                            i32.const 1065784
                            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                            unreachable
                          end
                        else
                          local.get 36
                          local.get 38
                          i32.const 1065908
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        local.get 23
                        local.get 38
                        i32.const 1065956
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 23
                      i64.extend_i32_u
                      i64.const -7046029254386353131
                      i64.mul
                      local.get 22
                      i64.xor
                      i64.const 1099511628211
                      i64.mul
                      i32.const 7
                      i64.extend_i32_u
                      i64.rotl
                      local.set 22
                      br 7 (;@2;)
                    end
                  end
                else
                  local.get 23
                  local.get 25
                  i32.const 1065924
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 23
                local.get 24
                i32.const 1065972
                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                unreachable
              end
            end
          else
            local.get 23
            local.get 25
            i32.const 1065940
            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
            unreachable
          end
        else
          local.get 7
          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VecjEECsfRjVcpQQBS9_5scene
          local.get 5
          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VecbEECsfRjVcpQQBS9_5scene
          local.get 22
          local.get 40
          i32.const 144
          i32.add
          global.set $__stack_pointer
          return
        end
      end
    else
      i64.const 0
      local.get 40
      i32.const 144
      i32.add
      global.set $__stack_pointer
      return
    end
    unreachable
  )

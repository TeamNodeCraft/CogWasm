(func $_RNvCs9103VBb1EJO_6sha2564body (;4;) (type 6) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 176
    i32.sub
    local.tee 17
    global.set $__stack_pointer
    local.get 17
    i32.const 172
    i32.add
    i32.const 0
    i32.store8
    i32.const 1065332
    local.set 1
    local.get 17
    i32.const 76
    i32.add
    local.tee 2
    i32.const 65608
    i32.const 0
    local.get 1
    i32.load align=1
    local.get 1
    i32.load offset=4 align=1
    call $_RNvMs8_NtCs45LlgTF0LZI_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs9103VBb1EJO_6sha256
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 2
          i32.load align=1
          br_table 1 (;@2;) 2 (;@1;) 0 (;@3;)
        end
        unreachable
      end
      local.get 17
      i32.const 80
      i32.add
      i32.load align=1
      local.set 3
      local.get 17
      i32.const 84
      i32.add
      i32.load align=1
      local.set 1
      local.get 17
      i32.const 68
      i32.add
      local.tee 2
      local.get 3
      i32.store align=1
      local.get 17
      i32.const 72
      i32.add
      local.tee 3
      local.get 1
      i32.store align=1
      local.get 17
      i32.const 96
      i32.add
      local.get 2
      i32.load align=1
      i32.store align=1
      local.get 2
      i32.load align=1
      local.set 2
      local.get 3
      i32.load align=1
      local.set 3
      local.get 17
      i32.const 60
      i32.add
      local.tee 1
      local.get 2
      i32.store align=1
      local.get 17
      i32.const 64
      i32.add
      local.get 3
      i32.store align=1
      local.get 17
      i32.const 0
      i32.add
      local.tee 2
      local.get 1
      i32.load align=1
      i32.store align=1
      local.get 17
      i32.const 4
      i32.add
      local.tee 1
      local.get 3
      i32.store align=1
      local.get 17
      i32.const 8
      i32.add
      local.tee 3
      i32.const 0
      i32.store align=1
      local.get 0
      i64.const -7046029254386353131
      i64.xor
      i64.const -7046029254386353131
      i64.add
      local.tee 0
      local.get 0
      i32.const 30
      i64.extend_i32_u
      i64.shr_u
      i64.xor
      i64.const -4658895280553007687
      i64.mul
      local.tee 0
      local.get 0
      i32.const 27
      i64.extend_i32_u
      i64.shr_u
      i64.xor
      i64.const -7723592293110705685
      i64.mul
      local.set 0
      local.get 17
      i32.const 12
      i32.add
      local.set 4
      i32.const 0
      local.get 0
      local.get 0
      i32.const 31
      i64.extend_i32_u
      i64.shr_u
      i64.xor
      local.set 0
      local.set 5
      loop ;; label = @2
        local.get 5
        i32.const 65536
        i32.lt_u
        if ;; label = @3
          local.get 0
          i64.const 4354685564936845354
          i64.add
          local.tee 11
          local.get 11
          i32.const 30
          i64.extend_i32_u
          i64.shr_u
          i64.xor
          i64.const -4658895280553007687
          i64.mul
          local.tee 11
          local.get 11
          i32.const 27
          i64.extend_i32_u
          i64.shr_u
          i64.xor
          i64.const -7723592293110705685
          i64.mul
          local.set 11
          local.get 4
          local.get 11
          local.get 11
          i32.const 31
          i64.extend_i32_u
          i64.shr_u
          i64.xor
          local.get 5
          i64.extend_i32_u
          i64.const -7046029254386353131
          i64.mul
          i64.xor
          i64.store align=1
          local.get 17
          i32.const 100
          i32.add
          local.set 7
          local.get 17
          i32.const 104
          i32.add
          local.set 6
          i32.const 0
          local.set 16
          loop ;; label = @4
            local.get 16
            i32.const 8
            i32.lt_u
            local.tee 15
            if ;; label = @5
              local.get 15
              if ;; label = @6
                block ;; label = @7
                  local.get 4
                  local.get 16
                  i32.add
                  i32.load8_u
                  i32.const 255
                  i32.and
                  local.set 15
                  local.get 3
                  i32.load align=1
                  local.set 8
                  local.get 2
                  i32.load align=1
                  local.set 14
                  local.get 7
                  local.get 14
                  i32.store align=1
                  local.get 8
                  local.get 14
                  i32.eq
                  if ;; label = @8
                    local.get 2
                    call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                    br 1 (;@7;)
                  else
                    br 1 (;@7;)
                  end
                end
                local.get 1
                i32.load align=1
                local.set 14
                local.get 6
                local.get 14
                i32.store align=1
                local.get 14
                local.get 8
                i32.add
                local.get 15
                i32.store8
                local.get 3
                local.get 8
                i32.const 1
                i32.add
                i32.store align=1
                local.get 16
                i32.const 1
                i32.add
                local.set 16
                br 2 (;@4;)
              else
                local.get 16
                i32.const 8
                i32.const 1065316
                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                unreachable
              end
            else
              local.get 5
              i32.const 8
              i32.add
              local.get 0
              i64.const -7046029254386353131
              i64.add
              local.set 0
              local.set 5
              br 3 (;@2;)
            end
          end
        else
          block ;; label = @4
            local.get 3
            i32.load align=1
            local.set 5
            local.get 2
            i32.load align=1
            local.set 4
            local.get 17
            i32.const 108
            i32.add
            local.get 4
            i32.store align=1
            local.get 5
            local.get 4
            i32.eq
            if ;; label = @5
              local.get 2
              call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
              br 1 (;@4;)
            else
              br 1 (;@4;)
            end
          end
          local.get 1
          i32.load align=1
          local.set 4
          local.get 17
          i32.const 112
          i32.add
          local.get 4
          i32.store align=1
          local.get 4
          local.get 5
          i32.add
          i32.const 128
          i32.store8
          local.get 3
          local.get 5
          i32.const 1
          i32.add
          i32.store align=1
          local.get 17
          i32.const 116
          i32.add
          local.set 5
          local.get 17
          i32.const 120
          i32.add
          local.set 4
          loop ;; label = @4
            local.get 3
            i32.load align=1
            local.tee 6
            i32.const 63
            i32.and
            i32.const 56
            i32.eq
            if ;; label = @5
              local.get 17
              i32.const 20
              i32.add
              local.tee 6
              i64.const 8796093022208
              i64.store align=1
              local.get 17
              i32.const 124
              i32.add
              local.set 7
              local.get 17
              i32.const 128
              i32.add
              local.set 5
              i32.const 0
              local.set 4
              loop ;; label = @6
                local.get 4
                i32.const 8
                i32.lt_u
                local.tee 8
                if ;; label = @7
                  local.get 8
                  if ;; label = @8
                    block ;; label = @9
                      local.get 6
                      local.get 4
                      i32.add
                      i32.load8_u
                      i32.const 255
                      i32.and
                      local.set 8
                      local.get 3
                      i32.load align=1
                      local.set 16
                      local.get 2
                      i32.load align=1
                      local.set 15
                      local.get 7
                      local.get 15
                      i32.store align=1
                      local.get 16
                      local.get 15
                      i32.eq
                      if ;; label = @10
                        local.get 2
                        call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                        br 1 (;@9;)
                      else
                        br 1 (;@9;)
                      end
                    end
                    local.get 1
                    i32.load align=1
                    local.set 15
                    local.get 5
                    local.get 15
                    i32.store align=1
                    local.get 15
                    local.get 16
                    i32.add
                    local.get 8
                    i32.store8
                    local.get 3
                    local.get 16
                    i32.const 1
                    i32.add
                    i32.store align=1
                    local.get 4
                    i32.const 1
                    i32.add
                    local.set 4
                    br 2 (;@6;)
                  else
                    local.get 4
                    i32.const 8
                    i32.const 1065300
                    call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                    unreachable
                  end
                else
                  i32.const 1065268
                  local.tee 8
                  i64.load align=1
                  local.set 0
                  local.get 8
                  i64.load offset=8 align=1
                  local.set 9
                  local.get 8
                  i64.load offset=16 align=1
                  local.set 10
                  local.get 8
                  i64.load offset=24 align=1
                  local.set 11
                  local.get 17
                  i32.const 28
                  i32.add
                  local.tee 8
                  local.get 0
                  i64.store align=1
                  local.get 8
                  local.get 9
                  i64.store offset=8 align=1
                  local.get 8
                  local.get 10
                  i64.store offset=16 align=1
                  local.get 8
                  local.get 11
                  i64.store offset=24 align=1
                  local.get 17
                  i32.const 140
                  i32.add
                  local.set 7
                  local.get 17
                  i32.const 132
                  i32.add
                  local.set 6
                  local.get 17
                  i32.const 136
                  i32.add
                  local.set 5
                  local.get 17
                  i32.const 144
                  i32.add
                  local.set 4
                  local.get 17
                  i32.const 148
                  i32.add
                  local.set 12
                  i32.const 0
                  local.set 13
                  loop ;; label = @8
                    local.get 13
                    local.get 3
                    i32.load align=1
                    i32.lt_u
                    if ;; label = @9
                      block ;; label = @10
                        local.get 1
                        i32.load align=1
                        local.set 14
                        local.get 7
                        local.get 14
                        i32.store align=1
                        local.get 3
                        i32.load align=1
                        local.set 15
                        local.get 6
                        local.get 14
                        i32.store align=1
                        local.get 5
                        local.get 15
                        i32.store align=1
                        local.get 13
                        i32.const 64
                        i32.add
                        local.tee 14
                        local.get 13
                        i32.lt_u
                        if ;; label = @11
                          local.get 13
                          local.set 16
                          br 1 (;@10;)
                        else
                          local.get 14
                          local.get 15
                          i32.le_u
                          if ;; label = @12
                            local.get 4
                            local.get 6
                            i32.load align=1
                            local.get 13
                            i32.add
                            i32.store align=1
                            local.get 12
                            i32.const 64
                            i32.store align=1
                            local.get 8
                            local.get 4
                            i32.load align=1
                            i32.const 64
                            call $_RNvCs9103VBb1EJO_6sha2568compress
                            local.get 14
                            local.set 13
                            br 4 (;@8;)
                          else
                            local.get 13
                            local.set 16
                            br 2 (;@10;)
                          end
                        end
                      end
                      local.get 16
                      local.get 14
                      local.get 15
                      i32.const 1065236
                      call $_RNvNtNtCsk7Mwp4gGzxp_4core5slice5index16slice_index_fail
                      unreachable
                    else
                      local.get 17
                      i32.const 152
                      i32.add
                      local.tee 13
                      i64.const 65536
                      i64.store align=1
                      local.get 13
                      i32.const 8
                      call $_RNvCs9103VBb1EJO_6sha25610emit_bytes
                      local.get 17
                      i32.const 160
                      i32.add
                      local.set 13
                      i32.const 0
                      i64.const 0
                      local.set 11
                      local.set 12
                      loop ;; label = @10
                        local.get 12
                        i32.const 8
                        i32.lt_u
                        local.tee 7
                        if ;; label = @11
                          local.get 7
                          if ;; label = @12
                            local.get 8
                            local.get 12
                            i32.const 2
                            i32.shl
                            i32.add
                            local.set 6
                            local.get 13
                            local.get 6
                            i32.load align=1
                            i32.store align=1
                            local.get 13
                            i32.const 4
                            call $_RNvCs9103VBb1EJO_6sha25610emit_bytes
                            local.get 7
                            if ;; label = @13
                              local.get 12
                              i32.const 1
                              i32.add
                              local.get 11
                              i32.const 9
                              i64.extend_i32_u
                              i64.rotl
                              local.get 6
                              i32.load align=1
                              i64.extend_i32_u
                              i64.xor
                              local.set 11
                              local.set 12
                              br 3 (;@10;)
                            else
                              local.get 12
                              i32.const 8
                              i32.const 1065112
                              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                              unreachable
                            end
                          else
                            local.get 12
                            i32.const 8
                            i32.const 1065252
                            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                            unreachable
                          end
                        else
                          local.get 17
                          i32.const 164
                          i32.add
                          local.tee 13
                          local.get 11
                          i64.store align=1
                          local.get 13
                          i32.const 8
                          call $_RNvCs9103VBb1EJO_6sha25610emit_bytes
                          local.get 2
                          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VechEECs9103VBb1EJO_6sha256
                          local.get 11
                          local.get 17
                          i32.const 176
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
              block ;; label = @6
                local.get 2
                i32.load align=1
                local.set 7
                local.get 5
                local.get 7
                i32.store align=1
                local.get 6
                local.get 7
                i32.eq
                if ;; label = @7
                  local.get 2
                  call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
                  br 1 (;@6;)
                else
                  br 1 (;@6;)
                end
              end
              local.get 1
              i32.load align=1
              local.set 7
              local.get 4
              local.get 7
              i32.store align=1
              local.get 7
              local.get 6
              i32.add
              i32.const 0
              i32.store8
              local.get 3
              local.get 6
              i32.const 1
              i32.add
              i32.store align=1
              br 1 (;@4;)
            end
          end
        end
      end
    end
    local.get 17
    i32.const 80
    i32.add
    i32.load align=1
    local.set 2
    local.get 17
    i32.const 84
    i32.add
    i32.load align=1
    local.set 1
    local.get 17
    i32.const 88
    i32.add
    local.tee 3
    local.get 2
    i32.store align=1
    local.get 17
    i32.const 92
    i32.add
    local.get 1
    i32.store align=1
    local.get 3
    i32.load align=1
    local.get 1
    call $_RNvNtCs45LlgTF0LZI_5alloc7raw_vec12handle_error
    unreachable
    unreachable
  )

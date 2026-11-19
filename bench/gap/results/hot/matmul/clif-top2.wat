(func $_RNvCshS8UloB9tPT_6matmul4body (;9;) (type 6) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 f64 f64 f64 f64 i32 i32 i64 i64 i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 336
    i32.sub
    local.tee 25
    global.set $__stack_pointer
    local.get 25
    i32.const 328
    i32.add
    i32.const 0
    i32.store8
    i32.const 1065268
    local.set 1
    local.get 25
    i32.const 52
    i32.add
    local.tee 2
    i32.const 16384
    i32.const 0
    local.get 1
    i32.load align=1
    local.get 1
    i32.load offset=4 align=1
    call $_RNvMs8_NtCs45LlgTF0LZI_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshS8UloB9tPT_6matmul
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 2
            i32.load align=1
            br_table 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          br 2 (;@1;)
        end
        local.get 25
        i32.const 56
        i32.add
        i32.load align=1
        local.set 3
        local.get 25
        i32.const 60
        i32.add
        i32.load align=1
        local.set 1
        local.get 25
        i32.const 44
        i32.add
        local.tee 2
        local.get 3
        i32.store align=1
        local.get 25
        i32.const 48
        i32.add
        local.tee 3
        local.get 1
        i32.store align=1
        local.get 25
        i32.const 72
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
        local.get 25
        i32.const 36
        i32.add
        local.tee 1
        local.get 2
        i32.store align=1
        local.get 25
        i32.const 40
        i32.add
        local.get 3
        i32.store align=1
        local.get 25
        i32.const 0
        i32.add
        local.tee 2
        local.get 1
        i32.load align=1
        i32.store align=1
        local.get 25
        i32.const 4
        i32.add
        local.tee 1
        local.get 3
        i32.store align=1
        local.get 25
        i32.const 8
        i32.add
        local.tee 3
        i32.const 0
        i32.store align=1
        local.get 25
        i32.const 329
        i32.add
        i32.const 0
        i32.store8
        i32.const 1065268
        local.set 4
        local.get 25
        i32.const 92
        i32.add
        local.tee 5
        i32.const 16384
        i32.const 0
        local.get 4
        i32.load align=1
        local.get 4
        i32.load offset=4 align=1
        call $_RNvMs8_NtCs45LlgTF0LZI_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshS8UloB9tPT_6matmul
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 5
              i32.load align=1
              br_table 1 (;@4;) 2 (;@3;) 0 (;@5;)
            end
            br 3 (;@1;)
          end
          local.get 25
          i32.const 96
          i32.add
          i32.load align=1
          local.set 4
          local.get 25
          i32.const 100
          i32.add
          i32.load align=1
          local.set 5
          local.get 25
          i32.const 84
          i32.add
          local.tee 6
          local.get 4
          i32.store align=1
          local.get 25
          i32.const 88
          i32.add
          local.tee 4
          local.get 5
          i32.store align=1
          local.get 25
          i32.const 112
          i32.add
          local.get 6
          i32.load align=1
          i32.store align=1
          local.get 6
          i32.load align=1
          local.set 6
          local.get 4
          i32.load align=1
          local.set 4
          local.get 25
          i32.const 76
          i32.add
          local.tee 5
          local.get 6
          i32.store align=1
          local.get 25
          i32.const 80
          i32.add
          local.get 4
          i32.store align=1
          local.get 25
          i32.const 12
          i32.add
          local.tee 6
          local.get 5
          i32.load align=1
          i32.store align=1
          local.get 25
          i32.const 16
          i32.add
          local.tee 5
          local.get 4
          i32.store align=1
          local.get 25
          i32.const 20
          i32.add
          local.tee 4
          i32.const 0
          i32.store align=1
          local.get 25
          i32.const 330
          i32.add
          i32.const 0
          i32.store8
          i32.const 1065268
          local.set 7
          local.get 25
          i32.const 132
          i32.add
          local.tee 8
          i32.const 16384
          i32.const 0
          local.get 7
          i32.load align=1
          local.get 7
          i32.load offset=4 align=1
          call $_RNvMs8_NtCs45LlgTF0LZI_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshS8UloB9tPT_6matmul
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.load align=1
                br_table 1 (;@5;) 2 (;@4;) 0 (;@6;)
              end
              br 4 (;@1;)
            end
            local.get 25
            i32.const 136
            i32.add
            i32.load align=1
            local.set 7
            local.get 25
            i32.const 140
            i32.add
            i32.load align=1
            local.set 8
            local.get 25
            i32.const 124
            i32.add
            local.tee 9
            local.get 7
            i32.store align=1
            local.get 25
            i32.const 128
            i32.add
            local.tee 7
            local.get 8
            i32.store align=1
            local.get 25
            i32.const 152
            i32.add
            local.get 9
            i32.load align=1
            i32.store align=1
            local.get 9
            i32.load align=1
            local.set 9
            local.get 7
            i32.load align=1
            local.set 7
            local.get 25
            i32.const 116
            i32.add
            local.tee 8
            local.get 9
            i32.store align=1
            local.get 25
            i32.const 120
            i32.add
            local.get 7
            i32.store align=1
            local.get 25
            i32.const 24
            i32.add
            local.tee 9
            local.get 8
            i32.load align=1
            i32.store align=1
            local.get 25
            i32.const 28
            i32.add
            local.tee 8
            local.get 7
            i32.store align=1
            local.get 25
            i32.const 32
            i32.add
            local.tee 7
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
            local.get 25
            i32.const 156
            i32.add
            local.set 10
            local.get 25
            i32.const 160
            i32.add
            local.set 11
            local.get 25
            i32.const 164
            i32.add
            local.set 12
            local.get 25
            i32.const 168
            i32.add
            local.set 13
            local.get 25
            i32.const 172
            i32.add
            local.set 14
            local.get 25
            i32.const 176
            i32.add
            local.set 15
            i32.const 0
            local.get 0
            local.get 0
            i32.const 31
            i64.extend_i32_u
            i64.shr_u
            i64.xor
            local.set 0
            local.set 16
            loop ;; label = @5
              local.get 16
              i32.const 16384
              i32.lt_u
              if ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.load align=1
                  local.set 21
                  local.get 2
                  i32.load align=1
                  local.set 22
                  local.get 10
                  local.get 22
                  i32.store align=1
                  local.get 21
                  local.get 22
                  i32.eq
                  if ;; label = @8
                    local.get 2
                    call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCshS8UloB9tPT_6matmul
                    br 1 (;@7;)
                  else
                    br 1 (;@7;)
                  end
                end
                block ;; label = @7
                  local.get 1
                  i32.load align=1
                  local.set 22
                  local.get 11
                  local.get 22
                  i32.store align=1
                  local.get 0
                  i64.const 4354685564936845354
                  i64.add
                  local.tee 23
                  local.get 23
                  i32.const 30
                  i64.extend_i32_u
                  i64.shr_u
                  i64.xor
                  i64.const -4658895280553007687
                  i64.mul
                  local.tee 24
                  local.get 24
                  i32.const 27
                  i64.extend_i32_u
                  i64.shr_u
                  i64.xor
                  i64.const -7723592293110705685
                  i64.mul
                  local.tee 24
                  local.get 24
                  i32.const 31
                  i64.extend_i32_u
                  i64.shr_u
                  i64.xor
                  local.set 24
                  local.get 22
                  local.get 21
                  i32.const 3
                  i32.shl
                  i32.add
                  local.get 24
                  local.get 24
                  local.set 26
                  i64.const 4835700860608086395
                  local.set 27
                  local.get 26
                  i64.const 4294967295
                  i64.and
                  local.get 27
                  i64.const 32
                  i64.shr_u
                  i64.mul
                  local.set 28
                  local.get 26
                  i64.const 32
                  i64.shr_u
                  local.get 27
                  i64.const 4294967295
                  i64.and
                  i64.mul
                  local.set 29
                  local.get 26
                  i64.const 32
                  i64.shr_u
                  local.get 27
                  i64.const 32
                  i64.shr_u
                  i64.mul
                  local.get 28
                  i64.const 32
                  i64.shr_u
                  i64.add
                  local.get 29
                  i64.const 32
                  i64.shr_u
                  i64.add
                  local.get 26
                  i64.const 4294967295
                  i64.and
                  local.get 27
                  i64.const 4294967295
                  i64.and
                  i64.mul
                  i64.const 32
                  i64.shr_u
                  local.get 28
                  i64.const 4294967295
                  i64.and
                  i64.add
                  local.get 29
                  i64.const 4294967295
                  i64.and
                  i64.add
                  i64.const 32
                  i64.shr_u
                  i64.add
                  i64.const 19
                  i64.shr_u
                  i64.const 2000001
                  i64.mul
                  i64.sub
                  i64.const 1000000
                  i64.sub
                  f64.convert_i64_s
                  f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
                  f64.mul
                  f64.store align=1
                  local.get 3
                  local.get 21
                  i32.const 1
                  i32.add
                  i32.store align=1
                  local.get 4
                  i32.load align=1
                  local.set 21
                  local.get 6
                  i32.load align=1
                  local.set 22
                  local.get 12
                  local.get 22
                  i32.store align=1
                  local.get 21
                  local.get 22
                  i32.eq
                  if ;; label = @8
                    local.get 6
                    call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCshS8UloB9tPT_6matmul
                    br 1 (;@7;)
                  else
                    br 1 (;@7;)
                  end
                end
                block ;; label = @7
                  local.get 5
                  i32.load align=1
                  local.set 22
                  local.get 13
                  local.get 22
                  i32.store align=1
                  local.get 0
                  i64.const -2691343689449507777
                  i64.add
                  local.tee 24
                  local.get 24
                  i32.const 30
                  i64.extend_i32_u
                  i64.shr_u
                  i64.xor
                  i64.const -4658895280553007687
                  i64.mul
                  local.tee 24
                  local.get 24
                  i32.const 27
                  i64.extend_i32_u
                  i64.shr_u
                  i64.xor
                  i64.const -7723592293110705685
                  i64.mul
                  local.tee 24
                  local.get 24
                  i32.const 31
                  i64.extend_i32_u
                  i64.shr_u
                  i64.xor
                  local.set 24
                  local.get 22
                  local.get 21
                  i32.const 3
                  i32.shl
                  i32.add
                  local.get 24
                  local.get 24
                  local.set 28
                  i64.const 4835700860608086395
                  local.set 27
                  local.get 28
                  i64.const 4294967295
                  i64.and
                  local.get 27
                  i64.const 32
                  i64.shr_u
                  i64.mul
                  local.set 26
                  local.get 28
                  i64.const 32
                  i64.shr_u
                  local.get 27
                  i64.const 4294967295
                  i64.and
                  i64.mul
                  local.set 29
                  local.get 28
                  i64.const 32
                  i64.shr_u
                  local.get 27
                  i64.const 32
                  i64.shr_u
                  i64.mul
                  local.get 26
                  i64.const 32
                  i64.shr_u
                  i64.add
                  local.get 29
                  i64.const 32
                  i64.shr_u
                  i64.add
                  local.get 28
                  i64.const 4294967295
                  i64.and
                  local.get 27
                  i64.const 4294967295
                  i64.and
                  i64.mul
                  i64.const 32
                  i64.shr_u
                  local.get 26
                  i64.const 4294967295
                  i64.and
                  i64.add
                  local.get 29
                  i64.const 4294967295
                  i64.and
                  i64.add
                  i64.const 32
                  i64.shr_u
                  i64.add
                  i64.const 19
                  i64.shr_u
                  i64.const 2000001
                  i64.mul
                  i64.sub
                  i64.const 1000000
                  i64.sub
                  f64.convert_i64_s
                  f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
                  f64.mul
                  f64.store align=1
                  local.get 4
                  local.get 21
                  i32.const 1
                  i32.add
                  i32.store align=1
                  local.get 7
                  i32.load align=1
                  local.set 21
                  local.get 9
                  i32.load align=1
                  local.set 22
                  local.get 14
                  local.get 22
                  i32.store align=1
                  local.get 21
                  local.get 22
                  i32.eq
                  if ;; label = @8
                    local.get 9
                    call $_RNvMs7_NtCs45LlgTF0LZI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCshS8UloB9tPT_6matmul
                    br 1 (;@7;)
                  else
                    br 1 (;@7;)
                  end
                end
                local.get 8
                i32.load align=1
                local.set 22
                local.get 15
                local.get 22
                i32.store align=1
                local.get 22
                local.get 21
                i32.const 3
                i32.shl
                i32.add
                f64.const 0x0p+0 (;=0;)
                f64.store align=1
                local.get 7
                local.get 21
                i32.const 1
                i32.add
                i32.store align=1
                local.get 16
                i32.const 1
                i32.add
                local.get 23
                local.set 0
                local.set 16
                br 1 (;@5;)
              else
                local.get 1
                i32.load align=1
                local.set 16
                local.get 25
                i32.const 188
                i32.add
                local.get 16
                i32.store align=1
                local.get 3
                i32.load align=1
                local.set 15
                local.get 25
                i32.const 180
                i32.add
                local.tee 14
                local.get 16
                i32.store align=1
                local.get 25
                i32.const 184
                i32.add
                local.get 15
                i32.store align=1
                local.get 14
                i32.load align=1
                local.set 16
                local.get 5
                i32.load align=1
                local.set 14
                local.get 25
                i32.const 200
                i32.add
                local.get 14
                i32.store align=1
                local.get 4
                i32.load align=1
                local.set 13
                local.get 25
                i32.const 192
                i32.add
                local.tee 12
                local.get 14
                i32.store align=1
                local.get 25
                i32.const 196
                i32.add
                local.get 13
                i32.store align=1
                local.get 12
                i32.load align=1
                local.set 14
                local.get 8
                i32.load align=1
                local.set 12
                local.get 25
                i32.const 212
                i32.add
                local.get 12
                i32.store align=1
                local.get 7
                i32.load align=1
                local.set 11
                local.get 25
                i32.const 204
                i32.add
                local.tee 10
                local.get 12
                i32.store align=1
                local.get 25
                i32.const 208
                i32.add
                local.get 11
                i32.store align=1
                local.get 16
                local.get 15
                local.get 14
                local.get 13
                local.get 10
                i32.load align=1
                local.get 11
                i32.const 128
                call $_RNvCshS8UloB9tPT_6matmul6matmul
                i32.const 0
                f64.const 0x0p+0 (;=0;)
                f64.const 0x0p+0 (;=0;)
                f64.const 0x0p+0 (;=0;)
                i64.const 0
                local.set 0
                local.set 19
                local.set 18
                local.set 17
                local.set 16
                loop ;; label = @7
                  local.get 16
                  i32.const 128
                  i32.lt_u
                  if ;; label = @8
                    local.get 25
                    i32.const 224
                    i32.add
                    local.set 15
                    local.get 25
                    i32.const 216
                    i32.add
                    local.set 14
                    local.get 25
                    i32.const 220
                    i32.add
                    local.set 13
                    i32.const 0
                    local.get 16
                    local.set 12
                    local.set 16
                    loop ;; label = @9
                      local.get 16
                      i32.const 128
                      i32.lt_u
                      if ;; label = @10
                        local.get 8
                        i32.load align=1
                        local.set 11
                        local.get 15
                        local.get 11
                        i32.store align=1
                        local.get 7
                        i32.load align=1
                        local.set 10
                        local.get 14
                        local.get 11
                        i32.store align=1
                        local.get 13
                        local.get 10
                        i32.store align=1
                        local.get 12
                        i32.const 7
                        i32.shl
                        local.get 16
                        i32.add
                        local.tee 11
                        local.get 10
                        i32.lt_u
                        if ;; label = @11
                          block ;; label = @12
                            local.get 14
                            i32.load align=1
                            local.get 11
                            i32.const 3
                            i32.shl
                            i32.add
                            f64.load align=1
                            local.set 20
                            local.get 12
                            local.get 16
                            i32.eq
                            if ;; label = @13
                              local.get 18
                              local.get 20
                              f64.add
                              local.set 18
                              br 1 (;@12;)
                            else
                              br 1 (;@12;)
                            end
                          end
                          local.get 16
                          i32.const 1
                          i32.add
                          local.get 17
                          local.get 20
                          f64.add
                          local.get 19
                          local.get 20
                          local.get 11
                          f64.convert_i32_u
                          f64.const 0x1.a36e2eb1c432dp-14 (;=0.0001;)
                          f64.mul
                          f64.mul
                          f64.add
                          local.get 0
                          i32.const 3
                          i64.extend_i32_u
                          i64.rotl
                          local.get 20
                          i64.reinterpret_f64
                          i64.xor
                          local.set 0
                          local.set 19
                          local.set 17
                          local.set 16
                          br 2 (;@9;)
                        else
                          local.get 11
                          local.get 10
                          i32.const 1065276
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        local.get 12
                        i32.const 1
                        i32.add
                        local.set 16
                        br 3 (;@7;)
                      end
                    end
                  else
                    local.get 25
                    i32.const 228
                    i32.add
                    local.tee 16
                    i64.const 128
                    i64.store align=1
                    local.get 16
                    i32.const 8
                    call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                    local.get 25
                    i32.const 236
                    i32.add
                    local.tee 16
                    local.get 17
                    f64.store align=1
                    local.get 16
                    i32.const 8
                    call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                    local.get 25
                    i32.const 244
                    i32.add
                    local.tee 16
                    local.get 18
                    f64.store align=1
                    local.get 16
                    i32.const 8
                    call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                    local.get 25
                    i32.const 252
                    i32.add
                    local.tee 16
                    local.get 19
                    f64.store align=1
                    local.get 16
                    i32.const 8
                    call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                    local.get 25
                    i32.const 260
                    i32.add
                    local.tee 16
                    local.get 0
                    i64.store align=1
                    local.get 16
                    i32.const 8
                    call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                    local.get 8
                    i32.load align=1
                    local.set 16
                    local.get 25
                    i32.const 276
                    i32.add
                    local.get 16
                    i32.store align=1
                    local.get 7
                    i32.load align=1
                    local.set 15
                    local.get 25
                    i32.const 268
                    i32.add
                    local.tee 14
                    local.get 16
                    i32.store align=1
                    local.get 25
                    i32.const 272
                    i32.add
                    local.get 15
                    i32.store align=1
                    local.get 15
                    if ;; label = @9
                      local.get 25
                      i32.const 280
                      i32.add
                      local.tee 16
                      local.get 14
                      i32.load align=1
                      f64.load align=1
                      f64.store align=1
                      local.get 16
                      i32.const 8
                      call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                      local.get 8
                      i32.load align=1
                      local.set 16
                      local.get 25
                      i32.const 296
                      i32.add
                      local.get 16
                      i32.store align=1
                      local.get 7
                      i32.load align=1
                      local.set 15
                      local.get 25
                      i32.const 288
                      i32.add
                      local.tee 14
                      local.get 16
                      i32.store align=1
                      local.get 25
                      i32.const 292
                      i32.add
                      local.get 15
                      i32.store align=1
                      local.get 15
                      i32.const 8256
                      i32.gt_u
                      if ;; label = @10
                        local.get 25
                        i32.const 300
                        i32.add
                        local.tee 16
                        local.get 14
                        i32.load align=1
                        i32.const 66048
                        i32.add
                        f64.load align=1
                        f64.store align=1
                        local.get 16
                        i32.const 8
                        call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                        local.get 8
                        i32.load align=1
                        local.set 16
                        local.get 25
                        i32.const 316
                        i32.add
                        local.get 16
                        i32.store align=1
                        local.get 7
                        i32.load align=1
                        local.set 15
                        local.get 25
                        i32.const 308
                        i32.add
                        local.tee 14
                        local.get 16
                        i32.store align=1
                        local.get 25
                        i32.const 312
                        i32.add
                        local.get 15
                        i32.store align=1
                        local.get 15
                        i32.const 16383
                        i32.gt_u
                        if ;; label = @11
                          local.get 25
                          i32.const 320
                          i32.add
                          local.tee 16
                          local.get 14
                          i32.load align=1
                          i32.const 131064
                          i32.add
                          f64.load align=1
                          f64.store align=1
                          local.get 16
                          i32.const 8
                          call $_RNvCshS8UloB9tPT_6matmul10emit_bytes
                          local.get 9
                          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VecdEECshS8UloB9tPT_6matmul
                          local.get 6
                          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VecdEECshS8UloB9tPT_6matmul
                          local.get 2
                          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VecdEECshS8UloB9tPT_6matmul
                          local.get 0
                          local.get 17
                          i64.reinterpret_f64
                          i32.const 11
                          i64.extend_i32_u
                          i64.rotl
                          i64.xor
                          local.get 18
                          i64.reinterpret_f64
                          i32.const 31
                          i64.extend_i32_u
                          i64.rotl
                          i64.xor
                          local.get 19
                          i64.reinterpret_f64
                          i32.const 47
                          i64.extend_i32_u
                          i64.rotl
                          i64.xor
                          local.get 25
                          i32.const 336
                          i32.add
                          global.set $__stack_pointer
                          return
                        else
                          i32.const 16383
                          local.get 15
                          i32.const 1065112
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        i32.const 8256
                        local.get 15
                        i32.const 1065236
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      i32.const 0
                      local.get 15
                      i32.const 1065252
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  end
                end
              end
            end
          end
          local.get 25
          i32.const 136
          i32.add
          i32.load align=1
          local.set 8
          local.get 25
          i32.const 140
          i32.add
          i32.load align=1
          local.set 7
          local.get 25
          i32.const 144
          i32.add
          local.tee 6
          local.get 8
          i32.store align=1
          local.get 25
          i32.const 148
          i32.add
          local.get 7
          i32.store align=1
          local.get 6
          i32.load align=1
          local.get 7
          call $_RNvNtCs45LlgTF0LZI_5alloc7raw_vec12handle_error
          unreachable
        end
        local.get 25
        i32.const 96
        i32.add
        i32.load align=1
        local.set 5
        local.get 25
        i32.const 100
        i32.add
        i32.load align=1
        local.set 4
        local.get 25
        i32.const 104
        i32.add
        local.tee 3
        local.get 5
        i32.store align=1
        local.get 25
        i32.const 108
        i32.add
        local.get 4
        i32.store align=1
        local.get 3
        i32.load align=1
        local.get 4
        call $_RNvNtCs45LlgTF0LZI_5alloc7raw_vec12handle_error
        unreachable
      end
      local.get 25
      i32.const 56
      i32.add
      i32.load align=1
      local.set 2
      local.get 25
      i32.const 60
      i32.add
      i32.load align=1
      local.set 1
      local.get 25
      i32.const 64
      i32.add
      local.tee 3
      local.get 2
      i32.store align=1
      local.get 25
      i32.const 68
      i32.add
      local.get 1
      i32.store align=1
      local.get 3
      i32.load align=1
      local.get 1
      call $_RNvNtCs45LlgTF0LZI_5alloc7raw_vec12handle_error
      unreachable
    end
    unreachable
    unreachable
  )

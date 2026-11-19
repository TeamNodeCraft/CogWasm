# mandelbrot::body vs LLVM's inlined `run`

LLVM -O2 inlined the whole program into `run`; this is the hot function that survives as a separate body in our module.

## Ours (611,269,324 instructions, 100.0%)

```wat
(func $_RNvCsgUqo56RTmP6_10mandelbrot4body (;3;) (type 3) (param i64) (result i64)
    (local i32 i32 i64 i64 i64 i32 i32 f64 i64 i64 f64 f64 f64 f64 f64 f64 i32 i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 192
    i32.sub
    local.tee 18
    global.set $__stack_pointer
    local.get 18
    local.set 1
    i32.const 0
    local.set 2
    loop ;; label = @1
      local.get 2
      i32.const 32
      i32.eq
      if ;; label = @2
        i32.const 0
        i64.const 0
        i64.const 0
        i64.const 0
        local.set 5
        local.set 4
        local.set 3
        local.set 2
        loop ;; label = @3
          local.get 2
          i32.const 240
          i32.lt_u
          if ;; label = @4
            local.get 2
            f64.convert_i32_u
            local.set 8
            i32.const 0
            local.set 6
            loop ;; label = @5
              local.get 6
              i32.const 320
              i32.lt_u
              if ;; label = @6
                local.get 0
                i64.const -7046029254386353131
                i64.xor
                i64.const -7046029254386353131
                i64.add
                local.tee 9
                local.get 9
                i32.const 30
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 9
                local.get 9
                i32.const 27
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 9
                local.get 9
                i32.const 31
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                local.tee 9
                i64.const 8709371129873690708
                i64.add
                local.tee 10
                local.get 10
                i32.const 30
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 10
                local.get 10
                i32.const 27
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 10
                local.get 10
                i32.const 31
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                local.set 10
                f64.const 0x1p+0 (;=1;)
                local.get 10
                local.get 10
                local.set 19
                i64.const 3022254103954493847
                local.set 20
                local.get 19
                i64.const 4294967295
                i64.and
                local.get 20
                i64.const 32
                i64.shr_u
                i64.mul
                local.set 21
                local.get 19
                i64.const 32
                i64.shr_u
                local.get 20
                i64.const 4294967295
                i64.and
                i64.mul
                local.set 22
                local.get 19
                i64.const 32
                i64.shr_u
                local.get 20
                i64.const 32
                i64.shr_u
                i64.mul
                local.get 21
                i64.const 32
                i64.shr_u
                i64.add
                local.get 22
                i64.const 32
                i64.shr_u
                i64.add
                local.get 19
                i64.const 4294967295
                i64.and
                local.get 20
                i64.const 4294967295
                i64.and
                i64.mul
                i64.const 32
                i64.shr_u
                local.get 21
                i64.const 4294967295
                i64.and
                i64.add
                local.get 22
                i64.const 4294967295
                i64.and
                i64.add
                i64.const 32
                i64.shr_u
                i64.add
                i64.const 13
                i64.shr_u
                i64.const 50001
                i64.mul
                i64.sub
                f64.convert_i64_u
                f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
                f64.mul
                f64.add
                local.set 11
                local.get 9
                i64.const 4354685564936845354
                i64.add
                local.tee 10
                local.get 10
                i32.const 30
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 10
                local.get 10
                i32.const 27
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 10
                local.get 10
                i32.const 31
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                local.set 10
                local.get 6
                f64.convert_i32_u
                f64.const 0x1.4p+8 (;=320;)
                f64.div
                f64.const 0x1.3d70a3d70a3d7p-1 (;=0.62;)
                f64.sub
                f64.const 0x1.999999999999ap+1 (;=3.2;)
                local.get 11
                f64.div
                f64.mul
                local.get 10
                local.get 10
                local.set 21
                i64.const 1888852150540331069
                local.set 20
                local.get 21
                i64.const 4294967295
                i64.and
                local.get 20
                i64.const 32
                i64.shr_u
                i64.mul
                local.set 19
                local.get 21
                i64.const 32
                i64.shr_u
                local.get 20
                i64.const 4294967295
                i64.and
                i64.mul
                local.set 22
                local.get 21
                i64.const 32
                i64.shr_u
                local.get 20
                i64.const 32
                i64.shr_u
                i64.mul
                local.get 19
                i64.const 32
                i64.shr_u
                i64.add
                local.get 22
                i64.const 32
                i64.shr_u
                i64.add
                local.get 21
                i64.const 4294967295
                i64.and
                local.get 20
                i64.const 4294967295
                i64.and
                i64.mul
                i64.const 32
                i64.shr_u
                local.get 19
                i64.const 4294967295
                i64.and
                i64.add
                local.get 22
                i64.const 4294967295
                i64.and
                i64.add
                i64.const 32
                i64.shr_u
                i64.add
                i64.const 11
                i64.shr_u
                i64.const 20001
                i64.mul
                i64.sub
                i64.const 10000
                i64.sub
                f64.convert_i64_s
                f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
                f64.mul
                f64.add
                local.set 12
                local.get 9
                i64.const -2691343689449507777
                i64.add
                local.tee 9
                local.get 9
                i32.const 30
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 9
                local.get 9
                i32.const 27
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 9
                local.get 9
                i32.const 31
                i64.extend_i32_u
                i64.shr_u
                i64.xor
                local.set 9
                local.get 8
                f64.const 0x1.ep+7 (;=240;)
                f64.div
                f64.const 0x1p-1 (;=0.5;)
                f64.sub
                f64.const 0x1.3333333333333p+1 (;=2.4;)
                local.get 11
                f64.div
                f64.mul
                local.get 9
                local.get 9
                local.set 19
                i64.const 1888852150540331069
                local.set 20
                local.get 19
                i64.const 4294967295
                i64.and
                local.get 20
                i64.const 32
                i64.shr_u
                i64.mul
                local.set 21
                local.get 19
                i64.const 32
                i64.shr_u
                local.get 20
                i64.const 4294967295
                i64.and
                i64.mul
                local.set 22
                local.get 19
                i64.const 32
                i64.shr_u
                local.get 20
                i64.const 32
                i64.shr_u
                i64.mul
                local.get 21
                i64.const 32
                i64.shr_u
                i64.add
                local.get 22
                i64.const 32
                i64.shr_u
                i64.add
                local.get 19
                i64.const 4294967295
                i64.and
                local.get 20
                i64.const 4294967295
                i64.and
                i64.mul
                i64.const 32
                i64.shr_u
                local.get 21
                i64.const 4294967295
                i64.and
                i64.add
                local.get 22
                i64.const 4294967295
                i64.and
                i64.add
                i64.const 32
                i64.shr_u
                i64.add
                i64.const 11
                i64.shr_u
                i64.const 20001
                i64.mul
                i64.sub
                i64.const 10000
                i64.sub
                f64.convert_i64_s
                f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
                f64.mul
                f64.add
                local.set 11
                i32.const 0
                f64.const 0x0p+0 (;=0;)
                f64.const 0x0p+0 (;=0;)
                local.set 14
                local.set 13
                local.set 7
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    i32.const 200
                    i32.lt_u
                    if ;; label = @9
                      local.get 13
                      local.get 13
                      f64.mul
                      local.set 15
                      local.get 14
                      local.get 14
                      f64.mul
                      local.set 16
                      local.get 15
                      local.get 16
                      f64.add
                      f64.const 0x1p+2 (;=4;)
                      f64.gt
                      if ;; label = @10
                        br 2 (;@8;)
                      else
                        local.get 7
                        i32.const 1
                        i32.add
                        local.get 15
                        local.get 16
                        f64.sub
                        local.get 12
                        f64.add
                        f64.const 0x1p+1 (;=2;)
                        local.get 13
                        f64.mul
                        local.get 14
                        f64.mul
                        local.get 11
                        f64.add
                        local.set 14
                        local.set 13
                        local.set 7
                        br 3 (;@7;)
                      end
                    else
                      br 1 (;@8;)
                    end
                  end
                  block ;; label = @8
                    local.get 7
                    i32.const 200
                    i32.ge_u
                    if ;; label = @9
                      local.get 1
                      i32.const 124
                      i32.add
                      local.tee 17
                      local.get 17
                      i32.load align=1
                      i32.const 1
                      i32.add
                      i32.store align=1
                      local.get 4
                      i64.const 1
                      i64.add
                      local.set 4
                      br 1 (;@8;)
                    else
                      local.get 7
                      i32.const 31
                      i32.mul
                      i64.extend_i32_u
                      i32.const 1374389535
                      i64.extend_i32_u
                      i64.mul
                      i64.const 32
                      i64.shr_u
                      i32.wrap_i64
                      i32.const 6
                      i32.shr_u
                      local.tee 17
                      i32.const 32
                      i32.lt_u
                      if ;; label = @10
                        local.get 1
                        local.get 17
                        i32.const 2
                        i32.shl
                        i32.add
                        local.tee 17
                        local.get 17
                        i32.load align=1
                        i32.const 1
                        i32.add
                        i32.store align=1
                        br 2 (;@8;)
                      else
                        local.get 17
                        i32.const 32
                        i32.const 1065244
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    end
                  end
                  local.get 7
                  i64.extend_i32_u
                  local.set 9
                  local.get 6
                  i32.const 1
                  i32.add
                  local.get 3
                  local.get 9
                  i64.add
                  local.get 5
                  i32.const 5
                  i64.extend_i32_u
                  i64.rotl
                  local.get 9
                  i64.const -7046029254386353131
                  i64.mul
                  i64.xor
                  local.get 6
                  i64.extend_i32_u
                  i32.const 11
                  i64.extend_i32_u
                  i64.rotl
                  i64.xor
                  local.get 2
                  i64.extend_i32_u
                  i32.const 23
                  i64.extend_i32_u
                  i64.rotl
                  i64.xor
                  local.set 5
                  local.set 3
                  local.set 6
                  br 2 (;@5;)
                end
              else
                local.get 2
                i32.const 1
                i32.add
                local.set 2
                br 3 (;@3;)
              end
            end
          else
            local.get 18
            i32.const 128
            i32.add
            local.tee 2
            i64.const 320
            i64.store align=1
            local.get 2
            i32.const 8
            call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
            local.get 18
            i32.const 136
            i32.add
            local.tee 2
            i64.const 240
            i64.store align=1
            local.get 2
            i32.const 8
            call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
            local.get 18
            i32.const 144
            i32.add
            local.tee 2
            i64.const 200
            i64.store align=1
            local.get 2
            i32.const 8
            call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
            local.get 18
            i32.const 152
            i32.add
            local.set 2
            i32.const 0
            local.set 6
            loop ;; label = @5
              local.get 6
              i32.const 32
              i32.lt_u
              local.tee 7
              if ;; label = @6
                local.get 7
                if ;; label = @7
                  local.get 2
                  local.get 1
                  local.get 6
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load align=1
                  i32.store align=1
                  local.get 2
                  i32.const 4
                  call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
                  local.get 6
                  i32.const 1
                  i32.add
                  local.set 6
                  br 2 (;@5;)
                else
                  local.get 6
                  i32.const 32
                  i32.const 1065112
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 18
                i32.const 156
                i32.add
                local.tee 7
                local.get 3
                i64.store align=1
                local.get 7
                i32.const 8
                call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
                local.get 18
                i32.const 164
                i32.add
                local.tee 7
                local.get 4
                i64.store align=1
                local.get 7
                i32.const 8
                call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
                local.get 18
                i32.const 172
                i32.add
                local.tee 7
                local.get 5
                i64.store align=1
                local.get 7
                i32.const 8
                call $_RNvCsgUqo56RTmP6_10mandelbrot10emit_bytes
                local.get 5
                local.get 3
                i32.const 17
                i64.extend_i32_u
                i64.rotl
                i64.xor
                local.get 4
                i32.const 31
                i64.extend_i32_u
                i64.rotl
                i64.xor
                local.get 18
                i32.const 192
                i32.add
                global.set $__stack_pointer
                return
              end
            end
          end
        end
      else
        local.get 1
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        i32.const 0
        i32.store align=1
        local.get 2
        i32.const 1
        i32.add
        local.set 2
        br 1 (;@1;)
      end
    end
    unreachable
  )
```

## LLVM -O2 `run` (424,733,388 instructions, 100.0%)

```wat
(func $run (;4;) (type 5) (param i64) (result i64)
    (local i32 i64 f64 f64 f64 f64 f64 i64 i64 i64 i64 i64 f64 f64 i32 f64 f64 f64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 144
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    i32.const 8
    i32.add
    i32.const 0
    i32.const 128
    memory.fill
    local.get 0
    i64.const -7046029254386353131
    i64.xor
    i64.const -7046029254386353131
    i64.add
    local.tee 0
    i64.const 30
    i64.shr_u
    local.get 0
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 0
    i64.const 27
    i64.shr_u
    local.get 0
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 0
    i64.const 31
    i64.shr_u
    local.get 0
    i64.xor
    local.tee 0
    i64.const -2691343689449507777
    i64.add
    local.tee 2
    i64.const 30
    i64.shr_u
    local.get 2
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 2
    i64.const 27
    i64.shr_u
    local.get 2
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 2
    i64.const 31
    i64.shr_u
    local.get 2
    i64.xor
    i64.const 20001
    i64.rem_u
    i64.const -10000
    i64.add
    f64.convert_i64_s
    f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
    f64.mul
    local.set 3
    local.get 0
    i64.const 4354685564936845354
    i64.add
    local.tee 2
    i64.const 30
    i64.shr_u
    local.get 2
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 2
    i64.const 27
    i64.shr_u
    local.get 2
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 2
    i64.const 31
    i64.shr_u
    local.get 2
    i64.xor
    i64.const 20001
    i64.rem_u
    i64.const -10000
    i64.add
    f64.convert_i64_s
    f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
    f64.mul
    local.set 4
    f64.const 0x1.999999999999ap+1 (;=3.2;)
    local.get 0
    i64.const 8709371129873690708
    i64.add
    local.tee 0
    i64.const 30
    i64.shr_u
    local.get 0
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 0
    i64.const 27
    i64.shr_u
    local.get 0
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 0
    i64.const 31
    i64.shr_u
    local.get 0
    i64.xor
    i64.const 50001
    i64.rem_u
    f64.convert_i64_u
    f64.const 0x1.0c6f7a0b5ed8dp-20 (;=0.000001;)
    f64.mul
    f64.const 0x1p+0 (;=1;)
    f64.add
    local.tee 5
    f64.div
    local.set 6
    f64.const 0x1.3333333333333p+1 (;=2.4;)
    local.get 5
    f64.div
    local.set 7
    i64.const 0
    local.set 8
    i64.const 0
    local.set 9
    i64.const 0
    local.set 10
    i64.const 0
    local.set 11
    loop ;; label = @1
      local.get 8
      i64.const 23
      i64.shl
      local.set 12
      local.get 3
      local.get 7
      local.get 8
      f64.convert_i64_u
      f64.const 0x1.ep+7 (;=240;)
      f64.div
      f64.const -0x1p-1 (;=-0.5;)
      f64.add
      f64.mul
      f64.add
      local.set 13
      i64.const 0
      local.set 0
      loop ;; label = @2
        local.get 4
        local.get 6
        local.get 0
        f64.convert_i64_u
        f64.const 0x1.4p+8 (;=320;)
        f64.div
        f64.const -0x1.3d70a3d70a3d7p-1 (;=-0.62;)
        f64.add
        f64.mul
        f64.add
        local.set 14
        i32.const 0
        local.set 15
        f64.const 0x0p+0 (;=0;)
        local.set 5
        f64.const 0x0p+0 (;=0;)
        local.set 16
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              loop ;; label = @6
                local.get 5
                local.get 5
                f64.mul
                local.tee 17
                local.get 16
                local.get 16
                f64.mul
                local.tee 18
                f64.add
                f64.const 0x1p+2 (;=4;)
                f64.gt
                br_if 2 (;@4;)
                local.get 14
                local.get 17
                local.get 18
                f64.sub
                f64.add
                local.tee 17
                local.get 17
                f64.mul
                local.tee 18
                local.get 13
                local.get 16
                local.get 5
                local.get 5
                f64.add
                f64.mul
                f64.add
                local.tee 16
                local.get 16
                f64.mul
                local.tee 5
                f64.add
                f64.const 0x1p+2 (;=4;)
                f64.gt
                br_if 1 (;@5;)
                local.get 14
                local.get 18
                local.get 5
                f64.sub
                f64.add
                local.set 5
                local.get 13
                local.get 16
                local.get 17
                local.get 17
                f64.add
                f64.mul
                f64.add
                local.set 16
                local.get 15
                i32.const 2
                i32.add
                local.tee 15
                i32.const 200
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 1
              i32.load offset=132
              i32.const 1
              i32.add
              i32.store offset=132
              local.get 10
              i64.const 1
              i64.add
              local.set 10
              i64.const 200
              local.set 2
              br 2 (;@3;)
            end
            local.get 15
            i32.const 1
            i32.add
            local.set 15
          end
          local.get 1
          i32.const 8
          i32.add
          local.get 15
          i32.const 31
          i32.mul
          i32.const 65535
          i32.and
          i32.const 200
          i32.div_u
          i32.const 2
          i32.shl
          i32.add
          local.tee 19
          local.get 19
          i32.load
          i32.const 1
          i32.add
          i32.store
          local.get 15
          i64.extend_i32_u
          local.set 2
        end
        local.get 0
        i64.const 11
        i64.shl
        local.get 12
        i64.xor
        local.get 11
        i64.const 5
        i64.rotl
        i64.xor
        local.get 2
        i64.const -7046029254386353131
        i64.mul
        i64.xor
        local.set 11
        local.get 2
        local.get 9
        i64.add
        local.set 9
        local.get 0
        i64.const 1
        i64.add
        local.tee 0
        i64.const 320
        i64.ne
        br_if 0 (;@2;)
      end
      local.get 8
      i64.const 1
      i64.add
      local.tee 8
      i64.const 240
      i64.ne
      br_if 0 (;@1;)
    end
    local.get 1
    i64.const 320
    i64.store offset=136
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1065428
        local.tee 19
        i32.sub
        local.tee 15
        local.get 15
        i32.const 16384
        i32.gt_u
        select
        local.tee 15
        i32.const 8
        local.get 15
        i32.const 8
        i32.lt_u
        select
        local.tee 20
        local.get 19
        i32.add
        local.tee 15
        local.get 19
        i32.lt_u
        br_if 0 (;@2;)
        local.get 15
        i32.const 16385
        i32.ge_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 20
          i32.eqz
          br_if 0 (;@3;)
          local.get 19
          i32.const 1049044
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 20
          memory.copy
        end
        local.get 1
        i64.const 240
        i64.store offset=136
        block ;; label = @3
          i32.const 16384
          local.get 15
          i32.sub
          local.tee 19
          i32.const 8
          local.get 19
          i32.const 8
          i32.lt_u
          select
          local.tee 19
          i32.eqz
          br_if 0 (;@3;)
          local.get 15
          i32.const 1049044
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 19
          memory.copy
        end
        local.get 1
        i64.const 200
        i64.store offset=136
        block ;; label = @3
          i32.const 16384
          local.get 19
          local.get 15
          i32.add
          local.tee 15
          i32.sub
          local.tee 19
          i32.const 8
          local.get 19
          i32.const 8
          i32.lt_u
          select
          local.tee 19
          i32.eqz
          br_if 0 (;@3;)
          local.get 15
          i32.const 1049044
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 19
          memory.copy
        end
        i32.const 0
        local.get 19
        local.get 15
        i32.add
        local.tee 15
        i32.store offset=1065428
        i32.const 0
        local.set 20
        loop ;; label = @3
          local.get 1
          local.get 1
          i32.const 8
          i32.add
          local.get 20
          i32.add
          i32.load
          i32.store offset=136
          i32.const 0
          i32.const 16384
          local.get 15
          i32.sub
          local.tee 19
          local.get 19
          i32.const 16384
          i32.gt_u
          select
          local.tee 19
          i32.const 4
          local.get 19
          i32.const 4
          i32.lt_u
          select
          local.tee 21
          local.get 15
          i32.add
          local.tee 19
          i32.const 16385
          i32.ge_u
          br_if 2 (;@1;)
          block ;; label = @4
            local.get 21
            i32.eqz
            br_if 0 (;@4;)
            local.get 15
            i32.const 1049044
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 21
            memory.copy
          end
          i32.const 0
          local.get 19
          i32.store offset=1065428
          local.get 19
          local.set 15
          local.get 20
          i32.const 4
          i32.add
          local.tee 20
          i32.const 128
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 1
        local.get 9
        i64.store offset=136
        block ;; label = @3
          i32.const 16384
          local.get 19
          i32.sub
          local.tee 15
          i32.const 8
          local.get 15
          i32.const 8
          i32.lt_u
          select
          local.tee 15
          i32.eqz
          br_if 0 (;@3;)
          local.get 19
          i32.const 1049044
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 15
          memory.copy
        end
        local.get 1
        local.get 10
        i64.store offset=136
        block ;; label = @3
          i32.const 16384
          local.get 15
          local.get 19
          i32.add
          local.tee 15
          i32.sub
          local.tee 19
          i32.const 8
          local.get 19
          i32.const 8
          i32.lt_u
          select
          local.tee 19
          i32.eqz
          br_if 0 (;@3;)
          local.get 15
          i32.const 1049044
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 19
          memory.copy
        end
        local.get 1
        local.get 11
        i64.store offset=136
        block ;; label = @3
          i32.const 16384
          local.get 19
          local.get 15
          i32.add
          local.tee 15
          i32.sub
          local.tee 19
          i32.const 8
          local.get 19
          i32.const 8
          i32.lt_u
          select
          local.tee 19
          i32.eqz
          br_if 0 (;@3;)
          local.get 15
          i32.const 1049044
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 19
          memory.copy
        end
        i32.const 0
        local.get 19
        local.get 15
        i32.add
        i32.store offset=1065428
        local.get 1
        i32.const 144
        i32.add
        global.set $__stack_pointer
        local.get 10
        i64.const 31
        i64.rotl
        local.get 9
        i64.const 17
        i64.rotl
        i64.xor
        local.get 11
        i64.xor
        return
      end
      local.get 19
      local.get 15
      i32.const 16384
      i32.const 1048828
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    local.get 15
    local.get 19
    i32.const 16384
    i32.const 1048828
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )
```

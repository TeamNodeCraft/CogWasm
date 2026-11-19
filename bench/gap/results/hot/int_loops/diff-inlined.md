# int_loops::body vs LLVM's inlined `run`

LLVM -O2 inlined the whole program into `run`; this is the hot function that survives as a separate body in our module.

## Ours (181,175 instructions, 73.1%)

```wat
(func $_RNvCsgBEE3CyPQPa_9int_loops4body (;3;) (type 3) (param i64) (result i64)
    (local i64 i32 i32 i32 i32 i32 i64 i64 i64 i64 i32 i64 i32 i32 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 80
    i32.sub
    local.tee 14
    global.set $__stack_pointer
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
    local.tee 0
    local.get 0
    i32.const 31
    i64.extend_i32_u
    i64.shr_u
    i64.xor
    local.tee 0
    i64.const -2691343689449507777
    i64.add
    local.tee 1
    local.get 1
    i32.const 30
    i64.extend_i32_u
    i64.shr_u
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 1
    local.get 1
    i32.const 27
    i64.extend_i32_u
    i64.shr_u
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.set 1
    local.get 0
    i64.const 4354685564936845354
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
    local.get 14
    i32.const 8
    i32.add
    local.set 2
    local.get 14
    i32.const 0
    i32.add
    local.set 3
    local.get 14
    i32.const 48
    i32.add
    local.set 4
    local.get 14
    i32.const 56
    i32.add
    local.set 5
    local.get 14
    i32.const 64
    i32.add
    local.set 6
    i64.const 0
    local.get 1
    local.get 1
    i32.const 31
    i64.extend_i32_u
    i64.shr_u
    i64.xor
    i64.const 1
    i64.or
    local.get 0
    local.get 0
    i32.const 31
    i64.extend_i32_u
    i64.shr_u
    i64.xor
    i64.const 0
    local.set 8
    local.set 7
    local.set 0
    local.set 1
    loop ;; label = @1
      local.get 1
      i64.const 128
      i64.lt_u
      if ;; label = @2
        local.get 2
        local.get 1
        i64.store align=1
        local.get 3
        i64.const 1
        i64.store align=1
        local.get 2
        i64.load align=1
        local.set 10
        i32.const 0
        local.get 7
        local.get 0
        i64.xor
        local.get 0
        local.set 7
        local.set 9
        local.set 11
        loop ;; label = @3
          local.get 11
          local.get 0
          i32.wrap_i64
          i32.const 15
          i32.and
          i32.lt_u
          if ;; label = @4
            local.get 11
            i32.const 1
            i32.add
            local.get 9
            i64.const -7046029254386353131
            i64.mul
            local.get 11
            i64.extend_i32_u
            i32.const 7
            i64.extend_i32_u
            i64.shl
            i64.add
            local.get 11
            i32.const 63
            i32.and
            i64.extend_i32_u
            i64.rotl
            local.set 9
            local.set 11
            br 1 (;@3;)
          else
            block ;; label = @5
              block ;; label = @6
                local.get 9
                local.get 10
                i64.const 63
                i64.and
                i64.rotr
                local.tee 0
                local.get 0
                local.set 15
                i64.const -3689348814741910323
                local.set 16
                local.get 15
                i64.const 4294967295
                i64.and
                local.get 16
                i64.const 32
                i64.shr_u
                i64.mul
                local.set 17
                local.get 15
                i64.const 32
                i64.shr_u
                local.get 16
                i64.const 4294967295
                i64.and
                i64.mul
                local.set 18
                local.get 15
                i64.const 32
                i64.shr_u
                local.get 16
                i64.const 32
                i64.shr_u
                i64.mul
                local.get 17
                i64.const 32
                i64.shr_u
                i64.add
                local.get 18
                i64.const 32
                i64.shr_u
                i64.add
                local.get 15
                i64.const 4294967295
                i64.and
                local.get 16
                i64.const 4294967295
                i64.and
                i64.mul
                i64.const 32
                i64.shr_u
                local.get 17
                i64.const 4294967295
                i64.and
                i64.add
                local.get 18
                i64.const 4294967295
                i64.and
                i64.add
                i64.const 32
                i64.shr_u
                i64.add
                i64.const 2
                i64.shr_u
                i64.const 5
                i64.mul
                i64.sub
                local.tee 12
                i64.const 4294967295
                i64.gt_u
                if ;; label = @7
                  br 1 (;@6;)
                else
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            local.get 12
                            i32.wrap_i64
                            br_table 1 (;@11;) 2 (;@10;) 3 (;@9;) 4 (;@8;) 0 (;@12;)
                          end
                          br 5 (;@6;)
                        end
                        local.get 8
                        local.get 7
                        local.get 9
                        i64.add
                        i64.add
                        local.set 12
                        br 5 (;@5;)
                      end
                      local.get 8
                      local.get 7
                      local.get 9
                      i64.add
                      local.get 0
                      i64.xor
                      i64.sub
                      local.set 12
                      br 4 (;@5;)
                    end
                    local.get 8
                    local.get 7
                    local.get 9
                    i64.add
                    i32.const 17
                    i64.extend_i32_u
                    i64.rotl
                    i64.xor
                    local.set 12
                    br 3 (;@5;)
                  end
                  local.get 8
                  i64.const 3
                  i64.mul
                  local.get 7
                  local.get 9
                  i64.add
                  local.get 0
                  i64.xor
                  i32.const 3
                  i64.extend_i32_u
                  i64.shr_u
                  i64.add
                  local.set 12
                  br 2 (;@5;)
                end
              end
              local.get 7
              local.get 9
              i64.add
              i32.wrap_i64
              i32.const 15
              i32.and
              local.set 11
              i32.const 0
              local.get 0
              local.set 12
              local.set 13
              loop ;; label = @6
                local.get 13
                local.get 11
                i32.lt_u
                if ;; label = @7
                  local.get 13
                  i32.const 1
                  i32.add
                  local.get 12
                  i64.const -7046029254386353131
                  i64.mul
                  local.get 13
                  i64.extend_i32_u
                  i32.const 7
                  i64.extend_i32_u
                  i64.shl
                  i64.add
                  local.get 13
                  i32.const 63
                  i32.and
                  i64.extend_i32_u
                  i64.rotl
                  local.set 12
                  local.set 13
                  br 1 (;@6;)
                else
                  local.get 8
                  local.get 12
                  i64.add
                  local.set 12
                  br 2 (;@5;)
                end
              end
            end
            block ;; label = @5
              local.get 10
              i64.const 7
              i64.and
              i64.eqz
              if ;; label = @6
                local.get 7
                local.get 9
                i64.add
                local.set 10
                local.get 4
                local.get 10
                i64.store align=1
                local.get 4
                i32.const 8
                call $_RNvCsgBEE3CyPQPa_9int_loops10emit_bytes
                local.get 5
                local.get 10
                local.get 0
                i64.xor
                i64.store align=1
                local.get 5
                i32.const 8
                call $_RNvCsgBEE3CyPQPa_9int_loops10emit_bytes
                local.get 6
                local.get 12
                i64.store align=1
                local.get 6
                i32.const 8
                call $_RNvCsgBEE3CyPQPa_9int_loops10emit_bytes
                br 1 (;@5;)
              else
                br 1 (;@5;)
              end
            end
            local.get 7
            local.get 9
            i64.add
            local.set 7
            local.get 1
            i64.const 1
            i64.add
            local.get 7
            local.get 0
            i64.xor
            local.get 12
            local.set 8
            local.set 0
            local.set 1
            br 3 (;@1;)
          end
        end
      else
        local.get 14
        i32.const 24
        i32.add
        local.set 6
        local.get 14
        i32.const 16
        i32.add
        local.set 5
        i64.const 0
        local.get 8
        local.set 7
        local.set 8
        loop ;; label = @3
          block ;; label = @4
            local.get 8
            i64.const 16
            i64.lt_u
            if ;; label = @5
              local.get 6
              local.get 8
              i64.store align=1
              local.get 5
              i64.const 1
              i64.store align=1
              local.get 6
              i64.load align=1
              local.set 1
              local.get 14
              i32.const 40
              i32.add
              local.set 4
              local.get 14
              i32.const 32
              i32.add
              local.set 3
              i64.const 0
              local.get 7
              local.set 0
              local.set 7
              loop ;; label = @6
                local.get 7
                i64.const 16
                i64.lt_u
                if ;; label = @7
                  local.get 4
                  local.get 7
                  i64.store align=1
                  local.get 3
                  i64.const 1
                  i64.store align=1
                  local.get 4
                  i64.load align=1
                  local.set 9
                  local.get 1
                  local.get 9
                  i64.mul
                  local.get 0
                  i64.add
                  local.tee 10
                  i64.const 7
                  i64.and
                  i64.eqz
                  if ;; label = @8
                    local.get 7
                    i64.const 1
                    i64.add
                    local.set 7
                    br 2 (;@6;)
                  else
                    local.get 0
                    local.get 10
                    local.get 1
                    local.get 9
                    i64.add
                    i64.const 63
                    i64.and
                    i64.rotl
                    i64.xor
                    local.tee 0
                    i64.const 4095
                    i64.and
                    i64.eqz
                    if ;; label = @9
                      local.get 0
                      local.set 7
                      br 5 (;@4;)
                    else
                      local.get 7
                      i64.const 1
                      i64.add
                      local.set 7
                      br 3 (;@6;)
                    end
                  end
                else
                  local.get 8
                  i64.const 1
                  i64.add
                  local.get 0
                  local.set 7
                  local.set 8
                  br 4 (;@3;)
                end
              end
            else
              br 1 (;@4;)
            end
          end
          local.get 14
          i32.const 72
          i32.add
          local.tee 6
          local.get 7
          i64.store align=1
          local.get 6
          i32.const 8
          call $_RNvCsgBEE3CyPQPa_9int_loops10emit_bytes
          local.get 7
          local.get 14
          i32.const 80
          i32.add
          global.set $__stack_pointer
          return
        end
      end
    end
    unreachable
  )
```

## LLVM -O2 `run` (123,322 instructions, 100.0%)

```wat
(func $run (;4;) (type 5) (param i64) (result i64)
    (local i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 1
    global.set $__stack_pointer
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
    local.get 0
    i64.const -2691343689449507777
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
    i64.const 1
    i64.or
    local.tee 3
    i64.xor
    local.set 4
    i64.const 0
    local.set 5
    i64.const 0
    local.set 6
    block ;; label = @1
      block ;; label = @2
        loop ;; label = @3
          block ;; label = @4
            block ;; label = @5
              local.get 3
              i64.const 15
              i64.and
              local.tee 2
              i64.eqz
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              local.set 7
              br 1 (;@4;)
            end
            i64.const 0
            local.set 0
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i64.const 1
                i64.ne
                br_if 0 (;@6;)
                local.get 4
                local.set 7
                br 1 (;@5;)
              end
              local.get 3
              i64.const 1
              i64.and
              local.set 8
              local.get 3
              i64.const 14
              i64.and
              local.set 9
              i64.const 0
              local.set 0
              i64.const 128
              local.set 2
              local.get 4
              local.set 7
              loop ;; label = @6
                local.get 2
                local.get 2
                local.get 7
                i64.const -7046029254386353131
                i64.mul
                i64.add
                i64.const -128
                i64.add
                local.get 0
                i64.rotl
                i64.const -7046029254386353131
                i64.mul
                i64.add
                local.get 0
                i64.const 1
                i64.add
                i64.rotl
                local.set 7
                local.get 2
                i64.const 256
                i64.add
                local.set 2
                local.get 0
                i64.const 2
                i64.add
                local.tee 0
                local.get 9
                i64.ne
                br_if 0 (;@6;)
              end
              local.get 8
              i64.eqz
              br_if 1 (;@4;)
            end
            local.get 7
            i64.const -7046029254386353131
            i64.mul
            local.get 0
            i64.const 7
            i64.shl
            i64.add
            local.get 0
            i64.rotl
            local.set 7
          end
          local.get 7
          local.get 3
          i64.add
          local.tee 9
          local.get 7
          local.get 6
          i64.rotr
          local.tee 4
          i64.xor
          local.set 3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 4
                      i64.const 5
                      i64.rem_u
                      local.tee 0
                      i64.const 3
                      i64.gt_u
                      br_if 0 (;@9;)
                      local.get 0
                      i32.wrap_i64
                      br_table 1 (;@8;) 2 (;@7;) 3 (;@6;) 4 (;@5;) 1 (;@8;)
                    end
                    local.get 4
                    local.set 7
                    block ;; label = @9
                      local.get 9
                      i64.const 15
                      i64.and
                      local.tee 2
                      i64.eqz
                      br_if 0 (;@9;)
                      i64.const 0
                      local.set 0
                      local.get 4
                      local.set 7
                      block ;; label = @10
                        local.get 2
                        i64.const 1
                        i64.eq
                        br_if 0 (;@10;)
                        local.get 9
                        i64.const 1
                        i64.and
                        local.set 10
                        local.get 9
                        i64.const 14
                        i64.and
                        local.set 8
                        i64.const 0
                        local.set 0
                        i64.const 128
                        local.set 2
                        local.get 4
                        local.set 7
                        loop ;; label = @11
                          local.get 2
                          local.get 2
                          local.get 7
                          i64.const -7046029254386353131
                          i64.mul
                          i64.add
                          i64.const -128
                          i64.add
                          local.get 0
                          i64.rotl
                          i64.const -7046029254386353131
                          i64.mul
                          i64.add
                          local.get 0
                          i64.const 1
                          i64.add
                          i64.rotl
                          local.set 7
                          local.get 2
                          i64.const 256
                          i64.add
                          local.set 2
                          local.get 0
                          i64.const 2
                          i64.add
                          local.tee 0
                          local.get 8
                          i64.ne
                          br_if 0 (;@11;)
                        end
                        local.get 10
                        i64.eqz
                        br_if 1 (;@9;)
                      end
                      local.get 7
                      i64.const -7046029254386353131
                      i64.mul
                      local.get 0
                      i64.const 7
                      i64.shl
                      i64.add
                      local.get 0
                      i64.rotl
                      local.set 7
                    end
                    local.get 7
                    local.get 5
                    i64.add
                    local.set 5
                    br 4 (;@4;)
                  end
                  local.get 9
                  local.get 5
                  i64.add
                  local.set 5
                  br 3 (;@4;)
                end
                local.get 5
                local.get 3
                i64.sub
                local.set 5
                br 2 (;@4;)
              end
              local.get 9
              i64.const 17
              i64.rotl
              local.get 5
              i64.xor
              local.set 5
              br 1 (;@4;)
            end
            local.get 3
            i64.const 3
            i64.shr_u
            local.get 5
            i64.const 3
            i64.mul
            i64.add
            local.set 5
          end
          block ;; label = @4
            local.get 6
            i64.const 7
            i64.and
            i64.eqz
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 9
            i64.store offset=8
            i32.const 0
            i32.const 16384
            i32.const 0
            i32.load offset=1065428
            local.tee 11
            i32.sub
            local.tee 12
            local.get 12
            i32.const 16384
            i32.gt_u
            select
            local.tee 12
            i32.const 8
            local.get 12
            i32.const 8
            i32.lt_u
            select
            local.tee 13
            local.get 11
            i32.add
            local.tee 12
            local.get 11
            i32.lt_u
            br_if 2 (;@2;)
            local.get 12
            i32.const 16385
            i32.ge_u
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 13
              i32.eqz
              br_if 0 (;@5;)
              local.get 11
              i32.const 1049044
              i32.add
              local.get 1
              i32.const 8
              i32.add
              local.get 13
              memory.copy
            end
            local.get 1
            local.get 3
            i64.store offset=8
            block ;; label = @5
              i32.const 16384
              local.get 12
              i32.sub
              local.tee 11
              i32.const 8
              local.get 11
              i32.const 8
              i32.lt_u
              select
              local.tee 11
              i32.eqz
              br_if 0 (;@5;)
              local.get 12
              i32.const 1049044
              i32.add
              local.get 1
              i32.const 8
              i32.add
              local.get 11
              memory.copy
            end
            local.get 1
            local.get 5
            i64.store offset=8
            block ;; label = @5
              i32.const 16384
              local.get 11
              local.get 12
              i32.add
              local.tee 12
              i32.sub
              local.tee 11
              i32.const 8
              local.get 11
              i32.const 8
              i32.lt_u
              select
              local.tee 11
              i32.eqz
              br_if 0 (;@5;)
              local.get 12
              i32.const 1049044
              i32.add
              local.get 1
              i32.const 8
              i32.add
              local.get 11
              memory.copy
            end
            i32.const 0
            local.get 11
            local.get 12
            i32.add
            i32.store offset=1065428
          end
          local.get 6
          i64.const 1
          i64.add
          local.tee 6
          i64.const 128
          i64.ne
          br_if 0 (;@3;)
        end
        i64.const 0
        local.set 6
        br 1 (;@1;)
      end
      local.get 11
      local.get 12
      i32.const 16384
      i32.const 1048828
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    loop ;; label = @1
      local.get 6
      local.tee 7
      i64.const 1
      i64.add
      local.set 6
      i64.const 0
      local.set 3
      block ;; label = @2
        block ;; label = @3
          loop ;; label = @4
            local.get 3
            i64.const 15
            i64.gt_u
            br_if 1 (;@3;)
            i64.const 0
            local.get 3
            i64.const 16
            local.get 3
            i64.const 16
            i64.gt_u
            select
            i64.sub
            local.set 9
            i64.const 0
            local.get 3
            i64.sub
            local.set 2
            local.get 5
            local.get 7
            local.get 3
            i64.mul
            i64.add
            local.set 0
            block ;; label = @5
              loop ;; label = @6
                local.get 0
                i64.const 7
                i64.and
                i64.eqz
                i32.eqz
                br_if 1 (;@5;)
                local.get 0
                local.get 7
                i64.add
                local.set 0
                local.get 9
                local.get 2
                i64.const -1
                i64.add
                local.tee 2
                i64.eq
                br_if 3 (;@3;)
                br 0 (;@6;)
              end
            end
            i64.const 1
            local.get 2
            i64.sub
            local.set 3
            local.get 0
            local.get 7
            local.get 2
            i64.sub
            i64.rotl
            local.get 5
            i64.xor
            local.tee 5
            i64.const 4095
            i64.and
            i64.eqz
            i32.eqz
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 6
        i64.const 16
        i64.ne
        br_if 1 (;@1;)
      end
    end
    local.get 1
    local.get 5
    i64.store offset=8
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1065428
        local.tee 12
        i32.sub
        local.tee 11
        local.get 11
        i32.const 16384
        i32.gt_u
        select
        local.tee 11
        i32.const 8
        local.get 11
        i32.const 8
        i32.lt_u
        select
        local.tee 13
        local.get 12
        i32.add
        local.tee 11
        local.get 12
        i32.lt_u
        br_if 0 (;@2;)
        local.get 11
        i32.const 16385
        i32.lt_u
        br_if 1 (;@1;)
      end
      local.get 12
      local.get 11
      i32.const 16384
      i32.const 1048828
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    block ;; label = @1
      local.get 13
      i32.eqz
      br_if 0 (;@1;)
      local.get 12
      i32.const 1049044
      i32.add
      local.get 1
      i32.const 8
      i32.add
      local.get 13
      memory.copy
    end
    i32.const 0
    local.get 11
    i32.store offset=1065428
    local.get 1
    i32.const 16
    i32.add
    global.set $__stack_pointer
    local.get 5
  )
```

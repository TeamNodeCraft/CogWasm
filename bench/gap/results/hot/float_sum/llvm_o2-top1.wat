(func $run (;8;) (type 7) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i32 i32 i64 f64 f64 f64 f64 f32 f32 f32 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1065684
                  br_if 0 (;@7;)
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 2
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 2
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 3 (;@5;)
                  end
                  i32.const 0
                  local.get 2
                  i32.store offset=1065680
                  i32.const 0
                  local.get 2
                  i32.store offset=1065676
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1065684
                  local.get 2
                  i32.const 512
                  i32.or
                  local.set 3
                  local.get 2
                  local.set 4
                  br 1 (;@6;)
                end
                i32.const 0
                i32.load offset=1065676
                local.tee 2
                i32.const -520
                i32.gt_u
                br_if 1 (;@5;)
                local.get 2
                i32.const 7
                i32.add
                i32.const -8
                i32.and
                local.tee 4
                i32.const 512
                i32.add
                local.set 3
                i32.const 0
                i32.load offset=1065680
                local.set 2
              end
              block ;; label = @6
                local.get 3
                local.get 2
                i32.le_u
                br_if 0 (;@6;)
                local.get 3
                local.get 2
                i32.sub
                local.tee 2
                i32.const 16
                i32.shr_u
                local.get 2
                i32.const 65535
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 2
                memory.grow
                local.tee 5
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                i32.const 0
                local.get 5
                local.get 2
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1065680
              end
              i32.const 0
              local.get 3
              i32.store offset=1065676
              local.get 4
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              local.get 4
              i32.store offset=4
              local.get 1
              i32.const 64
              i32.store
              local.get 1
              i32.const 0
              i32.store offset=8
              call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1065684
                  br_if 0 (;@7;)
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 2
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 2
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 4 (;@4;)
                  end
                  i32.const 0
                  local.get 2
                  i32.store offset=1065680
                  i32.const 0
                  local.get 2
                  i32.store offset=1065676
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1065684
                  local.get 2
                  i32.const 512
                  i32.or
                  local.set 3
                  local.get 2
                  local.set 5
                  br 1 (;@6;)
                end
                i32.const 0
                i32.load offset=1065676
                local.tee 2
                i32.const -520
                i32.gt_u
                br_if 2 (;@4;)
                local.get 2
                i32.const 7
                i32.add
                i32.const -8
                i32.and
                local.tee 5
                i32.const 512
                i32.add
                local.set 3
                i32.const 0
                i32.load offset=1065680
                local.set 2
              end
              block ;; label = @6
                local.get 3
                local.get 2
                i32.le_u
                br_if 0 (;@6;)
                local.get 3
                local.get 2
                i32.sub
                local.tee 2
                i32.const 16
                i32.shr_u
                local.get 2
                i32.const 65535
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 2
                memory.grow
                local.tee 6
                i32.const -1
                i32.eq
                br_if 2 (;@4;)
                i32.const 0
                local.get 6
                local.get 2
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1065680
              end
              i32.const 0
              local.set 2
              i32.const 0
              local.get 3
              i32.store offset=1065676
              local.get 5
              i32.eqz
              br_if 1 (;@4;)
              local.get 1
              i32.const 0
              i32.store offset=20
              local.get 1
              local.get 5
              i32.store offset=16
              local.get 1
              i32.const 64
              i32.store offset=12
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
              i64.const -2691343689449507777
              i64.add
              local.set 0
              i32.const 0
              local.set 5
              i32.const 64
              local.set 7
              loop ;; label = @6
                local.get 0
                i64.const 7046029254386353131
                i64.add
                local.tee 8
                i64.const 30
                i64.shr_u
                local.get 8
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 8
                i64.const 27
                i64.shr_u
                local.get 8
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 8
                i64.const 31
                i64.shr_u
                local.get 8
                i64.xor
                i64.const 2000001
                i64.rem_u
                i64.const -1000000
                i64.add
                f64.convert_i64_s
                f64.const 0x1p-10 (;=0.0009765625;)
                f64.mul
                local.set 9
                local.get 0
                i64.const 30
                i64.shr_u
                local.get 0
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 8
                i64.const 27
                i64.shr_u
                local.get 8
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 8
                i64.const 31
                i64.shr_u
                local.get 8
                i64.xor
                i64.const 2000001
                i64.rem_u
                i64.const -1000000
                i64.add
                f64.convert_i64_s
                local.set 10
                block ;; label = @7
                  local.get 2
                  local.get 7
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 1
                  call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsacnkOpZMulW_9float_sum
                  local.get 1
                  i32.load offset=4
                  local.set 4
                  local.get 1
                  i32.load
                  local.set 7
                end
                local.get 10
                f64.const 0x1.848p+9 (;=777;)
                f64.div
                local.set 10
                local.get 4
                local.get 5
                i32.add
                local.get 9
                f64.store
                local.get 1
                local.get 2
                i32.const 1
                i32.add
                local.tee 6
                i32.store offset=8
                block ;; label = @7
                  local.get 2
                  local.get 1
                  i32.load offset=12
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 12
                  i32.add
                  call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecdE8grow_oneCsacnkOpZMulW_9float_sum
                end
                local.get 1
                i32.load offset=16
                local.tee 3
                local.get 5
                i32.add
                local.get 10
                f64.store
                local.get 1
                local.get 6
                i32.store offset=20
                local.get 0
                i64.const 4354685564936845354
                i64.add
                local.set 0
                local.get 5
                i32.const 8
                i32.add
                local.set 5
                local.get 6
                local.set 2
                local.get 6
                i32.const 64
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 1
              i32.load offset=4
              local.tee 2
              f64.load
              f64.const 0x0p+0 (;=0;)
              f64.add
              local.get 2
              f64.load offset=8
              f64.add
              local.get 2
              f64.load offset=16
              f64.add
              local.get 2
              f64.load offset=24
              f64.add
              local.get 2
              f64.load offset=32
              f64.add
              local.get 2
              f64.load offset=40
              f64.add
              local.get 2
              f64.load offset=48
              f64.add
              local.get 2
              f64.load offset=56
              f64.add
              local.get 2
              f64.load offset=64
              f64.add
              local.get 2
              f64.load offset=72
              f64.add
              local.get 2
              f64.load offset=80
              f64.add
              local.get 2
              f64.load offset=88
              f64.add
              local.get 2
              f64.load offset=96
              f64.add
              local.get 2
              f64.load offset=104
              f64.add
              local.get 2
              f64.load offset=112
              f64.add
              local.get 2
              f64.load offset=120
              f64.add
              local.get 2
              f64.load offset=128
              f64.add
              local.get 2
              f64.load offset=136
              f64.add
              local.get 2
              f64.load offset=144
              f64.add
              local.get 2
              f64.load offset=152
              f64.add
              local.get 2
              f64.load offset=160
              f64.add
              local.get 2
              f64.load offset=168
              f64.add
              local.get 2
              f64.load offset=176
              f64.add
              local.get 2
              f64.load offset=184
              f64.add
              local.get 2
              f64.load offset=192
              f64.add
              local.get 2
              f64.load offset=200
              f64.add
              local.get 2
              f64.load offset=208
              f64.add
              local.get 2
              f64.load offset=216
              f64.add
              local.get 2
              f64.load offset=224
              f64.add
              local.get 2
              f64.load offset=232
              f64.add
              local.get 2
              f64.load offset=240
              f64.add
              local.get 2
              f64.load offset=248
              f64.add
              local.get 2
              f64.load offset=256
              f64.add
              local.get 2
              f64.load offset=264
              f64.add
              local.get 2
              f64.load offset=272
              f64.add
              local.get 2
              f64.load offset=280
              f64.add
              local.get 2
              f64.load offset=288
              f64.add
              local.get 2
              f64.load offset=296
              f64.add
              local.get 2
              f64.load offset=304
              f64.add
              local.get 2
              f64.load offset=312
              f64.add
              local.get 2
              f64.load offset=320
              f64.add
              local.get 2
              f64.load offset=328
              f64.add
              local.get 2
              f64.load offset=336
              f64.add
              local.get 2
              f64.load offset=344
              f64.add
              local.get 2
              f64.load offset=352
              f64.add
              local.get 2
              f64.load offset=360
              f64.add
              local.get 2
              f64.load offset=368
              f64.add
              local.get 2
              f64.load offset=376
              f64.add
              local.get 2
              f64.load offset=384
              f64.add
              local.get 2
              f64.load offset=392
              f64.add
              local.get 2
              f64.load offset=400
              f64.add
              local.get 2
              f64.load offset=408
              f64.add
              local.get 2
              f64.load offset=416
              f64.add
              local.get 2
              f64.load offset=424
              f64.add
              local.get 2
              f64.load offset=432
              f64.add
              local.get 2
              f64.load offset=440
              f64.add
              local.get 2
              f64.load offset=448
              f64.add
              local.get 2
              f64.load offset=456
              f64.add
              local.get 2
              f64.load offset=464
              f64.add
              local.get 2
              f64.load offset=472
              f64.add
              local.get 2
              f64.load offset=480
              f64.add
              local.get 2
              f64.load offset=488
              f64.add
              local.get 2
              f64.load offset=496
              f64.add
              local.get 2
              f64.load offset=504
              f64.add
              f64.store offset=24
              i32.const 0
              i32.const 16384
              i32.const 0
              i32.load offset=1065672
              local.tee 5
              i32.sub
              local.tee 6
              local.get 6
              i32.const 16384
              i32.gt_u
              select
              local.tee 6
              i32.const 8
              local.get 6
              i32.const 8
              i32.lt_u
              select
              local.tee 6
              local.get 5
              i32.add
              local.tee 4
              local.get 5
              i32.lt_u
              br_if 2 (;@3;)
              local.get 4
              i32.const 16385
              i32.ge_u
              br_if 2 (;@3;)
              block ;; label = @6
                local.get 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 6
                memory.copy
              end
              i32.const 0
              local.set 6
              i32.const 0
              local.get 4
              i32.store offset=1065672
              f64.const 0x0p+0 (;=0;)
              local.set 9
              f64.const 0x0p+0 (;=0;)
              local.set 10
              loop ;; label = @6
                local.get 9
                local.get 2
                local.get 6
                i32.add
                local.tee 5
                f64.load
                local.get 10
                f64.sub
                local.tee 11
                f64.add
                local.tee 10
                local.get 5
                i32.const 8
                i32.add
                f64.load
                local.get 10
                local.get 9
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 9
                local.get 5
                i32.const 16
                i32.add
                f64.load
                local.get 9
                local.get 10
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 10
                local.get 5
                i32.const 24
                i32.add
                f64.load
                local.get 10
                local.get 9
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 9
                local.get 10
                f64.sub
                local.get 11
                f64.sub
                local.set 10
                local.get 6
                i32.const 32
                i32.add
                local.tee 6
                i32.const 512
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 9
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 4
                i32.sub
                local.tee 5
                i32.const 8
                local.get 5
                i32.const 8
                i32.lt_u
                select
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 6
                memory.copy
              end
              i32.const 0
              local.set 5
              i32.const 0
              local.get 6
              local.get 4
              i32.add
              local.tee 4
              i32.store offset=1065672
              f64.const 0x0p+0 (;=0;)
              local.set 10
              f64.const 0x0p+0 (;=0;)
              local.set 12
              loop ;; label = @6
                local.get 12
                local.get 2
                local.get 5
                i32.add
                local.tee 6
                f64.load
                local.tee 11
                local.get 10
                local.get 10
                local.get 11
                f64.add
                local.tee 9
                f64.sub
                f64.add
                local.get 10
                local.get 11
                local.get 9
                f64.sub
                f64.add
                local.get 10
                f64.abs
                local.get 11
                f64.abs
                f64.ge
                select
                f64.add
                local.get 6
                i32.const 8
                i32.add
                f64.load
                local.tee 10
                local.get 9
                local.get 9
                local.get 10
                f64.add
                local.tee 11
                f64.sub
                f64.add
                local.get 9
                local.get 10
                local.get 11
                f64.sub
                f64.add
                local.get 9
                f64.abs
                local.get 10
                f64.abs
                f64.ge
                select
                f64.add
                local.set 12
                local.get 11
                local.set 10
                local.get 5
                i32.const 16
                i32.add
                local.tee 5
                i32.const 512
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 11
              local.get 12
              f64.add
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 4
                i32.sub
                local.tee 5
                i32.const 8
                local.get 5
                i32.const 8
                i32.lt_u
                select
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 5
                memory.copy
              end
              i32.const 0
              local.get 5
              local.get 4
              i32.add
              local.tee 5
              i32.store offset=1065672
              local.get 1
              local.get 2
              i32.const 64
              call $_RNvCsacnkOpZMulW_9float_sum8pairwise
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 5
                i32.sub
                local.tee 6
                i32.const 8
                local.get 6
                i32.const 8
                i32.lt_u
                select
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 6
                memory.copy
              end
              i32.const 0
              local.get 6
              local.get 5
              i32.add
              local.tee 7
              i32.store offset=1065672
              f64.const 0x0p+0 (;=0;)
              local.set 9
              i32.const -512
              local.set 5
              loop ;; label = @6
                local.get 9
                local.get 2
                local.get 5
                i32.add
                local.tee 6
                i32.const 512
                i32.add
                f64.load
                local.get 3
                local.get 5
                i32.add
                local.tee 4
                i32.const 512
                i32.add
                f64.load
                f64.mul
                f64.add
                local.get 6
                i32.const 520
                i32.add
                f64.load
                local.get 4
                i32.const 520
                i32.add
                f64.load
                f64.mul
                f64.add
                local.get 6
                i32.const 528
                i32.add
                f64.load
                local.get 4
                i32.const 528
                i32.add
                f64.load
                f64.mul
                f64.add
                local.get 6
                i32.const 536
                i32.add
                f64.load
                local.get 4
                i32.const 536
                i32.add
                f64.load
                f64.mul
                f64.add
                local.set 9
                local.get 5
                i32.const 32
                i32.add
                local.tee 5
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 9
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 7
                i32.sub
                local.tee 5
                i32.const 8
                local.get 5
                i32.const 8
                i32.lt_u
                select
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 7
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 5
                memory.copy
              end
              i32.const 0
              local.set 6
              i32.const 0
              local.get 5
              local.get 7
              i32.add
              local.tee 5
              i32.store offset=1065672
              f64.const 0x0p+0 (;=0;)
              local.set 9
              local.get 1
              local.get 3
              f64.load
              f64.const 0x0p+0 (;=0;)
              f64.add
              local.get 3
              f64.load offset=8
              f64.add
              local.get 3
              f64.load offset=16
              f64.add
              local.get 3
              f64.load offset=24
              f64.add
              local.get 3
              f64.load offset=32
              f64.add
              local.get 3
              f64.load offset=40
              f64.add
              local.get 3
              f64.load offset=48
              f64.add
              local.get 3
              f64.load offset=56
              f64.add
              local.get 3
              f64.load offset=64
              f64.add
              local.get 3
              f64.load offset=72
              f64.add
              local.get 3
              f64.load offset=80
              f64.add
              local.get 3
              f64.load offset=88
              f64.add
              local.get 3
              f64.load offset=96
              f64.add
              local.get 3
              f64.load offset=104
              f64.add
              local.get 3
              f64.load offset=112
              f64.add
              local.get 3
              f64.load offset=120
              f64.add
              local.get 3
              f64.load offset=128
              f64.add
              local.get 3
              f64.load offset=136
              f64.add
              local.get 3
              f64.load offset=144
              f64.add
              local.get 3
              f64.load offset=152
              f64.add
              local.get 3
              f64.load offset=160
              f64.add
              local.get 3
              f64.load offset=168
              f64.add
              local.get 3
              f64.load offset=176
              f64.add
              local.get 3
              f64.load offset=184
              f64.add
              local.get 3
              f64.load offset=192
              f64.add
              local.get 3
              f64.load offset=200
              f64.add
              local.get 3
              f64.load offset=208
              f64.add
              local.get 3
              f64.load offset=216
              f64.add
              local.get 3
              f64.load offset=224
              f64.add
              local.get 3
              f64.load offset=232
              f64.add
              local.get 3
              f64.load offset=240
              f64.add
              local.get 3
              f64.load offset=248
              f64.add
              local.get 3
              f64.load offset=256
              f64.add
              local.get 3
              f64.load offset=264
              f64.add
              local.get 3
              f64.load offset=272
              f64.add
              local.get 3
              f64.load offset=280
              f64.add
              local.get 3
              f64.load offset=288
              f64.add
              local.get 3
              f64.load offset=296
              f64.add
              local.get 3
              f64.load offset=304
              f64.add
              local.get 3
              f64.load offset=312
              f64.add
              local.get 3
              f64.load offset=320
              f64.add
              local.get 3
              f64.load offset=328
              f64.add
              local.get 3
              f64.load offset=336
              f64.add
              local.get 3
              f64.load offset=344
              f64.add
              local.get 3
              f64.load offset=352
              f64.add
              local.get 3
              f64.load offset=360
              f64.add
              local.get 3
              f64.load offset=368
              f64.add
              local.get 3
              f64.load offset=376
              f64.add
              local.get 3
              f64.load offset=384
              f64.add
              local.get 3
              f64.load offset=392
              f64.add
              local.get 3
              f64.load offset=400
              f64.add
              local.get 3
              f64.load offset=408
              f64.add
              local.get 3
              f64.load offset=416
              f64.add
              local.get 3
              f64.load offset=424
              f64.add
              local.get 3
              f64.load offset=432
              f64.add
              local.get 3
              f64.load offset=440
              f64.add
              local.get 3
              f64.load offset=448
              f64.add
              local.get 3
              f64.load offset=456
              f64.add
              local.get 3
              f64.load offset=464
              f64.add
              local.get 3
              f64.load offset=472
              f64.add
              local.get 3
              f64.load offset=480
              f64.add
              local.get 3
              f64.load offset=488
              f64.add
              local.get 3
              f64.load offset=496
              f64.add
              local.get 3
              f64.load offset=504
              f64.add
              local.get 2
              f64.load
              f64.const 0x0p+0 (;=0;)
              f64.add
              local.get 2
              f64.load offset=8
              f64.add
              local.get 2
              f64.load offset=16
              f64.add
              local.get 2
              f64.load offset=24
              f64.add
              local.get 2
              f64.load offset=32
              f64.add
              local.get 2
              f64.load offset=40
              f64.add
              local.get 2
              f64.load offset=48
              f64.add
              local.get 2
              f64.load offset=56
              f64.add
              local.get 2
              f64.load offset=64
              f64.add
              local.get 2
              f64.load offset=72
              f64.add
              local.get 2
              f64.load offset=80
              f64.add
              local.get 2
              f64.load offset=88
              f64.add
              local.get 2
              f64.load offset=96
              f64.add
              local.get 2
              f64.load offset=104
              f64.add
              local.get 2
              f64.load offset=112
              f64.add
              local.get 2
              f64.load offset=120
              f64.add
              local.get 2
              f64.load offset=128
              f64.add
              local.get 2
              f64.load offset=136
              f64.add
              local.get 2
              f64.load offset=144
              f64.add
              local.get 2
              f64.load offset=152
              f64.add
              local.get 2
              f64.load offset=160
              f64.add
              local.get 2
              f64.load offset=168
              f64.add
              local.get 2
              f64.load offset=176
              f64.add
              local.get 2
              f64.load offset=184
              f64.add
              local.get 2
              f64.load offset=192
              f64.add
              local.get 2
              f64.load offset=200
              f64.add
              local.get 2
              f64.load offset=208
              f64.add
              local.get 2
              f64.load offset=216
              f64.add
              local.get 2
              f64.load offset=224
              f64.add
              local.get 2
              f64.load offset=232
              f64.add
              local.get 2
              f64.load offset=240
              f64.add
              local.get 2
              f64.load offset=248
              f64.add
              local.get 2
              f64.load offset=256
              f64.add
              local.get 2
              f64.load offset=264
              f64.add
              local.get 2
              f64.load offset=272
              f64.add
              local.get 2
              f64.load offset=280
              f64.add
              local.get 2
              f64.load offset=288
              f64.add
              local.get 2
              f64.load offset=296
              f64.add
              local.get 2
              f64.load offset=304
              f64.add
              local.get 2
              f64.load offset=312
              f64.add
              local.get 2
              f64.load offset=320
              f64.add
              local.get 2
              f64.load offset=328
              f64.add
              local.get 2
              f64.load offset=336
              f64.add
              local.get 2
              f64.load offset=344
              f64.add
              local.get 2
              f64.load offset=352
              f64.add
              local.get 2
              f64.load offset=360
              f64.add
              local.get 2
              f64.load offset=368
              f64.add
              local.get 2
              f64.load offset=376
              f64.add
              local.get 2
              f64.load offset=384
              f64.add
              local.get 2
              f64.load offset=392
              f64.add
              local.get 2
              f64.load offset=400
              f64.add
              local.get 2
              f64.load offset=408
              f64.add
              local.get 2
              f64.load offset=416
              f64.add
              local.get 2
              f64.load offset=424
              f64.add
              local.get 2
              f64.load offset=432
              f64.add
              local.get 2
              f64.load offset=440
              f64.add
              local.get 2
              f64.load offset=448
              f64.add
              local.get 2
              f64.load offset=456
              f64.add
              local.get 2
              f64.load offset=464
              f64.add
              local.get 2
              f64.load offset=472
              f64.add
              local.get 2
              f64.load offset=480
              f64.add
              local.get 2
              f64.load offset=488
              f64.add
              local.get 2
              f64.load offset=496
              f64.add
              local.get 2
              f64.load offset=504
              f64.add
              f64.sub
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 5
                i32.sub
                local.tee 4
                i32.const 8
                local.get 4
                i32.const 8
                i32.lt_u
                select
                local.tee 4
                i32.eqz
                br_if 0 (;@6;)
                local.get 5
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 4
                memory.copy
              end
              i32.const 0
              local.get 4
              local.get 5
              i32.add
              local.tee 4
              i32.store offset=1065672
              f64.const 0x0p+0 (;=0;)
              local.set 10
              loop ;; label = @6
                local.get 9
                local.get 3
                local.get 6
                i32.add
                local.tee 5
                f64.load
                local.get 10
                f64.sub
                local.tee 11
                f64.add
                local.tee 10
                local.get 5
                i32.const 8
                i32.add
                f64.load
                local.get 10
                local.get 9
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 9
                local.get 5
                i32.const 16
                i32.add
                f64.load
                local.get 9
                local.get 10
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 10
                local.get 5
                i32.const 24
                i32.add
                f64.load
                local.get 10
                local.get 9
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 9
                local.get 10
                f64.sub
                local.get 11
                f64.sub
                local.set 10
                local.get 6
                i32.const 32
                i32.add
                local.tee 6
                i32.const 512
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 9
              f64.const 0x1p-1 (;=0.5;)
              f64.mul
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 4
                i32.sub
                local.tee 3
                i32.const 8
                local.get 3
                i32.const 8
                i32.lt_u
                select
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 4
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 5
                memory.copy
              end
              i32.const 0
              local.set 3
              i32.const 0
              local.get 5
              local.get 4
              i32.add
              local.tee 6
              i32.store offset=1065672
              f32.const 0x0p+0 (;=0;)
              local.set 13
              f32.const 0x0p+0 (;=0;)
              local.set 14
              loop ;; label = @6
                local.get 13
                local.get 2
                local.get 3
                i32.add
                local.tee 5
                f64.load
                f32.demote_f64
                local.get 14
                f32.sub
                local.tee 15
                f32.add
                local.tee 14
                local.get 5
                i32.const 8
                i32.add
                f64.load
                f32.demote_f64
                local.get 14
                local.get 13
                f32.sub
                local.get 15
                f32.sub
                f32.sub
                local.tee 15
                f32.add
                local.tee 13
                local.get 14
                f32.sub
                local.get 15
                f32.sub
                local.set 14
                local.get 3
                i32.const 16
                i32.add
                local.tee 3
                i32.const 512
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 13
              f32.store offset=24
              i32.const 0
              i32.const 16384
              local.get 6
              i32.sub
              local.tee 3
              local.get 3
              i32.const 16384
              i32.gt_u
              select
              local.tee 3
              i32.const 4
              local.get 3
              i32.const 4
              i32.lt_u
              select
              local.tee 5
              local.get 6
              i32.add
              local.tee 3
              i32.const 16385
              i32.ge_u
              br_if 3 (;@2;)
              block ;; label = @6
                local.get 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 5
                memory.copy
              end
              local.get 1
              local.get 13
              f32.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 3
                i32.sub
                local.tee 5
                i32.const 4
                local.get 5
                i32.const 4
                i32.lt_u
                select
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 6
                memory.copy
              end
              i32.const 0
              local.set 5
              i32.const 0
              local.get 6
              local.get 3
              i32.add
              local.tee 6
              i32.store offset=1065672
              f64.const 0x0p+0 (;=0;)
              local.set 16
              f64.const 0x0p+0 (;=0;)
              local.set 17
              loop ;; label = @6
                local.get 17
                local.get 2
                local.get 5
                i32.add
                local.tee 3
                f64.load
                local.tee 9
                f64.const 0x1.56e1fc2f8f359p-997 (;=0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;)
                f64.mul
                f64.add
                local.get 3
                i32.const 8
                i32.add
                f64.load
                local.tee 10
                f64.const 0x1.56e1fc2f8f359p-997 (;=0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;)
                f64.mul
                f64.add
                local.get 3
                i32.const 16
                i32.add
                f64.load
                local.tee 11
                f64.const 0x1.56e1fc2f8f359p-997 (;=0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;)
                f64.mul
                f64.add
                local.get 3
                i32.const 24
                i32.add
                f64.load
                local.tee 12
                f64.const 0x1.56e1fc2f8f359p-997 (;=0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001;)
                f64.mul
                f64.add
                local.set 17
                local.get 16
                local.get 9
                local.get 9
                f64.mul
                f64.add
                local.get 10
                local.get 10
                f64.mul
                f64.add
                local.get 11
                local.get 11
                f64.mul
                f64.add
                local.get 12
                local.get 12
                f64.mul
                f64.add
                local.set 16
                local.get 5
                i32.const 32
                i32.add
                local.tee 5
                i32.const 512
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 16
              f64.store offset=24
              i32.const 0
              i32.const 16384
              local.get 6
              i32.sub
              local.tee 3
              local.get 3
              i32.const 16384
              i32.gt_u
              select
              local.tee 3
              i32.const 8
              local.get 3
              i32.const 8
              i32.lt_u
              select
              local.tee 5
              local.get 6
              i32.add
              local.tee 3
              i32.const 16385
              i32.ge_u
              br_if 4 (;@1;)
              block ;; label = @6
                local.get 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 5
                memory.copy
              end
              local.get 1
              local.get 17
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 3
                i32.sub
                local.tee 5
                i32.const 8
                local.get 5
                i32.const 8
                i32.lt_u
                select
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 5
                memory.copy
              end
              local.get 1
              local.get 16
              f64.const 0x1p-6 (;=0.015625;)
              f64.mul
              f64.store offset=24
              block ;; label = @6
                i32.const 16384
                local.get 5
                local.get 3
                i32.add
                local.tee 3
                i32.sub
                local.tee 5
                i32.const 8
                local.get 5
                i32.const 8
                i32.lt_u
                select
                local.tee 6
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.const 1049288
                i32.add
                local.get 1
                i32.const 24
                i32.add
                local.get 6
                memory.copy
              end
              i32.const 0
              local.set 5
              i32.const 0
              local.get 6
              local.get 3
              i32.add
              i32.store offset=1065672
              local.get 2
              f64.load offset=504
              local.set 12
              local.get 2
              f64.load offset=496
              local.set 16
              local.get 2
              f64.load offset=488
              local.set 17
              local.get 2
              f64.load offset=480
              local.set 18
              local.get 2
              f64.load offset=472
              local.set 19
              local.get 2
              f64.load offset=464
              local.set 20
              local.get 2
              f64.load offset=456
              local.set 21
              local.get 2
              f64.load offset=448
              local.set 22
              local.get 2
              f64.load offset=440
              local.set 23
              local.get 2
              f64.load offset=432
              local.set 24
              local.get 2
              f64.load offset=424
              local.set 25
              local.get 2
              f64.load offset=416
              local.set 26
              local.get 2
              f64.load offset=408
              local.set 27
              local.get 2
              f64.load offset=400
              local.set 28
              local.get 2
              f64.load offset=392
              local.set 29
              local.get 2
              f64.load offset=384
              local.set 30
              local.get 2
              f64.load offset=376
              local.set 31
              local.get 2
              f64.load offset=368
              local.set 32
              local.get 2
              f64.load offset=360
              local.set 33
              local.get 2
              f64.load offset=352
              local.set 34
              local.get 2
              f64.load offset=344
              local.set 35
              local.get 2
              f64.load offset=336
              local.set 36
              local.get 2
              f64.load offset=328
              local.set 37
              local.get 2
              f64.load offset=320
              local.set 38
              local.get 2
              f64.load offset=312
              local.set 39
              local.get 2
              f64.load offset=304
              local.set 40
              local.get 2
              f64.load offset=296
              local.set 41
              local.get 2
              f64.load offset=288
              local.set 42
              local.get 2
              f64.load offset=280
              local.set 43
              local.get 2
              f64.load offset=272
              local.set 44
              local.get 2
              f64.load offset=264
              local.set 45
              local.get 2
              f64.load offset=256
              local.set 46
              local.get 2
              f64.load offset=248
              local.set 47
              local.get 2
              f64.load offset=240
              local.set 48
              local.get 2
              f64.load offset=232
              local.set 49
              local.get 2
              f64.load offset=224
              local.set 50
              local.get 2
              f64.load offset=216
              local.set 51
              local.get 2
              f64.load offset=208
              local.set 52
              local.get 2
              f64.load offset=200
              local.set 53
              local.get 2
              f64.load offset=192
              local.set 54
              local.get 2
              f64.load offset=184
              local.set 55
              local.get 2
              f64.load offset=176
              local.set 56
              local.get 2
              f64.load offset=168
              local.set 57
              local.get 2
              f64.load offset=160
              local.set 58
              local.get 2
              f64.load offset=152
              local.set 59
              local.get 2
              f64.load offset=144
              local.set 60
              local.get 2
              f64.load offset=136
              local.set 61
              local.get 2
              f64.load offset=128
              local.set 62
              local.get 2
              f64.load offset=120
              local.set 63
              local.get 2
              f64.load offset=112
              local.set 64
              local.get 2
              f64.load offset=104
              local.set 65
              local.get 2
              f64.load offset=96
              local.set 66
              local.get 2
              f64.load offset=88
              local.set 67
              local.get 2
              f64.load offset=80
              local.set 68
              local.get 2
              f64.load offset=72
              local.set 69
              local.get 2
              f64.load offset=64
              local.set 70
              local.get 2
              f64.load offset=56
              local.set 71
              local.get 2
              f64.load offset=48
              local.set 72
              local.get 2
              f64.load offset=40
              local.set 73
              local.get 2
              f64.load offset=32
              local.set 74
              local.get 2
              f64.load offset=24
              local.set 75
              local.get 2
              f64.load offset=16
              local.set 76
              local.get 2
              f64.load offset=8
              local.set 77
              local.get 2
              f64.load
              local.set 78
              f64.const 0x0p+0 (;=0;)
              local.set 9
              f64.const 0x0p+0 (;=0;)
              local.set 10
              loop ;; label = @6
                local.get 9
                local.get 2
                local.get 5
                i32.add
                local.tee 3
                f64.load
                local.get 10
                f64.sub
                local.tee 11
                f64.add
                local.tee 10
                local.get 3
                i32.const 8
                i32.add
                f64.load
                local.get 10
                local.get 9
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 9
                local.get 3
                i32.const 16
                i32.add
                f64.load
                local.get 9
                local.get 10
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 10
                local.get 3
                i32.const 24
                i32.add
                f64.load
                local.get 10
                local.get 9
                f64.sub
                local.get 11
                f64.sub
                f64.sub
                local.tee 11
                f64.add
                local.tee 9
                local.get 10
                f64.sub
                local.get 11
                f64.sub
                local.set 10
                local.get 5
                i32.const 32
                i32.add
                local.tee 5
                i32.const 512
                i32.ne
                br_if 0 (;@6;)
              end
              local.get 1
              i32.const 32
              i32.add
              global.set $__stack_pointer
              local.get 9
              i64.reinterpret_f64
              i64.const 11
              i64.rotl
              local.get 78
              f64.const 0x0p+0 (;=0;)
              f64.add
              local.get 77
              f64.add
              local.get 76
              f64.add
              local.get 75
              f64.add
              local.get 74
              f64.add
              local.get 73
              f64.add
              local.get 72
              f64.add
              local.get 71
              f64.add
              local.get 70
              f64.add
              local.get 69
              f64.add
              local.get 68
              f64.add
              local.get 67
              f64.add
              local.get 66
              f64.add
              local.get 65
              f64.add
              local.get 64
              f64.add
              local.get 63
              f64.add
              local.get 62
              f64.add
              local.get 61
              f64.add
              local.get 60
              f64.add
              local.get 59
              f64.add
              local.get 58
              f64.add
              local.get 57
              f64.add
              local.get 56
              f64.add
              local.get 55
              f64.add
              local.get 54
              f64.add
              local.get 53
              f64.add
              local.get 52
              f64.add
              local.get 51
              f64.add
              local.get 50
              f64.add
              local.get 49
              f64.add
              local.get 48
              f64.add
              local.get 47
              f64.add
              local.get 46
              f64.add
              local.get 45
              f64.add
              local.get 44
              f64.add
              local.get 43
              f64.add
              local.get 42
              f64.add
              local.get 41
              f64.add
              local.get 40
              f64.add
              local.get 39
              f64.add
              local.get 38
              f64.add
              local.get 37
              f64.add
              local.get 36
              f64.add
              local.get 35
              f64.add
              local.get 34
              f64.add
              local.get 33
              f64.add
              local.get 32
              f64.add
              local.get 31
              f64.add
              local.get 30
              f64.add
              local.get 29
              f64.add
              local.get 28
              f64.add
              local.get 27
              f64.add
              local.get 26
              f64.add
              local.get 25
              f64.add
              local.get 24
              f64.add
              local.get 23
              f64.add
              local.get 22
              f64.add
              local.get 21
              f64.add
              local.get 20
              f64.add
              local.get 19
              f64.add
              local.get 18
              f64.add
              local.get 17
              f64.add
              local.get 16
              f64.add
              local.get 12
              f64.add
              i64.reinterpret_f64
              i64.xor
              return
            end
            i32.const 8
            i32.const 512
            call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
            unreachable
          end
          i32.const 8
          i32.const 512
          call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
          unreachable
        end
        local.get 5
        local.get 4
        i32.const 16384
        i32.const 1049020
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      local.get 6
      local.get 3
      i32.const 16384
      i32.const 1049020
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    local.get 6
    local.get 3
    i32.const 16384
    i32.const 1049020
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )

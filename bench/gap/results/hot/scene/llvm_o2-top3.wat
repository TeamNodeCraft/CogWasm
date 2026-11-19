(func $_RNvCs4gyxU9834UH_5scene9make_node (;4;) (type 6) (param i32 i32 i32 i32)
    (local i32 i32 i64 i64 i64 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    local.get 2
    local.get 2
    i32.load
    local.tee 5
    i32.const 1
    i32.add
    i32.store
    local.get 1
    i64.load
    local.tee 6
    i64.const -5382687378899015554
    i64.add
    local.tee 7
    i64.const 30
    i64.shr_u
    local.get 7
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
    local.set 9
    local.get 6
    i64.const 1663341875487337577
    i64.add
    local.tee 8
    i64.const 30
    i64.shr_u
    local.get 8
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 10
    i64.const 27
    i64.shr_u
    local.get 10
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 10
    i64.const 31
    i64.shr_u
    local.get 10
    i64.xor
    i64.const 100
    i64.rem_u
    local.set 10
    local.get 6
    i64.const 8709371129873690708
    i64.add
    local.tee 11
    i64.const 30
    i64.shr_u
    local.get 11
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 11
    i64.const 27
    i64.shr_u
    local.get 11
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 11
    i64.const 31
    i64.shr_u
    local.get 11
    i64.xor
    i64.const 3
    i64.rem_u
    i64.const 1
    i64.add
    local.set 12
    local.get 6
    i64.const -2691343689449507777
    i64.add
    local.tee 11
    i64.const 30
    i64.shr_u
    local.get 11
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 11
    i64.const 27
    i64.shr_u
    local.get 11
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 11
    i64.const 31
    i64.shr_u
    local.get 11
    i64.xor
    i64.const 21
    i64.rem_u
    i64.const -10
    i64.add
    local.set 13
    local.get 6
    i64.const 4354685564936845354
    i64.add
    local.tee 11
    i64.const 30
    i64.shr_u
    local.get 11
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 11
    i64.const 27
    i64.shr_u
    local.get 11
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 11
    i64.const 31
    i64.shr_u
    local.get 11
    i64.xor
    i64.const 21
    i64.rem_u
    i64.const -10
    i64.add
    local.set 11
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.const 3
                i32.eq
                br_if 0 (;@6;)
                local.get 10
                i64.const 42
                i64.lt_u
                br_if 1 (;@5;)
              end
              block ;; label = @6
                local.get 10
                i64.const 76
                i64.lt_u
                br_if 0 (;@6;)
                local.get 1
                local.get 7
                i64.store
                local.get 6
                i64.const 6018027440424182931
                i64.add
                local.tee 6
                i64.const 30
                i64.shr_u
                local.get 6
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 6
                i64.const 27
                i64.shr_u
                local.get 6
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 6
                i64.const 31
                i64.shr_u
                local.get 6
                i64.xor
                i64.const 5
                i64.rem_u
                local.set 6
                call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                block ;; label = @7
                  block ;; label = @8
                    i32.const 0
                    i32.load8_u offset=1066232
                    br_if 0 (;@8;)
                    block ;; label = @9
                      memory.size
                      i32.const 16
                      i32.shl
                      local.tee 14
                      br_if 0 (;@9;)
                      i32.const 65536
                      local.set 14
                      i32.const 1
                      memory.grow
                      i32.const -1
                      i32.eq
                      br_if 5 (;@4;)
                    end
                    i32.const 0
                    local.get 14
                    i32.store offset=1066228
                    i32.const 0
                    local.get 14
                    i32.store offset=1066224
                    i32.const 0
                    i32.const 1
                    i32.store8 offset=1066232
                    local.get 14
                    i32.const 48
                    i32.or
                    local.set 15
                    local.get 14
                    local.set 3
                    br 1 (;@7;)
                  end
                  i32.const 0
                  i32.load offset=1066224
                  local.tee 3
                  i32.const -56
                  i32.gt_u
                  br_if 3 (;@4;)
                  local.get 3
                  i32.const 7
                  i32.add
                  i32.const -8
                  i32.and
                  local.tee 3
                  i32.const 48
                  i32.add
                  local.set 15
                  i32.const 0
                  i32.load offset=1066228
                  local.set 14
                end
                block ;; label = @7
                  local.get 15
                  local.get 14
                  i32.le_u
                  br_if 0 (;@7;)
                  local.get 15
                  local.get 14
                  i32.sub
                  local.tee 14
                  i32.const 16
                  i32.shr_u
                  local.get 14
                  i32.const 65535
                  i32.and
                  i32.const 0
                  i32.ne
                  i32.add
                  local.tee 14
                  memory.grow
                  local.tee 16
                  i32.const -1
                  i32.eq
                  br_if 3 (;@4;)
                  i32.const 0
                  local.get 16
                  local.get 14
                  i32.add
                  i32.const 16
                  i32.shl
                  i32.store offset=1066228
                end
                i32.const 0
                local.get 15
                i32.store offset=1066224
                local.get 3
                i32.eqz
                br_if 2 (;@4;)
                local.get 3
                local.get 5
                i32.store offset=40
                local.get 3
                local.get 6
                i64.const 1
                i64.add
                i64.store offset=32
                local.get 3
                local.get 9
                i64.store offset=24
                local.get 3
                local.get 12
                i64.store offset=16
                local.get 3
                local.get 13
                i64.store offset=8
                local.get 3
                local.get 11
                i64.store
                i32.const 1049452
                local.set 14
                br 5 (;@1;)
              end
              local.get 1
              local.get 6
              i64.const 6018027440424182931
              i64.add
              local.tee 8
              i64.store
              local.get 6
              i64.const -1028001813962170200
              i64.add
              local.tee 6
              i64.const 30
              i64.shr_u
              local.get 6
              i64.xor
              i64.const -4658895280553007687
              i64.mul
              local.tee 6
              i64.const 27
              i64.shr_u
              local.get 6
              i64.xor
              i64.const -7723592293110705685
              i64.mul
              local.tee 6
              i64.const 31
              i64.shr_u
              local.get 6
              i64.xor
              i64.const 10
              i64.rem_u
              local.set 7
              local.get 8
              i64.const 30
              i64.shr_u
              local.get 8
              i64.xor
              i64.const -4658895280553007687
              i64.mul
              local.tee 6
              i64.const 27
              i64.shr_u
              local.get 6
              i64.xor
              i64.const -7723592293110705685
              i64.mul
              local.tee 6
              i64.const 31
              i64.shr_u
              local.get 6
              i64.xor
              i64.const 7
              i64.rem_u
              local.set 6
              local.get 9
              i64.const 6
              i64.rem_u
              local.set 8
              call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1066232
                  br_if 0 (;@7;)
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 14
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 14
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 5 (;@3;)
                  end
                  i32.const 0
                  local.get 14
                  i32.store offset=1066228
                  i32.const 0
                  local.get 14
                  i32.store offset=1066224
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1066232
                  local.get 14
                  i32.const 48
                  i32.or
                  local.set 15
                  local.get 14
                  local.set 3
                  br 1 (;@6;)
                end
                i32.const 0
                i32.load offset=1066224
                local.tee 3
                i32.const -56
                i32.gt_u
                br_if 3 (;@3;)
                local.get 3
                i32.const 7
                i32.add
                i32.const -8
                i32.and
                local.tee 3
                i32.const 48
                i32.add
                local.set 15
                i32.const 0
                i32.load offset=1066228
                local.set 14
              end
              block ;; label = @6
                local.get 15
                local.get 14
                i32.le_u
                br_if 0 (;@6;)
                local.get 15
                local.get 14
                i32.sub
                local.tee 14
                i32.const 16
                i32.shr_u
                local.get 14
                i32.const 65535
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 14
                memory.grow
                local.tee 16
                i32.const -1
                i32.eq
                br_if 3 (;@3;)
                i32.const 0
                local.get 16
                local.get 14
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1066228
              end
              i32.const 0
              local.get 15
              i32.store offset=1066224
              local.get 3
              i32.eqz
              br_if 2 (;@3;)
              local.get 3
              local.get 8
              i32.wrap_i64
              i32.const 3
              i32.add
              i32.store offset=44
              local.get 3
              local.get 5
              i32.store offset=40
              local.get 3
              local.get 7
              i64.const 1
              i64.add
              i64.store offset=32
              local.get 3
              local.get 6
              i64.const 1
              i64.add
              i64.store offset=24
              local.get 3
              local.get 12
              i64.store offset=16
              local.get 3
              local.get 13
              i64.store offset=8
              local.get 3
              local.get 11
              i64.store
              i32.const 1049496
              local.set 14
              br 4 (;@1;)
            end
            local.get 1
            local.get 8
            i64.store
            local.get 4
            i32.const 0
            i32.store offset=28
            local.get 4
            i64.const 17179869184
            i64.store offset=20 align=4
            local.get 3
            i32.const 1
            i32.add
            local.set 17
            local.get 9
            i64.const 3
            i64.and
            i64.const 1
            i64.add
            local.set 6
            i32.const 4
            local.set 18
            i32.const 4
            local.set 14
            i32.const 1
            local.set 3
            loop ;; label = @5
              local.get 4
              i32.const 8
              i32.add
              local.get 1
              local.get 2
              local.get 17
              call $_RNvCs4gyxU9834UH_5scene9make_node
              local.get 4
              i32.load offset=12
              local.set 15
              local.get 4
              i32.load offset=8
              local.set 16
              block ;; label = @6
                local.get 3
                i32.const -1
                i32.add
                local.get 4
                i32.load offset=20
                i32.ne
                br_if 0 (;@6;)
                local.get 4
                i32.const 20
                i32.add
                call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecINtNtB7_5boxed3BoxDNtCs4gyxU9834UH_5scene4NodeEL_EE8grow_oneB17_
                local.get 4
                i32.load offset=24
                local.set 18
              end
              local.get 18
              local.get 14
              i32.add
              local.tee 19
              local.get 15
              i32.store
              local.get 19
              i32.const -4
              i32.add
              local.get 16
              i32.store
              local.get 14
              i32.const 8
              i32.add
              local.set 14
              local.get 4
              local.get 3
              i32.store offset=28
              local.get 3
              i32.const 1
              i32.add
              local.set 3
              local.get 6
              i64.const -1
              i64.add
              local.tee 6
              i64.eqz
              br_if 3 (;@2;)
              br 0 (;@5;)
            end
          end
          i32.const 8
          i32.const 48
          call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
          unreachable
        end
        i32.const 8
        i32.const 48
        call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
        unreachable
      end
      call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load8_u offset=1066232
            br_if 0 (;@4;)
            block ;; label = @5
              memory.size
              i32.const 16
              i32.shl
              local.tee 14
              br_if 0 (;@5;)
              i32.const 65536
              local.set 14
              i32.const 1
              memory.grow
              i32.const -1
              i32.eq
              br_if 3 (;@2;)
            end
            i32.const 0
            local.get 14
            i32.store offset=1066228
            i32.const 0
            local.get 14
            i32.store offset=1066224
            i32.const 0
            i32.const 1
            i32.store8 offset=1066232
            local.get 14
            i32.const 40
            i32.or
            local.set 15
            local.get 14
            local.set 3
            br 1 (;@3;)
          end
          i32.const 0
          i32.load offset=1066224
          local.tee 3
          i32.const -48
          i32.gt_u
          br_if 1 (;@2;)
          local.get 3
          i32.const 7
          i32.add
          i32.const -8
          i32.and
          local.tee 3
          i32.const 40
          i32.add
          local.set 15
          i32.const 0
          i32.load offset=1066228
          local.set 14
        end
        block ;; label = @3
          local.get 15
          local.get 14
          i32.le_u
          br_if 0 (;@3;)
          local.get 15
          local.get 14
          i32.sub
          local.tee 14
          i32.const 16
          i32.shr_u
          local.get 14
          i32.const 65535
          i32.and
          i32.const 0
          i32.ne
          i32.add
          local.tee 14
          memory.grow
          local.tee 16
          i32.const -1
          i32.eq
          br_if 1 (;@2;)
          i32.const 0
          local.get 16
          local.get 14
          i32.add
          i32.const 16
          i32.shl
          i32.store offset=1066228
        end
        i32.const 0
        local.get 15
        i32.store offset=1066224
        local.get 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        local.get 5
        i32.store offset=24
        local.get 3
        local.get 12
        i64.store offset=16
        local.get 3
        local.get 13
        i64.store offset=8
        local.get 3
        local.get 11
        i64.store
        local.get 3
        local.get 4
        i64.load offset=20 align=4
        i64.store offset=28 align=4
        local.get 3
        local.get 4
        i32.load offset=28
        i32.store offset=36
        i32.const 1049540
        local.set 14
        br 1 (;@1;)
      end
      i32.const 8
      i32.const 40
      call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 0
    local.get 14
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 4
    i32.const 32
    i32.add
    global.set $__stack_pointer
  )

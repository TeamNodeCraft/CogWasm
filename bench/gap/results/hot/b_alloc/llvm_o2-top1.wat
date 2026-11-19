(func $run (;9;) (type 8) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i32 i32 f64 i32 i32 i32 i32 f64 f64 f64)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    i64.const 2000
    i64.rem_u
    local.set 0
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load8_u offset=1065948
                            br_if 0 (;@12;)
                            block ;; label = @13
                              memory.size
                              i32.const 16
                              i32.shl
                              local.tee 2
                              br_if 0 (;@13;)
                              i32.const 65536
                              local.set 2
                              i32.const 1
                              memory.grow
                              i32.const -1
                              i32.eq
                              br_if 3 (;@10;)
                            end
                            i32.const 0
                            local.get 2
                            i32.store offset=1065944
                            i32.const 0
                            local.get 2
                            i32.store offset=1065940
                            i32.const 0
                            i32.const 1
                            i32.store8 offset=1065948
                            local.get 2
                            i32.const 512
                            i32.or
                            local.set 3
                            local.get 2
                            local.set 4
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i32.load offset=1065940
                          local.tee 2
                          i32.const -520
                          i32.gt_u
                          br_if 1 (;@10;)
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
                          i32.load offset=1065944
                          local.set 2
                        end
                        block ;; label = @11
                          local.get 3
                          local.get 2
                          i32.le_u
                          br_if 0 (;@11;)
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
                          br_if 1 (;@10;)
                          i32.const 0
                          local.get 5
                          local.get 2
                          i32.add
                          i32.const 16
                          i32.shl
                          i32.store offset=1065944
                        end
                        i32.const 0
                        local.get 3
                        i32.store offset=1065940
                        local.get 4
                        i32.eqz
                        br_if 0 (;@10;)
                        local.get 0
                        i32.wrap_i64
                        local.tee 6
                        i32.const 1000
                        i32.add
                        local.set 7
                        f64.const 0x0p+0 (;=0;)
                        local.set 8
                        i32.const 3
                        local.set 2
                        i32.const -512
                        local.set 5
                        loop ;; label = @11
                          local.get 4
                          local.get 5
                          i32.add
                          local.tee 3
                          i32.const 536
                          i32.add
                          local.get 2
                          f64.convert_i32_u
                          f64.const 0x1p-2 (;=0.25;)
                          f64.mul
                          f64.store
                          local.get 3
                          i32.const 512
                          i32.add
                          local.get 8
                          f64.const 0x1p-2 (;=0.25;)
                          f64.mul
                          f64.store
                          local.get 3
                          i32.const 528
                          i32.add
                          local.get 2
                          i32.const -1
                          i32.add
                          f64.convert_i32_u
                          f64.const 0x1p-2 (;=0.25;)
                          f64.mul
                          f64.store
                          local.get 3
                          i32.const 520
                          i32.add
                          local.get 2
                          i32.const -2
                          i32.add
                          f64.convert_i32_u
                          f64.const 0x1p-2 (;=0.25;)
                          f64.mul
                          f64.store
                          local.get 2
                          i32.const 4
                          i32.add
                          local.set 2
                          local.get 8
                          f64.const 0x1p+2 (;=4;)
                          f64.add
                          local.set 8
                          local.get 5
                          i32.const 32
                          i32.add
                          local.tee 5
                          br_if 0 (;@11;)
                        end
                        call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load8_u offset=1065948
                            br_if 0 (;@12;)
                            block ;; label = @13
                              memory.size
                              i32.const 16
                              i32.shl
                              local.tee 3
                              br_if 0 (;@13;)
                              i32.const 65536
                              local.set 3
                              i32.const 1
                              memory.grow
                              i32.const -1
                              i32.eq
                              br_if 4 (;@9;)
                            end
                            i32.const 0
                            local.get 3
                            i32.store offset=1065944
                            i32.const 0
                            local.get 3
                            i32.store offset=1065940
                            i32.const 0
                            i32.const 1
                            i32.store8 offset=1065948
                            local.get 3
                            i32.const 7
                            i32.or
                            local.set 5
                            local.get 3
                            local.set 2
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i32.load offset=1065940
                          local.tee 2
                          i32.const -8
                          i32.gt_u
                          br_if 2 (;@9;)
                          local.get 2
                          i32.const 7
                          i32.add
                          local.set 5
                          i32.const 0
                          i32.load offset=1065944
                          local.set 3
                        end
                        block ;; label = @11
                          local.get 5
                          local.get 3
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 5
                          local.get 3
                          i32.sub
                          local.tee 3
                          i32.const 16
                          i32.shr_u
                          local.get 3
                          i32.const 65535
                          i32.and
                          i32.const 0
                          i32.ne
                          i32.add
                          local.tee 3
                          memory.grow
                          local.tee 9
                          i32.const -1
                          i32.eq
                          br_if 2 (;@9;)
                          i32.const 0
                          local.get 9
                          local.get 3
                          i32.add
                          i32.const 16
                          i32.shl
                          i32.store offset=1065944
                        end
                        i32.const 0
                        local.get 5
                        i32.store offset=1065940
                        local.get 2
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 2
                        i32.const 0
                        i32.load offset=1049127 align=1
                        i32.store offset=3 align=1
                        local.get 2
                        i32.const 0
                        i32.load offset=1049124 align=1
                        i32.store align=1
                        local.get 1
                        i32.const 7
                        i32.store offset=20
                        local.get 1
                        local.get 2
                        i32.store offset=16
                        local.get 1
                        i32.const 7
                        i32.store offset=12
                        local.get 1
                        i32.const 1
                        i64.extend_i32_u
                        i64.const 32
                        i64.shl
                        local.get 1
                        i32.const 12
                        i32.add
                        i64.extend_i32_u
                        i64.or
                        i64.store offset=40
                        local.get 1
                        i32.const 24
                        i32.add
                        i32.const 1049060
                        local.get 1
                        i32.const 40
                        i32.add
                        call $_RNvNvNtCseFzb18zECBI_5alloc3fmt6format12format_inner
                        local.get 1
                        i32.load offset=32
                        local.set 10
                        call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                        block ;; label = @11
                          block ;; label = @12
                            i32.const 0
                            i32.load8_u offset=1065948
                            br_if 0 (;@12;)
                            block ;; label = @13
                              memory.size
                              i32.const 16
                              i32.shl
                              local.tee 3
                              br_if 0 (;@13;)
                              i32.const 65536
                              local.set 3
                              i32.const 1
                              memory.grow
                              i32.const -1
                              i32.eq
                              br_if 5 (;@8;)
                            end
                            i32.const 0
                            local.get 3
                            i32.store offset=1065944
                            i32.const 0
                            local.get 3
                            i32.store offset=1065940
                            i32.const 0
                            i32.const 1
                            i32.store8 offset=1065948
                            local.get 3
                            i32.const 16
                            i32.or
                            local.set 5
                            local.get 3
                            local.set 2
                            br 1 (;@11;)
                          end
                          i32.const 0
                          i32.load offset=1065940
                          local.tee 2
                          i32.const -20
                          i32.gt_u
                          br_if 3 (;@8;)
                          local.get 2
                          i32.const 3
                          i32.add
                          i32.const -4
                          i32.and
                          local.tee 2
                          i32.const 16
                          i32.add
                          local.set 5
                          i32.const 0
                          i32.load offset=1065944
                          local.set 3
                        end
                        block ;; label = @11
                          local.get 5
                          local.get 3
                          i32.le_u
                          br_if 0 (;@11;)
                          local.get 5
                          local.get 3
                          i32.sub
                          local.tee 3
                          i32.const 16
                          i32.shr_u
                          local.get 3
                          i32.const 65535
                          i32.and
                          i32.const 0
                          i32.ne
                          i32.add
                          local.tee 3
                          memory.grow
                          local.tee 9
                          i32.const -1
                          i32.eq
                          br_if 3 (;@8;)
                          i32.const 0
                          local.get 9
                          local.get 3
                          i32.add
                          i32.const 16
                          i32.shl
                          i32.store offset=1065944
                        end
                        i32.const 0
                        local.get 5
                        i32.store offset=1065940
                        local.get 2
                        i32.eqz
                        br_if 2 (;@8;)
                        call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                        i32.const 0
                        i32.load8_u offset=1065948
                        br_if 3 (;@7;)
                        block ;; label = @11
                          memory.size
                          i32.const 16
                          i32.shl
                          local.tee 3
                          br_if 0 (;@11;)
                          i32.const 65536
                          local.set 3
                          i32.const 1
                          memory.grow
                          i32.const -1
                          i32.eq
                          br_if 6 (;@5;)
                        end
                        i32.const 0
                        local.get 3
                        i32.store offset=1065944
                        i32.const 0
                        local.get 3
                        i32.store offset=1065940
                        i32.const 0
                        i32.const 1
                        i32.store8 offset=1065948
                        br 4 (;@6;)
                      end
                      i32.const 8
                      i32.const 512
                      call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
                      unreachable
                    end
                    i32.const 1
                    i32.const 7
                    call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
                    unreachable
                  end
                  i32.const 4
                  i32.const 16
                  call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
                  unreachable
                end
                i32.const 0
                i32.load offset=1065940
                local.tee 3
                i32.const -8
                i32.gt_u
                br_if 1 (;@5;)
              end
              local.get 3
              i32.const 7
              i32.add
              i32.const -8
              i32.and
              local.tee 3
              i32.const 8
              i32.add
              local.tee 5
              i32.eqz
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 5
                i32.const 0
                i32.load offset=1065944
                local.tee 9
                i32.le_u
                br_if 0 (;@6;)
                local.get 5
                local.get 9
                i32.sub
                local.tee 9
                i32.const 16
                i32.shr_u
                local.get 9
                i32.const 65535
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 9
                memory.grow
                local.tee 11
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                i32.const 0
                local.get 11
                local.get 9
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1065944
              end
              i32.const 0
              local.get 5
              i32.store offset=1065940
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i64.const 4609434218613702656
              i64.store
              call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
              i32.const 0
              i32.load8_u offset=1065948
              br_if 1 (;@4;)
              block ;; label = @6
                memory.size
                i32.const 16
                i32.shl
                local.tee 5
                br_if 0 (;@6;)
                i32.const 65536
                local.set 5
                i32.const 1
                memory.grow
                i32.const -1
                i32.eq
                br_if 4 (;@2;)
              end
              i32.const 0
              local.get 5
              i32.store offset=1065944
              i32.const 0
              local.get 5
              i32.store offset=1065940
              i32.const 0
              i32.const 1
              i32.store8 offset=1065948
              br 2 (;@3;)
            end
            i32.const 8
            i32.const 8
            call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
            unreachable
          end
          i32.const 0
          i32.load offset=1065940
          local.tee 5
          i32.const -8
          i32.gt_u
          br_if 1 (;@2;)
        end
        local.get 5
        i32.const 7
        i32.add
        i32.const -8
        i32.and
        local.tee 5
        i32.const 8
        i32.add
        local.tee 9
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 9
          i32.const 0
          i32.load offset=1065944
          local.tee 11
          i32.le_u
          br_if 0 (;@3;)
          local.get 9
          local.get 11
          i32.sub
          local.tee 11
          i32.const 16
          i32.shr_u
          local.get 11
          i32.const 65535
          i32.and
          i32.const 0
          i32.ne
          i32.add
          local.tee 11
          memory.grow
          local.tee 12
          i32.const -1
          i32.eq
          br_if 1 (;@2;)
          i32.const 0
          local.get 12
          local.get 11
          i32.add
          i32.const 16
          i32.shl
          i32.store offset=1065944
        end
        i32.const 0
        local.get 9
        i32.store offset=1065940
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 10
        f64.convert_i32_u
        local.set 13
        local.get 5
        i64.const 4602678819172646912
        i64.store
        local.get 2
        i32.const 1049132
        i32.store offset=12
        local.get 2
        local.get 5
        i32.store offset=8
        local.get 2
        i32.const 1049132
        i32.store offset=4
        local.get 2
        local.get 3
        i32.store
        f64.const 0x0p+0 (;=0;)
        local.set 14
        i32.const -1000
        local.set 3
        f64.const 0x1p+0 (;=1;)
        local.set 8
        loop ;; label = @3
          local.get 2
          i32.load
          local.get 4
          local.get 3
          i32.const 1001
          i32.add
          i32.const 63
          i32.and
          i32.const 3
          i32.shl
          i32.add
          f64.load
          local.get 8
          f64.const 0x1.0624dd2f1a9fcp-10 (;=0.001;)
          f64.mul
          f64.add
          local.get 13
          f64.add
          local.get 2
          i32.load offset=4
          i32.load offset=12
          call_indirect (type 0)
          local.set 15
          local.get 14
          local.get 2
          i32.load offset=8
          local.get 15
          local.get 2
          i32.load offset=12
          i32.load offset=12
          call_indirect (type 0)
          local.tee 15
          local.get 15
          f64.abs
          f64.const 0x1p-1 (;=0.5;)
          f64.mul
          f64.add
          f64.const 0x1.ff7ced916872bp-1 (;=0.999;)
          f64.mul
          f64.add
          local.set 14
          local.get 8
          f64.const 0x1p+0 (;=1;)
          f64.add
          local.set 8
          local.get 6
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          i32.ne
          br_if 0 (;@3;)
        end
        block ;; label = @3
          local.get 2
          i32.load offset=4
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.load
          local.get 3
          call_indirect (type 1)
        end
        block ;; label = @3
          local.get 2
          i32.load offset=12
          i32.load
          local.tee 3
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=8
          local.get 3
          call_indirect (type 1)
        end
        local.get 1
        local.get 14
        local.get 7
        f64.convert_i32_u
        f64.div
        local.tee 8
        f64.store offset=24
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1065936
        local.tee 2
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
        local.get 2
        i32.add
        local.tee 3
        local.get 2
        i32.lt_u
        br_if 1 (;@1;)
        local.get 3
        i32.const 16385
        i32.ge_u
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 2
          i32.const 1049552
          i32.add
          local.get 1
          i32.const 24
          i32.add
          local.get 5
          memory.copy
        end
        i32.const 0
        local.get 3
        i32.store offset=1065936
        local.get 1
        i32.const 48
        i32.add
        global.set $__stack_pointer
        local.get 8
        i64.reinterpret_f64
        return
      end
      i32.const 8
      i32.const 8
      call $_RNvNtCseFzb18zECBI_5alloc5alloc18handle_alloc_error
      unreachable
    end
    local.get 2
    local.get 3
    i32.const 16384
    i32.const 1049108
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )

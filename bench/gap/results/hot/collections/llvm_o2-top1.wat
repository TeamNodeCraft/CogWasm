(func $run (;45;) (type 17) (param i64) (result i64)
    (local i32 i64 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i64 i32 i64)
    global.get $__stack_pointer
    i32.const 272
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 0
    local.get 0
    i64.const 65
    i64.div_u
    local.tee 2
    i64.const 65
    i64.mul
    i64.sub
    i32.wrap_i64
    local.tee 3
    i32.const 96
    i32.add
    local.tee 4
    i32.const 3
    i32.shl
    local.set 5
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              i32.const 0
              i32.load8_u offset=1067876
              br_if 0 (;@5;)
              block ;; label = @6
                memory.size
                i32.const 16
                i32.shl
                local.tee 6
                br_if 0 (;@6;)
                i32.const 65536
                local.set 6
                i32.const 1
                memory.grow
                i32.const -1
                i32.eq
                br_if 3 (;@3;)
              end
              i32.const 0
              local.get 6
              i32.store offset=1067872
              i32.const 0
              local.get 6
              i32.store offset=1067868
              i32.const 0
              i32.const 1
              i32.store8 offset=1067876
              br 1 (;@4;)
            end
            i32.const 0
            i32.load offset=1067868
            local.tee 6
            i32.const -8
            i32.gt_u
            br_if 1 (;@3;)
          end
          local.get 6
          i32.const 7
          i32.add
          i32.const -8
          i32.and
          local.tee 7
          local.get 5
          i32.add
          local.tee 6
          local.get 7
          i32.lt_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 6
            i32.const 0
            i32.load offset=1067872
            local.tee 8
            i32.le_u
            br_if 0 (;@4;)
            local.get 6
            local.get 8
            i32.sub
            local.tee 8
            i32.const 16
            i32.shr_u
            local.get 8
            i32.const 65535
            i32.and
            i32.const 0
            i32.ne
            i32.add
            local.tee 8
            memory.grow
            local.tee 9
            i32.const -1
            i32.eq
            br_if 1 (;@3;)
            i32.const 0
            local.get 9
            local.get 8
            i32.add
            i32.const 16
            i32.shl
            i32.store offset=1067872
          end
          i32.const 0
          local.get 6
          i32.store offset=1067868
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 6
          local.get 1
          i32.const 0
          i32.store offset=32
          local.get 1
          local.get 7
          i32.store offset=28
          local.get 0
          i64.const -7046029254386353131
          i64.xor
          i64.const -7046029254386353131
          i64.add
          local.tee 10
          i64.const 30
          i64.shr_u
          local.get 10
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
          i64.const -2691343689449507777
          i64.add
          local.set 11
          local.get 1
          local.get 4
          i32.store offset=24
          i64.const 0
          local.set 10
          local.get 4
          i64.extend_i32_u
          local.tee 12
          local.set 13
          local.get 4
          local.set 5
          i32.const 0
          local.set 9
          loop ;; label = @4
            local.get 11
            i64.const 7046029254386353131
            i64.add
            local.tee 14
            i64.const 30
            i64.shr_u
            local.get 14
            i64.xor
            i64.const -4658895280553007687
            i64.mul
            local.tee 14
            i64.const 27
            i64.shr_u
            local.get 14
            i64.xor
            i64.const -7723592293110705685
            i64.mul
            local.tee 14
            i64.const 31
            i64.shr_u
            local.get 14
            i64.xor
            i64.const 10000
            i64.rem_u
            local.get 10
            i64.xor
            local.set 14
            block ;; label = @5
              local.get 9
              local.get 5
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 24
              i32.add
              call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecyE8grow_oneCsbgrmSDC215G_11collections
              local.get 1
              i32.load offset=28
              local.set 7
              local.get 1
              i32.load offset=24
              local.set 5
            end
            local.get 7
            local.get 6
            i32.add
            local.get 14
            i64.store
            local.get 1
            local.get 9
            i32.const 1
            i32.add
            local.tee 9
            i32.store offset=32
            local.get 11
            i64.const -7046029254386353131
            i64.add
            local.set 11
            local.get 10
            i64.const 17
            i64.add
            local.set 10
            local.get 6
            i32.const 8
            i32.add
            local.set 6
            local.get 13
            i64.const -1
            i64.add
            local.tee 13
            i64.eqz
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 1
          i32.const 36
          i32.add
          local.get 1
          i32.const 24
          i32.add
          call $_RNvXsb_NtCseFzb18zECBI_5alloc3vecINtB5_3VecyENtNtCs8euEcNjxCEf_4core5clone5Clone5cloneCsbgrmSDC215G_11collections
          local.get 1
          i32.load offset=40
          local.set 15
          local.get 1
          i32.load offset=44
          local.tee 16
          i32.const 2
          i32.lt_u
          br_if 2 (;@1;)
          local.get 16
          i32.const 21
          i32.lt_u
          br_if 1 (;@2;)
          local.get 15
          local.get 16
          local.get 1
          i32.const 271
          i32.add
          call $_RINvNtNtNtCs8euEcNjxCEf_4core5slice4sort6stable14driftsort_mainyNvYyNtNtB8_3cmp10PartialOrd2ltINtNtCseFzb18zECBI_5alloc3vec3VecyEECsbgrmSDC215G_11collections
          br 2 (;@1;)
        end
        i32.const 8
        local.get 5
        call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
        unreachable
      end
      local.get 15
      local.get 16
      i32.const 1
      local.get 1
      i32.const 271
      i32.add
      call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftyNvYyNtNtBa_3cmp10PartialOrd2ltECsbgrmSDC215G_11collections
    end
    local.get 1
    i32.const 48
    i32.add
    local.get 1
    i32.const 24
    i32.add
    call $_RNvXsb_NtCseFzb18zECBI_5alloc3vecINtB5_3VecyENtNtCs8euEcNjxCEf_4core5clone5Clone5cloneCsbgrmSDC215G_11collections
    local.get 1
    i32.load offset=52
    local.set 8
    block ;; label = @1
      local.get 1
      i32.load offset=56
      local.tee 5
      i32.const 2
      i32.lt_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 5
        i32.const 21
        i32.lt_u
        br_if 0 (;@2;)
        local.get 8
        local.get 5
        local.get 1
        i32.const 271
        i32.add
        call $_RINvNtNtNtCs8euEcNjxCEf_4core5slice4sort8unstable7ipnsortyNvYyNtNtB8_3cmp10PartialOrd2ltECsbgrmSDC215G_11collections
        br 1 (;@1;)
      end
      local.get 8
      local.get 5
      i32.const 1
      local.get 1
      i32.const 271
      i32.add
      call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftyNvYyNtNtBa_3cmp10PartialOrd2ltECsbgrmSDC215G_11collections
    end
    i32.const 0
    local.set 7
    block ;; label = @1
      local.get 16
      local.get 5
      i32.ne
      br_if 0 (;@1;)
      local.get 15
      local.get 8
      local.get 16
      i32.const 3
      i32.shl
      call $memcmp
      i32.eqz
      local.set 7
    end
    local.get 1
    local.get 7
    i32.store8 offset=136
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load offset=1067864
            local.tee 7
            local.get 7
            i32.const 16384
            i32.lt_u
            local.tee 5
            i32.add
            local.tee 8
            local.get 7
            i32.lt_u
            br_if 0 (;@4;)
            local.get 8
            i32.const 16385
            i32.ge_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 5
              i32.eqz
              br_if 0 (;@5;)
              local.get 7
              i32.const 1051480
              i32.add
              local.get 1
              i32.const 136
              i32.add
              local.get 5
              memory.copy
            end
            i32.const 0
            local.set 5
            i32.const 0
            local.get 8
            i32.store offset=1067864
            local.get 1
            i32.const 60
            i32.add
            local.get 1
            i32.const 36
            i32.add
            call $_RNvXsb_NtCseFzb18zECBI_5alloc3vecINtB5_3VecyENtNtCs8euEcNjxCEf_4core5clone5Clone5cloneCsbgrmSDC215G_11collections
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.load offset=68
                local.tee 17
                i32.const 2
                i32.ge_u
                br_if 0 (;@6;)
                local.get 17
                local.set 18
                br 1 (;@5;)
              end
              local.get 1
              i32.load offset=64
              local.tee 19
              local.set 7
              block ;; label = @6
                loop ;; label = @7
                  local.get 7
                  i32.const 8
                  i32.add
                  local.tee 8
                  i64.load
                  local.get 7
                  i64.load
                  i64.eq
                  br_if 1 (;@6;)
                  local.get 8
                  local.set 7
                  local.get 17
                  local.get 5
                  i32.const -1
                  i32.add
                  local.tee 5
                  i32.add
                  i32.const 1
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 17
                local.set 18
                br 1 (;@5;)
              end
              i32.const 1
              local.get 5
              i32.sub
              local.set 18
              block ;; label = @6
                i32.const 2
                local.get 5
                i32.sub
                local.tee 8
                local.get 17
                i32.ge_u
                br_if 0 (;@6;)
                local.get 17
                local.get 5
                i32.add
                local.tee 20
                i32.const -3
                i32.add
                local.set 21
                block ;; label = @7
                  local.get 20
                  i32.const 1
                  i32.and
                  i32.eqz
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 7
                    i32.const 16
                    i32.add
                    i64.load
                    local.tee 10
                    local.get 7
                    i64.load
                    i64.eq
                    br_if 0 (;@8;)
                    local.get 7
                    i32.const 8
                    i32.add
                    local.get 10
                    i64.store
                    i32.const 2
                    local.get 5
                    i32.sub
                    local.set 18
                  end
                  i32.const 3
                  local.get 5
                  i32.sub
                  local.set 8
                end
                local.get 21
                i32.eqz
                br_if 0 (;@6;)
                local.get 17
                local.get 8
                i32.sub
                local.set 5
                local.get 19
                local.get 8
                i32.const 3
                i32.shl
                i32.add
                local.set 7
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    i64.load
                    local.tee 10
                    local.get 19
                    local.get 18
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 8
                    i32.const -8
                    i32.add
                    i64.load
                    i64.eq
                    br_if 0 (;@8;)
                    local.get 8
                    local.get 10
                    i64.store
                    local.get 18
                    i32.const 1
                    i32.add
                    local.set 18
                  end
                  block ;; label = @8
                    local.get 7
                    i32.const 8
                    i32.add
                    i64.load
                    local.tee 10
                    local.get 19
                    local.get 18
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 8
                    i32.const -8
                    i32.add
                    i64.load
                    i64.eq
                    br_if 0 (;@8;)
                    local.get 8
                    local.get 10
                    i64.store
                    local.get 18
                    i32.const 1
                    i32.add
                    local.set 18
                  end
                  local.get 7
                  i32.const 16
                  i32.add
                  local.set 7
                  local.get 5
                  i32.const -2
                  i32.add
                  local.tee 5
                  br_if 0 (;@7;)
                end
              end
              local.get 1
              local.get 18
              i32.store offset=68
            end
            local.get 15
            local.get 16
            i32.const 3
            i32.shl
            local.tee 22
            i32.add
            local.set 19
            local.get 16
            i32.eqz
            br_if 1 (;@3;)
            local.get 15
            i64.load
            local.set 23
            block ;; label = @5
              local.get 16
              i32.const 1
              i32.ne
              br_if 0 (;@5;)
              local.get 23
              local.set 24
              local.get 23
              local.set 25
              br 4 (;@1;)
            end
            block ;; label = @5
              block ;; label = @6
                local.get 16
                i32.const 1
                i32.and
                i32.eqz
                br_if 0 (;@6;)
                local.get 15
                i32.const 8
                i32.add
                local.set 7
                local.get 23
                local.set 24
                br 1 (;@5;)
              end
              local.get 15
              i64.load offset=8
              local.tee 10
              local.get 23
              local.get 10
              local.get 23
              i64.gt_u
              select
              local.set 24
              local.get 10
              local.get 23
              local.get 10
              local.get 23
              i64.lt_u
              select
              local.set 23
              local.get 15
              i32.const 16
              i32.add
              local.set 7
            end
            block ;; label = @5
              local.get 22
              i32.const 16
              i32.eq
              br_if 0 (;@5;)
              local.get 23
              local.set 14
              local.get 24
              local.set 26
              loop ;; label = @6
                local.get 7
                i32.const 8
                i32.add
                i64.load
                local.tee 10
                local.get 7
                i64.load
                local.tee 13
                local.get 24
                local.get 13
                local.get 26
                i64.gt_u
                local.tee 5
                select
                local.get 10
                local.get 13
                local.get 26
                local.get 5
                select
                local.tee 26
                i64.gt_u
                local.tee 5
                select
                local.set 24
                local.get 10
                local.get 26
                local.get 5
                select
                local.set 26
                local.get 10
                local.get 13
                local.get 23
                local.get 13
                local.get 14
                i64.lt_u
                local.tee 5
                select
                local.get 10
                local.get 13
                local.get 14
                local.get 5
                select
                local.tee 13
                i64.lt_u
                local.tee 5
                select
                local.set 23
                local.get 10
                local.get 13
                local.get 5
                select
                local.set 14
                local.get 7
                i32.const 16
                i32.add
                local.tee 7
                local.get 19
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 16
            i32.const 1
            i32.shr_u
            local.set 7
            block ;; label = @5
              local.get 16
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 15
              local.get 7
              i32.const 3
              i32.shl
              i32.add
              i64.load
              local.set 25
              br 4 (;@1;)
            end
            local.get 7
            i32.const -1
            i32.add
            local.tee 5
            local.get 16
            i32.ge_u
            br_if 2 (;@2;)
            local.get 15
            local.get 7
            i32.const 3
            i32.shl
            i32.add
            i64.load
            local.get 15
            local.get 5
            i32.const 3
            i32.shl
            i32.add
            i64.load
            i64.add
            i64.const 1
            i64.shr_u
            local.set 25
            br 3 (;@1;)
          end
          local.get 7
          local.get 8
          i32.const 16384
          i32.const 1050608
          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
          unreachable
        end
        i64.const 0
        local.set 24
        i64.const 0
        local.set 23
        i64.const 0
        local.set 25
        br 1 (;@1;)
      end
      local.get 5
      local.get 16
      i32.const 1050624
      call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
      unreachable
    end
    local.get 1
    local.get 23
    i64.store offset=136
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.const 16384
          i32.const 0
          i32.load offset=1067864
          local.tee 5
          i32.sub
          local.tee 7
          local.get 7
          i32.const 16384
          i32.gt_u
          select
          local.tee 7
          i32.const 8
          local.get 7
          i32.const 8
          i32.lt_u
          select
          local.tee 8
          local.get 5
          i32.add
          local.tee 7
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
          local.get 7
          i32.const 16385
          i32.ge_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 5
            i32.const 1051480
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 24
          i64.store offset=136
          block ;; label = @4
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
            br_if 0 (;@4;)
            local.get 7
            i32.const 1051480
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 5
            memory.copy
          end
          local.get 1
          local.get 25
          i64.store offset=136
          block ;; label = @4
            i32.const 16384
            local.get 5
            local.get 7
            i32.add
            local.tee 7
            i32.sub
            local.tee 5
            i32.const 8
            local.get 5
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.const 1051480
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 8
            memory.copy
          end
          i32.const 0
          local.set 5
          i32.const 0
          local.get 8
          local.get 7
          i32.add
          local.tee 17
          i32.store offset=1067864
          i64.const 0
          local.set 10
          block ;; label = @4
            local.get 16
            i32.eqz
            br_if 0 (;@4;)
            local.get 22
            i32.const -8
            i32.add
            local.tee 8
            i32.const 3
            i32.shr_u
            i32.const 1
            i32.add
            local.tee 21
            i32.const 3
            i32.and
            local.set 20
            local.get 15
            local.set 7
            block ;; label = @5
              block ;; label = @6
                local.get 8
                i32.const 24
                i32.lt_u
                br_if 0 (;@6;)
                local.get 21
                i32.const 1073741820
                i32.and
                local.set 8
                i32.const 0
                local.set 5
                local.get 15
                local.set 7
                loop ;; label = @7
                  local.get 5
                  local.get 7
                  i64.load
                  local.get 25
                  i64.ge_u
                  i32.add
                  local.get 7
                  i32.const 8
                  i32.add
                  i64.load
                  local.get 25
                  i64.ge_u
                  i32.add
                  local.get 7
                  i32.const 16
                  i32.add
                  i64.load
                  local.get 25
                  i64.ge_u
                  i32.add
                  local.get 7
                  i32.const 24
                  i32.add
                  i64.load
                  local.get 25
                  i64.ge_u
                  i32.add
                  local.set 5
                  local.get 7
                  i32.const 32
                  i32.add
                  local.set 7
                  local.get 8
                  i32.const -4
                  i32.add
                  local.tee 8
                  br_if 0 (;@7;)
                end
                local.get 20
                i32.eqz
                br_if 1 (;@5;)
              end
              local.get 20
              i32.const 3
              i32.shl
              local.set 8
              loop ;; label = @6
                local.get 5
                local.get 7
                i64.load
                local.get 25
                i64.ge_u
                i32.add
                local.set 5
                local.get 7
                i32.const 8
                i32.add
                local.set 7
                local.get 8
                i32.const -8
                i32.add
                local.tee 8
                br_if 0 (;@6;)
              end
            end
            local.get 5
            i64.extend_i32_u
            local.set 10
          end
          local.get 1
          local.get 10
          i64.store offset=136
          block ;; label = @4
            i32.const 16384
            local.get 17
            i32.sub
            local.tee 7
            i32.const 8
            local.get 7
            i32.const 8
            i32.lt_u
            select
            local.tee 7
            i32.eqz
            br_if 0 (;@4;)
            local.get 17
            i32.const 1051480
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 7
            memory.copy
          end
          local.get 1
          local.get 18
          i64.extend_i32_u
          i64.store offset=136
          block ;; label = @4
            i32.const 16384
            local.get 7
            local.get 17
            i32.add
            local.tee 7
            i32.sub
            local.tee 5
            i32.const 8
            local.get 5
            i32.const 8
            i32.lt_u
            select
            local.tee 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.const 1051480
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 5
            memory.copy
          end
          i32.const 0
          local.get 5
          local.get 7
          i32.add
          i32.store offset=1067864
          local.get 1
          i32.const 72
          i32.add
          local.get 1
          i32.const 24
          i32.add
          call $_RNvXsb_NtCseFzb18zECBI_5alloc3vecINtB5_3VecyENtNtCs8euEcNjxCEf_4core5clone5Clone5cloneCsbgrmSDC215G_11collections
          local.get 1
          i32.load offset=80
          local.set 7
          local.get 1
          i32.load offset=76
          local.set 5
          local.get 1
          local.get 1
          i32.const 271
          i32.add
          i32.store offset=136
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 7
                i32.const 2
                i32.lt_u
                br_if 0 (;@6;)
                local.get 7
                i32.const 21
                i32.lt_u
                br_if 1 (;@5;)
                local.get 5
                local.get 7
                local.get 1
                i32.const 136
                i32.add
                call $_RINvNtNtNtCs8euEcNjxCEf_4core5slice4sort8unstable7ipnsortyNCINvMB6_Sy16sort_unstable_byNCNvCsbgrmSDC215G_11collections4body0E0EB1r_
                br 2 (;@4;)
              end
              local.get 7
              br_if 1 (;@4;)
              local.get 16
              i32.eqz
              local.get 15
              local.get 16
              i32.const 3
              i32.shl
              i32.add
              i32.const 8
              i32.eq
              i32.or
              local.set 7
              br 4 (;@1;)
            end
            local.get 5
            local.get 7
            i32.const 1
            local.get 1
            i32.const 136
            i32.add
            call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftyNCINvMB8_Sy16sort_unstable_byNCNvCsbgrmSDC215G_11collections4body0E0EB1U_
          end
          i32.const 0
          local.set 7
          local.get 16
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 5
        local.get 7
        i32.const 16384
        i32.const 1050608
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      local.get 5
      i64.load
      local.get 15
      local.get 16
      i32.const 3
      i32.shl
      i32.add
      i32.const -8
      i32.add
      i64.load
      i64.eq
      local.set 7
    end
    local.get 1
    local.get 7
    i32.store8 offset=136
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load offset=1067864
          local.tee 7
          local.get 7
          i32.const 16384
          i32.lt_u
          local.tee 5
          i32.add
          local.tee 20
          local.get 7
          i32.lt_u
          br_if 0 (;@3;)
          local.get 20
          i32.const 16385
          i32.ge_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 7
            i32.const 1051480
            i32.add
            local.get 1
            i32.const 136
            i32.add
            local.get 5
            memory.copy
          end
          i32.const 0
          local.set 7
          i32.const 0
          local.get 20
          i32.store offset=1067864
          block ;; label = @4
            block ;; label = @5
              local.get 4
              i32.const 1
              i32.shr_u
              local.tee 5
              local.get 16
              i32.ge_u
              br_if 0 (;@5;)
              local.get 15
              local.get 5
              i32.const 3
              i32.shl
              i32.add
              i64.load
              local.set 10
              local.get 16
              local.set 5
              loop ;; label = @6
                local.get 7
                local.get 5
                i32.const 1
                i32.shr_u
                local.tee 8
                local.get 7
                i32.add
                local.tee 17
                local.get 15
                local.get 17
                i32.const 3
                i32.shl
                i32.add
                i64.load
                local.get 10
                i64.gt_u
                select
                local.set 7
                local.get 5
                local.get 8
                i32.sub
                local.tee 5
                i32.const 1
                i32.gt_u
                br_if 0 (;@6;)
              end
              block ;; label = @6
                local.get 15
                local.get 7
                i32.const 3
                i32.shl
                i32.add
                i64.load
                local.tee 13
                local.get 10
                i64.eq
                local.tee 5
                br_if 0 (;@6;)
                local.get 7
                local.get 13
                local.get 10
                i64.lt_u
                i32.add
                local.set 7
              end
              local.get 1
              local.get 7
              i64.extend_i32_u
              i64.const 0
              local.get 5
              select
              i64.store offset=136
              block ;; label = @6
                i32.const 16384
                local.get 20
                i32.sub
                local.tee 7
                i32.const 8
                local.get 7
                i32.const 8
                i32.lt_u
                select
                local.tee 5
                i32.eqz
                br_if 0 (;@6;)
                local.get 20
                i32.const 1051480
                i32.add
                local.get 1
                i32.const 136
                i32.add
                local.get 5
                memory.copy
              end
              i32.const 0
              local.set 7
              i32.const 0
              local.get 5
              local.get 20
              i32.add
              local.tee 20
              i32.store offset=1067864
              local.get 16
              local.set 5
              loop ;; label = @6
                local.get 7
                local.get 5
                i32.const 1
                i32.shr_u
                local.tee 8
                local.get 7
                i32.add
                local.tee 17
                local.get 15
                local.get 17
                i32.const 3
                i32.shl
                i32.add
                i64.load
                local.get 10
                i64.gt_u
                select
                local.set 7
                local.get 5
                local.get 8
                i32.sub
                local.tee 5
                i32.const 1
                i32.gt_u
                br_if 0 (;@6;)
              end
              local.get 1
              local.get 15
              local.get 7
              i32.const 3
              i32.shl
              i32.add
              i64.load
              local.get 10
              i64.eq
              i32.store8 offset=136
              local.get 20
              local.get 20
              i32.const 16384
              i32.ne
              local.tee 7
              i32.add
              local.tee 5
              i32.const 16385
              i32.ge_u
              br_if 1 (;@4;)
              block ;; label = @6
                local.get 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 20
                i32.const 1051480
                i32.add
                local.get 1
                i32.const 136
                i32.add
                local.get 7
                memory.copy
              end
              i32.const 0
              local.set 7
              i32.const 0
              local.get 5
              i32.store offset=1067864
              i64.const 0
              local.set 27
              local.get 1
              i32.load offset=28
              local.set 20
              i64.const 0
              local.set 28
              block ;; label = @6
                local.get 9
                i32.eqz
                br_if 0 (;@6;)
                i64.const 0
                local.set 10
                i64.const 0
                local.set 13
                block ;; label = @7
                  block ;; label = @8
                    local.get 2
                    i32.wrap_i64
                    i32.const -65
                    i32.mul
                    i32.const -95
                    local.get 0
                    i32.wrap_i64
                    i32.sub
                    i32.eq
                    br_if 0 (;@8;)
                    i32.const 1
                    local.set 8
                    local.get 3
                    i32.const 1
                    i32.and
                    local.set 3
                    local.get 4
                    i32.const 254
                    i32.and
                    local.set 17
                    i32.const 0
                    local.set 7
                    local.get 20
                    local.set 5
                    i64.const 1
                    local.set 10
                    i64.const 0
                    local.set 13
                    loop ;; label = @9
                      i64.const 0
                      local.get 5
                      i32.const 8
                      i32.add
                      i64.load
                      local.get 10
                      i64.const 1
                      i64.add
                      i64.mul
                      local.get 8
                      i32.const 3
                      i32.div_u
                      i32.const 3
                      i32.mul
                      i32.const -1
                      i32.add
                      local.get 7
                      i32.eq
                      select
                      i64.const 0
                      local.get 5
                      i64.load
                      local.get 10
                      i64.mul
                      local.get 7
                      i32.const 3
                      i32.div_u
                      i32.const 3
                      i32.mul
                      local.get 7
                      i32.eq
                      select
                      local.get 13
                      i64.add
                      i64.add
                      local.set 13
                      local.get 5
                      i32.const 16
                      i32.add
                      local.set 5
                      local.get 8
                      i32.const 2
                      i32.add
                      local.set 8
                      local.get 10
                      i64.const 2
                      i64.add
                      local.set 10
                      local.get 17
                      local.get 7
                      i32.const 2
                      i32.add
                      local.tee 7
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 3
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 10
                    i64.const -1
                    i64.add
                    local.set 10
                  end
                  local.get 20
                  local.get 7
                  i32.const 3
                  i32.shl
                  i32.add
                  i64.load
                  local.get 10
                  i64.const 1
                  i64.add
                  i64.mul
                  i64.const 0
                  local.get 10
                  i32.wrap_i64
                  i32.const 3
                  i32.rem_u
                  select
                  local.get 13
                  i64.add
                  local.set 13
                end
                local.get 13
                i64.const 1099511628211
                i64.mul
                i64.const 9
                i64.rotl
                local.set 28
              end
              local.get 19
              i32.const -8
              i32.add
              local.set 17
              i32.const 0
              local.set 7
              i32.const 0
              local.get 16
              i32.const 3
              i32.shl
              i32.sub
              local.set 8
              local.get 15
              local.set 5
              block ;; label = @6
                loop ;; label = @7
                  local.get 8
                  local.get 7
                  i32.eq
                  br_if 1 (;@6;)
                  local.get 5
                  i64.load
                  local.get 17
                  local.get 7
                  i32.add
                  i64.load
                  i64.const 17
                  i64.rotl
                  i64.xor
                  local.get 27
                  i64.xor
                  i64.const 1099511628211
                  i64.mul
                  i64.const 9
                  i64.rotl
                  local.set 27
                  local.get 5
                  i32.const 8
                  i32.add
                  local.set 5
                  local.get 7
                  i32.const -8
                  i32.add
                  local.tee 7
                  i32.const -128
                  i32.ne
                  br_if 0 (;@7;)
                end
              end
              local.get 20
              local.get 6
              i32.add
              local.set 19
              local.get 1
              i32.load offset=64
              local.tee 7
              local.get 18
              i32.const 3
              i32.shl
              i32.add
              local.set 18
              i64.const 0
              local.set 14
              i32.const 8
              local.set 5
              i32.const 8
              local.set 17
              local.get 20
              local.set 6
              block ;; label = @6
                loop ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 6
                      i32.eqz
                      br_if 0 (;@9;)
                      block ;; label = @10
                        local.get 17
                        br_if 0 (;@10;)
                        i32.const 0
                        local.set 17
                        br 1 (;@9;)
                      end
                      local.get 17
                      i32.const -1
                      i32.add
                      local.set 17
                      local.get 19
                      local.get 6
                      i32.eq
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const 8
                      i32.add
                      local.set 8
                      br 1 (;@8;)
                    end
                    local.get 5
                    i32.eqz
                    br_if 2 (;@6;)
                    local.get 7
                    local.get 18
                    i32.eq
                    br_if 2 (;@6;)
                    local.get 5
                    i32.const -1
                    i32.add
                    local.set 5
                    i32.const 0
                    local.set 8
                    local.get 7
                    local.set 6
                    local.get 7
                    i32.const 8
                    i32.add
                    local.set 7
                  end
                  local.get 6
                  i64.load
                  local.get 14
                  i64.xor
                  i64.const 1099511628211
                  i64.mul
                  i64.const 9
                  i64.rotl
                  local.set 14
                  local.get 8
                  local.set 6
                  br 0 (;@7;)
                end
              end
              local.get 1
              i32.const 0
              i32.store offset=92
              local.get 1
              i64.const 34359738368
              i64.store offset=84 align=4
              i32.const 8
              local.set 7
              i32.const 0
              local.set 4
              block ;; label = @6
                local.get 9
                i32.eqz
                br_if 0 (;@6;)
                i32.const 0
                local.set 18
                i32.const 8
                local.set 5
                i32.const 0
                local.set 4
                local.get 20
                local.set 8
                loop ;; label = @7
                  local.get 8
                  i64.load
                  local.set 13
                  block ;; label = @8
                    local.get 18
                    local.tee 6
                    local.get 4
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 84
                    i32.add
                    call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecyE8grow_oneCsbgrmSDC215G_11collections
                    local.get 1
                    i32.load offset=88
                    local.set 5
                    local.get 1
                    i32.load offset=84
                    local.set 4
                  end
                  local.get 5
                  local.get 6
                  i32.const 3
                  i32.shl
                  i32.add
                  local.get 13
                  i64.store
                  local.get 1
                  local.get 6
                  i32.const 1
                  i32.add
                  local.tee 18
                  i32.store offset=92
                  i32.const 0
                  local.set 17
                  block ;; label = @8
                    local.get 6
                    i32.eqz
                    br_if 0 (;@8;)
                    loop ;; label = @9
                      block ;; label = @10
                        local.get 13
                        local.get 5
                        local.get 6
                        i32.const -1
                        i32.add
                        i32.const 1
                        i32.shr_u
                        local.tee 7
                        i32.const 3
                        i32.shl
                        i32.add
                        i64.load
                        local.tee 10
                        i64.gt_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.set 17
                        br 2 (;@8;)
                      end
                      local.get 5
                      local.get 6
                      i32.const 3
                      i32.shl
                      i32.add
                      local.get 10
                      i64.store
                      local.get 7
                      local.set 6
                      local.get 7
                      br_if 0 (;@9;)
                    end
                  end
                  local.get 5
                  local.get 17
                  i32.const 3
                  i32.shl
                  i32.add
                  local.get 13
                  i64.store
                  local.get 19
                  local.get 8
                  i32.const 8
                  i32.add
                  local.tee 8
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 1
                i32.load offset=88
                local.set 7
                local.get 1
                i32.load offset=92
                local.set 4
              end
              local.get 1
              i32.const 0
              i32.store offset=144
              local.get 1
              i64.const 34359738368
              i64.store offset=136 align=4
              i32.const 8
              local.set 21
              i32.const 0
              local.set 18
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      loop ;; label = @10
                        local.get 4
                        i32.eqz
                        br_if 1 (;@9;)
                        local.get 7
                        local.get 4
                        i32.const -1
                        i32.add
                        local.tee 3
                        i32.const 3
                        i32.shl
                        i32.add
                        i64.load
                        local.set 13
                        block ;; label = @11
                          block ;; label = @12
                            local.get 3
                            br_if 0 (;@12;)
                            local.get 13
                            local.set 26
                            br 1 (;@11;)
                          end
                          local.get 7
                          i64.load
                          local.set 26
                          local.get 7
                          local.get 13
                          i64.store
                          block ;; label = @12
                            block ;; label = @13
                              block ;; label = @14
                                block ;; label = @15
                                  local.get 4
                                  i32.const 4
                                  i32.lt_u
                                  br_if 0 (;@15;)
                                  i32.const 0
                                  local.get 3
                                  i32.const -2
                                  i32.add
                                  local.tee 6
                                  local.get 6
                                  local.get 3
                                  i32.gt_u
                                  select
                                  local.set 17
                                  i32.const 0
                                  local.set 5
                                  i32.const 1
                                  local.set 6
                                  loop ;; label = @16
                                    local.get 7
                                    local.get 5
                                    i32.const 3
                                    i32.shl
                                    i32.add
                                    local.get 7
                                    local.get 6
                                    local.get 7
                                    local.get 6
                                    i32.const 3
                                    i32.shl
                                    i32.add
                                    local.tee 5
                                    i64.load
                                    local.get 5
                                    i32.const 8
                                    i32.add
                                    i64.load
                                    i64.le_u
                                    i32.add
                                    local.tee 5
                                    i32.const 3
                                    i32.shl
                                    i32.add
                                    local.tee 19
                                    i64.load
                                    i64.store
                                    local.get 5
                                    i32.const 1
                                    i32.shl
                                    local.tee 8
                                    i32.const 1
                                    i32.or
                                    local.set 6
                                    local.get 8
                                    local.get 17
                                    i32.lt_u
                                    br_if 0 (;@16;)
                                  end
                                  local.get 8
                                  local.get 4
                                  i32.const -3
                                  i32.add
                                  i32.eq
                                  br_if 1 (;@14;)
                                  local.get 19
                                  local.get 13
                                  i64.store
                                  local.get 5
                                  local.set 6
                                  br 2 (;@13;)
                                end
                                i32.const 0
                                local.set 5
                                i32.const 1
                                local.set 6
                                local.get 3
                                i32.const 2
                                i32.ne
                                br_if 2 (;@12;)
                              end
                              local.get 7
                              local.get 5
                              i32.const 3
                              i32.shl
                              i32.add
                              local.get 7
                              local.get 6
                              i32.const 3
                              i32.shl
                              i32.add
                              local.tee 5
                              i64.load
                              i64.store
                              local.get 5
                              local.get 13
                              i64.store
                            end
                            loop ;; label = @13
                              block ;; label = @14
                                local.get 13
                                local.get 7
                                local.get 6
                                i32.const -1
                                i32.add
                                i32.const 1
                                i32.shr_u
                                local.tee 5
                                i32.const 3
                                i32.shl
                                i32.add
                                i64.load
                                local.tee 10
                                i64.gt_u
                                br_if 0 (;@14;)
                                local.get 6
                                local.set 5
                                br 2 (;@12;)
                              end
                              local.get 7
                              local.get 6
                              i32.const 3
                              i32.shl
                              i32.add
                              local.get 10
                              i64.store
                              local.get 5
                              local.set 6
                              local.get 5
                              br_if 0 (;@13;)
                            end
                            i32.const 0
                            local.set 5
                          end
                          local.get 7
                          local.get 5
                          i32.const 3
                          i32.shl
                          i32.add
                          local.get 13
                          i64.store
                        end
                        block ;; label = @11
                          local.get 18
                          local.get 1
                          i32.load offset=136
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 1
                          i32.const 136
                          i32.add
                          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecyE8grow_oneCsbgrmSDC215G_11collections
                          local.get 1
                          i32.load offset=140
                          local.set 21
                        end
                        local.get 21
                        local.get 18
                        i32.const 3
                        i32.shl
                        i32.add
                        local.get 26
                        i64.store
                        local.get 1
                        local.get 18
                        i32.const 1
                        i32.add
                        local.tee 6
                        i32.store offset=144
                        local.get 18
                        i32.const 7
                        i32.lt_u
                        local.set 5
                        local.get 6
                        local.set 18
                        local.get 3
                        local.set 4
                        local.get 5
                        br_if 0 (;@10;)
                      end
                      local.get 1
                      i32.load offset=140
                      local.tee 5
                      local.get 6
                      i32.const 3
                      i32.shl
                      i32.add
                      local.set 17
                      br 1 (;@8;)
                    end
                    local.get 18
                    i32.eqz
                    br_if 1 (;@7;)
                    local.get 1
                    i32.load offset=140
                    local.tee 5
                    local.get 18
                    i32.const 3
                    i32.shl
                    i32.add
                    local.set 17
                  end
                  i32.const 0
                  i32.load offset=1067864
                  local.set 6
                  loop ;; label = @8
                    local.get 1
                    local.get 5
                    i64.load
                    i64.store offset=136
                    i32.const 0
                    i32.const 16384
                    local.get 6
                    i32.sub
                    local.tee 7
                    local.get 7
                    i32.const 16384
                    i32.gt_u
                    select
                    local.tee 7
                    i32.const 8
                    local.get 7
                    i32.const 8
                    i32.lt_u
                    select
                    local.tee 8
                    local.get 6
                    i32.add
                    local.tee 7
                    local.get 6
                    i32.lt_u
                    br_if 2 (;@6;)
                    local.get 7
                    i32.const 16385
                    i32.ge_u
                    br_if 2 (;@6;)
                    block ;; label = @9
                      local.get 8
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 6
                      i32.const 1051480
                      i32.add
                      local.get 1
                      i32.const 136
                      i32.add
                      local.get 8
                      memory.copy
                    end
                    i32.const 0
                    local.get 7
                    i32.store offset=1067864
                    local.get 7
                    local.set 6
                    local.get 5
                    i32.const 8
                    i32.add
                    local.tee 5
                    local.get 17
                    i32.ne
                    br_if 0 (;@8;)
                  end
                end
                local.get 1
                i32.const 0
                i32.store offset=92
                local.get 1
                i64.const 34359738368
                i64.store offset=84 align=4
                local.get 0
                i64.const 5
                i64.rem_u
                local.set 10
                local.get 9
                i32.eqz
                br_if 4 (;@2;)
                local.get 10
                i32.wrap_i64
                i32.const 4
                i32.add
                local.set 21
                local.get 1
                i64.const 17179869184
                i64.store offset=136 align=4
                local.get 1
                i64.const 0
                i64.store offset=144 align=4
                i32.const 8
                local.set 29
                i32.const 0
                local.set 5
                i32.const 0
                local.set 3
                i32.const 0
                local.set 6
                i32.const 0
                local.set 18
                loop ;; label = @7
                  i32.const 0
                  local.set 4
                  block ;; label = @8
                    local.get 6
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load offset=144
                    local.tee 7
                    i32.const -1
                    i32.add
                    local.set 17
                    local.get 20
                    local.get 18
                    i32.const 3
                    i32.shl
                    i32.add
                    local.set 19
                    local.get 1
                    i32.load offset=140
                    local.get 6
                    local.get 7
                    i32.add
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const -4
                    i32.add
                    local.set 7
                    loop ;; label = @9
                      block ;; label = @10
                        local.get 7
                        i32.const 0
                        local.get 5
                        local.get 17
                        local.get 6
                        i32.add
                        local.get 5
                        i32.lt_u
                        select
                        i32.const 2
                        i32.shl
                        i32.sub
                        i32.load
                        local.tee 8
                        local.get 9
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 8
                        local.get 9
                        i32.const 1050640
                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                        unreachable
                      end
                      block ;; label = @10
                        local.get 20
                        local.get 8
                        i32.const 3
                        i32.shl
                        i32.add
                        i64.load
                        local.get 19
                        i64.load
                        i64.le_u
                        br_if 0 (;@10;)
                        local.get 6
                        local.set 4
                        br 2 (;@8;)
                      end
                      local.get 1
                      local.get 6
                      i32.const -1
                      i32.add
                      local.tee 6
                      i32.store offset=148
                      local.get 7
                      i32.const -4
                      i32.add
                      local.set 7
                      local.get 6
                      br_if 0 (;@9;)
                    end
                  end
                  block ;; label = @8
                    local.get 4
                    local.get 5
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 1
                    i32.const 136
                    i32.add
                    call $_RNvMs4_NtNtCseFzb18zECBI_5alloc11collections9vec_dequeINtB5_8VecDequejE4growCsbgrmSDC215G_11collections.llvm.15053149350743835710
                    local.get 1
                    i32.load offset=136
                    local.set 5
                    local.get 1
                    i32.load offset=148
                    local.set 4
                  end
                  local.get 1
                  local.get 4
                  i32.const 1
                  i32.add
                  i32.store offset=148
                  local.get 1
                  i32.load offset=140
                  local.get 1
                  i32.load offset=144
                  local.get 4
                  i32.add
                  local.tee 6
                  i32.const 0
                  local.get 5
                  local.get 6
                  local.get 5
                  i32.lt_u
                  select
                  i32.sub
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 18
                  i32.store
                  block ;; label = @8
                    block ;; label = @9
                      local.get 1
                      i32.load offset=148
                      local.tee 8
                      br_if 0 (;@9;)
                      local.get 18
                      i32.const 1
                      i32.add
                      local.set 18
                      br 1 (;@8;)
                    end
                    i32.const 0
                    local.set 5
                    block ;; label = @9
                      local.get 1
                      i32.load offset=140
                      local.tee 17
                      local.get 1
                      i32.load offset=144
                      local.tee 6
                      i32.const 0
                      local.get 1
                      i32.load offset=136
                      local.tee 7
                      local.get 6
                      local.get 7
                      i32.lt_u
                      select
                      i32.sub
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load
                      local.get 21
                      i32.add
                      local.get 18
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 1
                      local.get 6
                      i32.const 1
                      i32.add
                      local.tee 6
                      i32.const 0
                      local.get 7
                      local.get 6
                      local.get 7
                      i32.lt_u
                      select
                      i32.sub
                      local.tee 6
                      i32.store offset=144
                      local.get 1
                      local.get 8
                      i32.const -1
                      i32.add
                      local.tee 5
                      i32.store offset=148
                      local.get 5
                      i32.eqz
                      local.set 5
                    end
                    local.get 18
                    i32.const 1
                    i32.add
                    local.tee 18
                    local.get 21
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 5
                    br_if 0 (;@8;)
                    block ;; label = @9
                      local.get 17
                      local.get 6
                      i32.const 0
                      local.get 7
                      local.get 6
                      local.get 7
                      i32.lt_u
                      select
                      i32.sub
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load
                      local.tee 6
                      local.get 9
                      i32.ge_u
                      br_if 0 (;@9;)
                      local.get 20
                      local.get 6
                      i32.const 3
                      i32.shl
                      i32.add
                      i64.load
                      local.set 10
                      block ;; label = @10
                        local.get 3
                        local.get 1
                        i32.load offset=84
                        i32.ne
                        br_if 0 (;@10;)
                        local.get 1
                        i32.const 84
                        i32.add
                        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecyE8grow_oneCsbgrmSDC215G_11collections
                        local.get 1
                        i32.load offset=88
                        local.set 29
                      end
                      local.get 29
                      local.get 3
                      i32.const 3
                      i32.shl
                      i32.add
                      local.get 10
                      i64.store
                      local.get 1
                      local.get 3
                      i32.const 1
                      i32.add
                      local.tee 3
                      i32.store offset=92
                      br 1 (;@8;)
                    end
                    local.get 6
                    local.get 9
                    i32.const 1050656
                    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                    unreachable
                  end
                  block ;; label = @8
                    local.get 9
                    local.get 18
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load offset=136
                    local.set 5
                    local.get 1
                    i32.load offset=148
                    local.set 6
                    br 1 (;@7;)
                  end
                end
                local.get 3
                i32.eqz
                br_if 4 (;@2;)
                local.get 1
                i32.load offset=88
                local.set 9
                local.get 3
                i32.const 3
                i32.and
                local.set 7
                block ;; label = @7
                  block ;; label = @8
                    local.get 3
                    i32.const 4
                    i32.ge_u
                    br_if 0 (;@8;)
                    i64.const 0
                    local.set 30
                    i32.const 0
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 3
                  i32.const -4
                  i32.and
                  local.set 8
                  i64.const 0
                  local.set 30
                  i32.const 0
                  local.set 5
                  local.get 9
                  local.set 6
                  loop ;; label = @8
                    local.get 6
                    i32.const 24
                    i32.add
                    i64.load
                    local.get 6
                    i32.const 16
                    i32.add
                    i64.load
                    local.get 6
                    i32.const 8
                    i32.add
                    i64.load
                    local.get 6
                    i64.load
                    local.get 30
                    i64.add
                    i64.add
                    i64.add
                    i64.add
                    local.set 30
                    local.get 6
                    i32.const 32
                    i32.add
                    local.set 6
                    local.get 8
                    local.get 5
                    i32.const 4
                    i32.add
                    local.tee 5
                    i32.ne
                    br_if 0 (;@8;)
                  end
                  local.get 7
                  i32.eqz
                  br_if 6 (;@1;)
                end
                local.get 9
                local.get 5
                i32.const 3
                i32.shl
                i32.add
                local.set 6
                loop ;; label = @7
                  local.get 6
                  i64.load
                  local.get 30
                  i64.add
                  local.set 30
                  local.get 6
                  i32.const 8
                  i32.add
                  local.set 6
                  local.get 7
                  i32.const -1
                  i32.add
                  local.tee 7
                  br_if 0 (;@7;)
                  br 6 (;@1;)
                end
              end
              local.get 6
              local.get 7
              i32.const 16384
              i32.const 1050608
              call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
              unreachable
            end
            local.get 5
            local.get 16
            i32.const 1050700
            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
            unreachable
          end
          local.get 20
          local.get 5
          i32.const 16384
          i32.const 1050608
          call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
          unreachable
        end
        local.get 7
        local.get 20
        i32.const 16384
        i32.const 1050608
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      i64.const 0
      local.set 30
      i32.const 0
      local.set 3
    end
    local.get 1
    local.get 3
    i64.extend_i32_u
    i64.store offset=136
    block ;; label = @1
      block ;; label = @2
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1067864
        local.tee 7
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
        local.tee 5
        local.get 7
        i32.add
        local.tee 6
        local.get 7
        i32.lt_u
        br_if 0 (;@2;)
        local.get 6
        i32.const 16385
        i32.ge_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 7
          i32.const 1051480
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 5
          memory.copy
        end
        local.get 1
        local.get 30
        i64.store offset=136
        block ;; label = @3
          i32.const 16384
          local.get 6
          i32.sub
          local.tee 7
          i32.const 8
          local.get 7
          i32.const 8
          i32.lt_u
          select
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1051480
          i32.add
          local.get 1
          i32.const 136
          i32.add
          local.get 7
          memory.copy
        end
        i32.const 0
        local.get 7
        local.get 6
        i32.add
        i32.store offset=1067864
        local.get 1
        i64.const 34359738368
        i64.store offset=84 align=4
        local.get 1
        i64.const 0
        i64.store offset=92 align=4
        local.get 15
        local.set 8
        i32.const 0
        local.set 5
        loop ;; label = @3
          local.get 16
          local.get 5
          i32.eq
          br_if 2 (;@1;)
          local.get 8
          i64.load
          local.set 10
          block ;; label = @4
            local.get 1
            i32.load offset=96
            local.tee 6
            local.get 1
            i32.load offset=84
            local.tee 7
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            i32.const 84
            i32.add
            call $_RNvMs4_NtNtCseFzb18zECBI_5alloc11collections9vec_dequeINtB5_8VecDequeyE4growCsbgrmSDC215G_11collections.llvm.15053149350743835710
            local.get 1
            i32.load offset=84
            local.set 7
            local.get 1
            i32.load offset=96
            local.set 6
          end
          local.get 1
          local.get 6
          i32.const 1
          i32.add
          i32.store offset=96
          local.get 1
          i32.load offset=88
          local.get 1
          i32.load offset=92
          local.get 6
          i32.add
          local.tee 6
          i32.const 0
          local.get 7
          local.get 6
          local.get 7
          i32.lt_u
          select
          i32.sub
          i32.const 3
          i32.shl
          i32.add
          local.get 10
          i64.store
          local.get 8
          i32.const 8
          i32.add
          local.set 8
          local.get 5
          i32.const 1
          i32.add
          local.tee 5
          i32.const 24
          i32.ne
          br_if 0 (;@3;)
          br 2 (;@1;)
        end
      end
      local.get 7
      local.get 6
      i32.const 16384
      i32.const 1050608
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    block ;; label = @1
      local.get 0
      i64.const 7
      i64.rem_u
      local.tee 10
      i64.eqz
      br_if 0 (;@1;)
      local.get 10
      i32.wrap_i64
      local.set 5
      loop ;; label = @2
        block ;; label = @3
          local.get 1
          i32.load offset=96
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          local.get 1
          i32.load offset=92
          local.tee 8
          i32.const 1
          i32.add
          local.tee 9
          i32.const 0
          local.get 1
          i32.load offset=84
          local.tee 6
          local.get 9
          local.get 6
          i32.lt_u
          select
          i32.sub
          local.tee 9
          i32.store offset=92
          local.get 1
          i32.load offset=88
          local.tee 17
          local.get 8
          i32.const 3
          i32.shl
          i32.add
          i64.load
          local.set 10
          local.get 1
          local.get 7
          i32.store offset=96
          local.get 17
          local.get 7
          local.get 9
          i32.add
          i32.const -1
          i32.add
          local.tee 7
          i32.const 0
          local.get 6
          local.get 7
          local.get 6
          i32.lt_u
          select
          i32.sub
          i32.const 3
          i32.shl
          i32.add
          local.get 10
          i64.store
        end
        local.get 5
        i32.const -1
        i32.add
        local.tee 5
        br_if 0 (;@2;)
      end
    end
    i32.const 0
    local.set 7
    i32.const 0
    local.set 6
    i32.const 0
    local.set 5
    block ;; label = @1
      local.get 1
      i32.load offset=96
      local.tee 8
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      local.set 5
      block ;; label = @2
        local.get 8
        local.get 1
        i32.load offset=84
        local.tee 6
        local.get 1
        i32.load offset=92
        local.tee 7
        i32.const 0
        local.get 6
        local.get 7
        local.get 6
        i32.lt_u
        select
        i32.sub
        local.tee 7
        i32.sub
        local.tee 9
        i32.le_u
        br_if 0 (;@2;)
        local.get 8
        local.get 9
        i32.sub
        local.set 5
        br 1 (;@1;)
      end
      local.get 7
      local.get 8
      i32.add
      local.set 6
    end
    local.get 1
    i32.load offset=88
    local.tee 9
    local.get 5
    i32.const 3
    i32.shl
    i32.add
    local.set 8
    local.get 9
    local.get 6
    i32.const 3
    i32.shl
    i32.add
    local.set 6
    local.get 9
    local.get 7
    i32.const 3
    i32.shl
    i32.add
    local.set 5
    i64.const 0
    local.set 13
    block ;; label = @1
      loop ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 6
            local.get 5
            i32.eq
            br_if 0 (;@4;)
            local.get 5
            local.set 7
            local.get 6
            local.set 17
            local.get 8
            local.set 6
            br 1 (;@3;)
          end
          local.get 9
          i32.eqz
          br_if 2 (;@1;)
          local.get 9
          local.get 8
          i32.eq
          local.set 5
          local.get 9
          local.set 7
          local.get 8
          local.set 17
          local.get 6
          local.set 9
          local.get 5
          br_if 2 (;@1;)
        end
        local.get 7
        i32.const 8
        i32.add
        local.set 5
        local.get 7
        i64.load
        local.get 13
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 9
        i64.rotl
        local.set 13
        local.get 6
        local.set 8
        local.get 17
        local.set 6
        br 0 (;@2;)
      end
    end
    i32.const 0
    local.set 6
    local.get 1
    i32.const 0
    i32.store offset=108
    local.get 1
    i32.const 0
    i32.store offset=100
    local.get 22
    i32.const -8
    i32.add
    local.set 5
    block ;; label = @1
      loop ;; label = @2
        local.get 1
        i32.const 100
        i32.add
        local.get 15
        local.get 6
        i32.add
        i64.load
        i64.const 101
        i64.rem_u
        call $_RNvMsi_NtNtNtCseFzb18zECBI_5alloc11collections5btree3mapINtB5_8BTreeMapyNtNtB7_7set_val9SetValZSTE6insertCsbgrmSDC215G_11collections
        drop
        local.get 6
        i32.const 376
        i32.eq
        br_if 1 (;@1;)
        local.get 5
        local.get 6
        i32.eq
        local.set 7
        local.get 6
        i32.const 8
        i32.add
        local.set 6
        local.get 7
        i32.eqz
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.const 0
    i32.store offset=120
    local.get 1
    i32.const 0
    i32.store offset=112
    local.get 1
    i32.const 112
    i32.add
    local.get 15
    i64.load offset=128
    i64.const 67
    i64.rem_u
    call $_RNvMsi_NtNtNtCseFzb18zECBI_5alloc11collections5btree3mapINtB5_8BTreeMapyNtNtB7_7set_val9SetValZSTE6insertCsbgrmSDC215G_11collections
    drop
    local.get 16
    i32.const 3
    i32.shl
    local.set 7
    i32.const 136
    local.set 6
    block ;; label = @1
      loop ;; label = @2
        local.get 7
        local.get 6
        i32.eq
        br_if 1 (;@1;)
        local.get 1
        i32.const 112
        i32.add
        local.get 15
        local.get 6
        i32.add
        i64.load
        i64.const 67
        i64.rem_u
        call $_RNvMsi_NtNtNtCseFzb18zECBI_5alloc11collections5btree3mapINtB5_8BTreeMapyNtNtB7_7set_val9SetValZSTE6insertCsbgrmSDC215G_11collections
        drop
        local.get 6
        i32.const 8
        i32.add
        local.tee 6
        i32.const 512
        i32.ne
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.const 136
    i32.add
    local.get 1
    i32.const 100
    i32.add
    local.get 1
    i32.const 112
    i32.add
    call $_RNvMsc_NtNtNtCseFzb18zECBI_5alloc11collections5btree3setINtB5_8BTreeSetyE12intersectionCsbgrmSDC215G_11collections
    local.get 1
    i32.const 124
    i32.add
    local.get 1
    i32.const 136
    i32.add
    call $_RNvXNtNtCseFzb18zECBI_5alloc3vec21spec_from_iter_nestedINtB4_3VecyEINtB2_18SpecFromIterNestedyINtNtNtNtCs8euEcNjxCEf_4core4iter8adapters6copied6CopiedINtNtNtNtB6_11collections5btree3set12IntersectionyEEE9from_iterCsbgrmSDC215G_11collections
    local.get 1
    i32.const 2
    i32.store offset=208
    local.get 1
    local.get 1
    i32.load offset=120
    i32.const 0
    local.get 1
    i32.load offset=112
    local.tee 6
    select
    i32.store offset=204
    local.get 1
    local.get 1
    i32.load offset=116
    local.tee 7
    i32.store offset=200
    local.get 1
    local.get 6
    i32.store offset=196
    local.get 1
    i32.const 0
    i32.store offset=192
    local.get 1
    local.get 6
    i32.const 0
    i32.ne
    local.tee 5
    i32.store offset=188
    local.get 1
    local.get 7
    i32.store offset=184
    local.get 1
    local.get 6
    i32.store offset=180
    local.get 1
    i32.const 0
    i32.store offset=176
    local.get 1
    local.get 5
    i32.store offset=172
    local.get 1
    local.get 1
    i32.load offset=108
    i32.const 0
    local.get 1
    i32.load offset=100
    local.tee 6
    select
    i32.store offset=168
    local.get 1
    local.get 1
    i32.load offset=104
    local.tee 7
    i32.store offset=164
    local.get 1
    local.get 6
    i32.store offset=160
    local.get 1
    i32.const 0
    i32.store offset=156
    local.get 1
    local.get 6
    i32.const 0
    i32.ne
    local.tee 5
    i32.store offset=152
    local.get 1
    local.get 7
    i32.store offset=148
    local.get 1
    local.get 6
    i32.store offset=144
    local.get 1
    i32.const 0
    i32.store offset=140
    local.get 1
    local.get 5
    i32.store offset=136
    i32.const -1
    local.set 15
    loop ;; label = @1
      local.get 15
      i32.const 1
      i32.add
      local.set 15
      local.get 1
      i32.const 16
      i32.add
      local.get 1
      i32.const 136
      i32.add
      call $_RINvMs0_NtNtNtCseFzb18zECBI_5alloc11collections5btree10merge_iterINtB6_14MergeIterInnerINtNtB8_3set4IteryEE5nextsNvYRyNtNtCs8euEcNjxCEf_4core3cmp3Ord3cmpECsbgrmSDC215G_11collections
      local.get 1
      i32.load offset=16
      local.get 1
      i32.load offset=20
      i32.or
      br_if 0 (;@1;)
    end
    i64.const 0
    local.set 2
    block ;; label = @1
      local.get 1
      i32.load offset=132
      local.tee 7
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=128
      local.set 6
      local.get 7
      i32.const -1
      i32.add
      i32.const 536870911
      i32.and
      local.tee 5
      i32.const 1
      i32.add
      local.tee 8
      i32.const 3
      i32.and
      local.set 7
      i64.const 0
      local.set 2
      block ;; label = @2
        local.get 5
        i32.const 3
        i32.lt_u
        br_if 0 (;@2;)
        local.get 8
        i32.const 1073741820
        i32.and
        local.set 5
        i64.const 0
        local.set 2
        loop ;; label = @3
          local.get 6
          i32.const 24
          i32.add
          i64.load
          local.get 6
          i32.const 16
          i32.add
          i64.load
          local.get 6
          i32.const 8
          i32.add
          i64.load
          local.get 6
          i64.load
          local.get 2
          i64.xor
          i64.const 1099511628211
          i64.mul
          i64.const 9
          i64.rotl
          i64.xor
          i64.const 1099511628211
          i64.mul
          i64.const 9
          i64.rotl
          i64.xor
          i64.const 1099511628211
          i64.mul
          i64.const 9
          i64.rotl
          i64.xor
          i64.const 1099511628211
          i64.mul
          i64.const 9
          i64.rotl
          local.set 2
          local.get 6
          i32.const 32
          i32.add
          local.set 6
          local.get 5
          i32.const -4
          i32.add
          local.tee 5
          br_if 0 (;@3;)
        end
        local.get 7
        i32.eqz
        br_if 1 (;@1;)
      end
      loop ;; label = @2
        local.get 6
        i64.load
        local.get 2
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 9
        i64.rotl
        local.set 2
        local.get 6
        i32.const 8
        i32.add
        local.set 6
        local.get 7
        i32.const -1
        i32.add
        local.tee 7
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.const 0
    i32.store offset=228
    local.get 1
    i64.const 4294967296
    i64.store offset=220 align=4
    local.get 1
    i32.const 220
    i32.add
    i32.const 0
    local.get 11
    i64.const 7046029254386353131
    i64.add
    local.tee 10
    i64.const 30
    i64.shr_u
    local.get 10
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 26
    i64.const 27
    i64.shr_u
    local.get 26
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 26
    i64.const 31
    i64.shr_u
    local.get 26
    i64.xor
    i32.wrap_i64
    i32.const 7
    i32.and
    i32.const 3
    i32.shl
    local.tee 8
    i32.load offset=1050760
    local.tee 6
    i32.const 1
    i32.const 1
    call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
    local.get 1
    i32.load offset=228
    local.set 5
    local.get 1
    i32.load offset=224
    local.set 7
    block ;; label = @1
      local.get 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 7
      local.get 5
      i32.add
      local.get 8
      i32.load offset=1050756
      local.get 6
      memory.copy
    end
    local.get 1
    local.get 5
    local.get 6
    i32.add
    local.tee 6
    i32.store offset=228
    local.get 0
    i64.const 17
    i64.rem_u
    i32.wrap_i64
    local.set 5
    block ;; label = @1
      local.get 1
      i32.load offset=220
      local.get 6
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 220
      i32.add
      local.get 6
      i32.const 1
      i32.const 1
      i32.const 1
      call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
      local.get 1
      i32.load offset=224
      local.set 7
    end
    local.get 7
    local.get 6
    i32.add
    i32.const 32
    i32.store8
    local.get 1
    local.get 6
    i32.const 1
    i32.add
    i32.store offset=228
    local.get 1
    i32.const 220
    i32.add
    local.get 11
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
    i64.const 1000
    i64.rem_u
    call $_RNvCsbgrmSDC215G_11collections8push_u64
    local.get 5
    i32.const 23
    i32.add
    local.set 9
    i32.const -1
    local.set 5
    i32.const 1
    local.set 8
    loop ;; label = @1
      local.get 8
      i32.const 6
      i32.div_u
      local.set 7
      block ;; label = @2
        local.get 1
        i32.load offset=220
        local.tee 17
        local.get 1
        i32.load offset=228
        local.tee 6
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 220
        i32.add
        local.get 6
        i32.const 1
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=220
        local.set 17
      end
      local.get 7
      i32.const 6
      i32.mul
      local.set 19
      local.get 1
      i32.load offset=224
      local.tee 20
      local.get 6
      i32.add
      i32.const 32
      i32.store8
      local.get 1
      local.get 6
      i32.const 1
      i32.add
      local.tee 6
      i32.store offset=228
      local.get 10
      i64.const 4354685564936845354
      i64.add
      local.tee 11
      i64.const 30
      i64.shr_u
      local.get 11
      i64.xor
      i64.const -4658895280553007687
      i64.mul
      local.tee 26
      i64.const 27
      i64.shr_u
      local.get 26
      i64.xor
      i64.const -7723592293110705685
      i64.mul
      local.tee 26
      i64.const 31
      i64.shr_u
      local.get 26
      i64.xor
      i32.wrap_i64
      i32.const 7
      i32.and
      i32.const 3
      i32.shl
      local.tee 7
      i32.load offset=1050756
      local.set 18
      block ;; label = @2
        local.get 7
        i32.load offset=1050760
        local.tee 7
        local.get 17
        local.get 6
        i32.sub
        i32.le_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 220
        i32.add
        local.get 6
        local.get 7
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=224
        local.set 20
        local.get 1
        i32.load offset=228
        local.set 6
      end
      local.get 5
      local.get 19
      i32.add
      local.set 17
      block ;; label = @2
        local.get 7
        i32.eqz
        br_if 0 (;@2;)
        local.get 20
        local.get 6
        i32.add
        local.get 18
        local.get 7
        memory.copy
      end
      local.get 1
      local.get 6
      local.get 7
      i32.add
      local.tee 6
      i32.store offset=228
      block ;; label = @2
        block ;; label = @3
          local.get 17
          i32.eqz
          br_if 0 (;@3;)
          local.get 10
          i64.const -7046029254386353131
          i64.add
          local.set 10
          br 1 (;@2;)
        end
        block ;; label = @3
          local.get 1
          i32.load offset=220
          local.get 6
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 220
          i32.add
          local.get 6
          i32.const 1
          i32.const 1
          i32.const 1
          call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
          local.get 1
          i32.load offset=224
          local.set 20
        end
        local.get 20
        local.get 6
        i32.add
        i32.const 32
        i32.store8
        local.get 1
        local.get 6
        i32.const 1
        i32.add
        i32.store offset=228
        local.get 1
        i32.const 220
        i32.add
        local.get 10
        i64.const -2691343689449507777
        i64.add
        local.tee 10
        i64.const 30
        i64.shr_u
        local.get 10
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
        i64.const 1000
        i64.rem_u
        call $_RNvCsbgrmSDC215G_11collections8push_u64
        local.get 11
        local.set 10
      end
      local.get 5
      i32.const -1
      i32.add
      local.set 5
      local.get 8
      i32.const 1
      i32.add
      local.set 8
      local.get 9
      i32.const -1
      i32.add
      local.tee 9
      br_if 0 (;@1;)
    end
    local.get 1
    i32.load offset=228
    local.set 16
    local.get 1
    i32.load offset=224
    local.set 29
    i32.const 0
    local.set 18
    local.get 1
    i32.const 0
    i32.store offset=240
    local.get 1
    i32.const 0
    i32.store offset=232
    local.get 1
    i32.const 0
    i32.store offset=252
    local.get 1
    i64.const 4294967296
    i64.store offset=244 align=4
    block ;; label = @1
      local.get 16
      i32.eqz
      br_if 0 (;@1;)
      local.get 29
      local.get 16
      i32.add
      local.set 21
      i32.const 0
      local.set 7
      i32.const 0
      local.set 5
      local.get 29
      local.set 18
      loop ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 18
                i32.load8_u
                local.tee 6
                i32.const 123
                i32.lt_u
                local.get 6
                i32.const 91
                i32.lt_u
                local.get 6
                i32.const -48
                i32.add
                i32.const 255
                i32.and
                i32.const 10
                i32.lt_u
                local.get 6
                i32.const 64
                i32.gt_u
                select
                local.get 6
                i32.const 96
                i32.gt_u
                select
                br_if 0 (;@6;)
                local.get 5
                br_if 1 (;@5;)
                i32.const 0
                local.set 5
                br 3 (;@3;)
              end
              block ;; label = @6
                i32.const 1
                i32.const 2
                local.get 6
                i32.extend8_s
                i32.const -1
                i32.gt_s
                local.tee 9
                select
                local.tee 8
                local.get 1
                i32.load offset=244
                local.get 7
                i32.sub
                i32.le_u
                br_if 0 (;@6;)
                local.get 1
                i32.const 244
                i32.add
                local.get 7
                local.get 8
                i32.const 1
                i32.const 1
                call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
              end
              local.get 1
              i32.load offset=248
              local.get 7
              i32.add
              local.set 5
              block ;; label = @6
                local.get 9
                br_if 0 (;@6;)
                local.get 5
                local.get 6
                i32.const 191
                i32.and
                i32.store8 offset=1
                local.get 6
                i32.const 6
                i32.shr_u
                i32.const -64
                i32.or
                local.set 6
              end
              local.get 5
              local.get 6
              i32.store8
              local.get 8
              local.get 7
              i32.add
              local.set 7
              br 1 (;@4;)
            end
            local.get 1
            i32.const 256
            i32.add
            local.get 1
            i32.const 244
            i32.add
            call $_RNvXs4_NtCseFzb18zECBI_5alloc6stringNtB5_6StringNtNtCs8euEcNjxCEf_4core5clone5Clone5clone
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.load offset=232
                    local.tee 17
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 1
                    i32.load offset=264
                    local.set 6
                    local.get 1
                    i32.load offset=260
                    local.set 19
                    local.get 1
                    i32.load offset=236
                    local.set 4
                    loop ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 17
                          i32.load16_u offset=226
                          local.tee 3
                          br_if 0 (;@11;)
                          local.get 3
                          local.set 8
                          br 1 (;@10;)
                        end
                        local.get 3
                        i32.const 12
                        i32.mul
                        local.set 20
                        i32.const 0
                        local.set 8
                        i32.const 0
                        local.set 5
                        block ;; label = @11
                          loop ;; label = @12
                            local.get 19
                            local.get 17
                            local.get 5
                            i32.add
                            local.tee 7
                            i32.const 96
                            i32.add
                            i32.load
                            local.get 6
                            local.get 7
                            i32.const 100
                            i32.add
                            i32.load
                            local.tee 7
                            local.get 6
                            local.get 7
                            i32.lt_u
                            select
                            call $memcmp
                            local.tee 9
                            local.get 6
                            local.get 7
                            i32.sub
                            local.get 9
                            select
                            local.tee 7
                            i32.const 0
                            i32.gt_s
                            local.get 7
                            i32.const 0
                            i32.lt_s
                            i32.sub
                            i32.const 255
                            i32.and
                            local.tee 7
                            i32.const 1
                            i32.ne
                            br_if 1 (;@11;)
                            local.get 8
                            i32.const 1
                            i32.add
                            local.set 8
                            local.get 20
                            local.get 5
                            i32.const 12
                            i32.add
                            local.tee 5
                            i32.ne
                            br_if 0 (;@12;)
                          end
                          local.get 3
                          local.set 8
                          br 1 (;@10;)
                        end
                        local.get 7
                        i32.eqz
                        br_if 4 (;@6;)
                      end
                      local.get 4
                      i32.eqz
                      br_if 2 (;@7;)
                      local.get 4
                      i32.const -1
                      i32.add
                      local.set 4
                      local.get 17
                      local.get 8
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load offset=232
                      local.set 17
                      br 0 (;@9;)
                    end
                  end
                  local.get 1
                  local.get 1
                  i32.load offset=264
                  i32.store offset=144
                  local.get 1
                  local.get 1
                  i64.load offset=256 align=4
                  i64.store offset=136
                  local.get 1
                  i32.const 0
                  i32.store offset=152
                  local.get 1
                  local.get 1
                  i32.const 232
                  i32.add
                  i32.store offset=148
                  br 2 (;@5;)
                end
                local.get 1
                local.get 8
                i32.store offset=160
                local.get 1
                i32.const 0
                i32.store offset=156
                local.get 1
                local.get 17
                i32.store offset=152
                local.get 1
                local.get 1
                i64.load offset=256 align=4
                i64.store offset=136
                local.get 1
                local.get 1
                i32.load offset=264
                i32.store offset=144
                local.get 1
                local.get 1
                i32.const 232
                i32.add
                i32.store offset=148
                br 1 (;@5;)
              end
              local.get 1
              local.get 8
              i32.store offset=148
              local.get 1
              local.get 4
              i32.store offset=144
              local.get 1
              local.get 17
              i32.store offset=140
              local.get 1
              i32.const -1
              i32.store offset=136
              local.get 1
              local.get 1
              i32.const 232
              i32.add
              i32.store offset=152
            end
            local.get 1
            i32.const 136
            i32.add
            i64.const 0
            call $_RNvMs2_NtNtNtNtCseFzb18zECBI_5alloc11collections5btree3map5entryINtB5_5EntryNtNtBd_6string6StringyE9or_insertCsbgrmSDC215G_11collections
            local.tee 6
            local.get 6
            i64.load
            i64.const 1
            i64.add
            i64.store
            i32.const 0
            local.set 7
          end
          local.get 1
          local.get 7
          i32.store offset=252
          local.get 7
          local.set 5
        end
        local.get 18
        i32.const 1
        i32.add
        local.tee 18
        local.get 21
        i32.ne
        br_if 0 (;@2;)
      end
      block ;; label = @2
        local.get 7
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=248
        local.set 20
        local.get 1
        i32.load offset=244
        local.set 3
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 1
                i32.load offset=232
                local.tee 17
                i32.eqz
                br_if 0 (;@6;)
                local.get 1
                i32.load offset=236
                local.set 18
                loop ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 17
                      i32.load16_u offset=226
                      local.tee 4
                      br_if 0 (;@9;)
                      local.get 4
                      local.set 8
                      br 1 (;@8;)
                    end
                    local.get 4
                    i32.const 12
                    i32.mul
                    local.set 19
                    i32.const 0
                    local.set 8
                    i32.const 0
                    local.set 5
                    block ;; label = @9
                      loop ;; label = @10
                        local.get 20
                        local.get 17
                        local.get 5
                        i32.add
                        local.tee 6
                        i32.const 96
                        i32.add
                        i32.load
                        local.get 7
                        local.get 6
                        i32.const 100
                        i32.add
                        i32.load
                        local.tee 6
                        local.get 7
                        local.get 6
                        i32.lt_u
                        select
                        call $memcmp
                        local.tee 9
                        local.get 7
                        local.get 6
                        i32.sub
                        local.get 9
                        select
                        local.tee 6
                        i32.const 0
                        i32.gt_s
                        local.get 6
                        i32.const 0
                        i32.lt_s
                        i32.sub
                        i32.const 255
                        i32.and
                        local.tee 6
                        i32.const 1
                        i32.ne
                        br_if 1 (;@9;)
                        local.get 8
                        i32.const 1
                        i32.add
                        local.set 8
                        local.get 19
                        local.get 5
                        i32.const 12
                        i32.add
                        local.tee 5
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      local.get 4
                      local.set 8
                      br 1 (;@8;)
                    end
                    local.get 6
                    i32.const 255
                    i32.ne
                    br_if 4 (;@4;)
                  end
                  local.get 18
                  i32.eqz
                  br_if 2 (;@5;)
                  local.get 18
                  i32.const -1
                  i32.add
                  local.set 18
                  local.get 17
                  local.get 8
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load offset=232
                  local.set 17
                  br 0 (;@7;)
                end
              end
              local.get 1
              i32.const 0
              i32.store offset=152
              local.get 1
              local.get 7
              i32.store offset=144
              local.get 1
              local.get 20
              i32.store offset=140
              local.get 1
              local.get 3
              i32.store offset=136
              local.get 1
              local.get 1
              i32.const 232
              i32.add
              i32.store offset=148
              br 2 (;@3;)
            end
            local.get 1
            local.get 8
            i32.store offset=160
            local.get 1
            i32.const 0
            i32.store offset=156
            local.get 1
            local.get 17
            i32.store offset=152
            local.get 1
            local.get 7
            i32.store offset=144
            local.get 1
            local.get 20
            i32.store offset=140
            local.get 1
            local.get 3
            i32.store offset=136
            local.get 1
            local.get 1
            i32.const 232
            i32.add
            i32.store offset=148
            br 1 (;@3;)
          end
          local.get 1
          local.get 8
          i32.store offset=148
          local.get 1
          local.get 18
          i32.store offset=144
          local.get 1
          local.get 17
          i32.store offset=140
          local.get 1
          i32.const -1
          i32.store offset=136
          local.get 1
          local.get 1
          i32.const 232
          i32.add
          i32.store offset=152
        end
        local.get 1
        i32.const 136
        i32.add
        i64.const 0
        call $_RNvMs2_NtNtNtNtCseFzb18zECBI_5alloc11collections5btree3map5entryINtB5_5EntryNtNtBd_6string6StringyE9or_insertCsbgrmSDC215G_11collections
        local.tee 6
        local.get 6
        i64.load
        i64.const 1
        i64.add
        i64.store
      end
      local.get 1
      i32.load offset=240
      local.set 18
    end
    local.get 1
    i32.load offset=236
    local.set 20
    local.get 1
    local.get 18
    i32.const 0
    local.get 1
    i32.load offset=232
    local.tee 19
    select
    i32.store offset=168
    local.get 1
    local.get 20
    i32.store offset=164
    local.get 1
    local.get 19
    i32.store offset=160
    local.get 1
    i32.const 0
    i32.store offset=156
    local.get 1
    local.get 19
    i32.const 0
    i32.ne
    local.tee 6
    i32.store offset=152
    local.get 1
    local.get 20
    i32.store offset=148
    local.get 1
    local.get 19
    i32.store offset=144
    local.get 1
    i32.const 0
    i32.store offset=140
    local.get 1
    local.get 6
    i32.store offset=136
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 136
    i32.add
    call $_RNvXsk_NtNtNtCseFzb18zECBI_5alloc11collections5btree3mapINtB5_4IterNtNtBb_6string6StringyENtNtNtNtCs8euEcNjxCEf_4core4iter6traits8iterator8Iterator4nextCsbgrmSDC215G_11collections
    i64.const 0
    local.set 10
    block ;; label = @1
      local.get 1
      i32.load offset=8
      local.tee 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.load offset=12
      local.set 9
      i64.const 0
      local.set 10
      loop ;; label = @2
        local.get 10
        local.get 6
        i32.load offset=8
        local.tee 8
        i64.extend_i32_u
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 9
        i64.rotl
        local.set 10
        block ;; label = @3
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.load offset=4
          local.set 17
          block ;; label = @4
            block ;; label = @5
              local.get 8
              i32.const 3
              i32.and
              local.tee 7
              br_if 0 (;@5;)
              local.get 17
              local.set 6
              br 1 (;@4;)
            end
            local.get 17
            local.set 6
            loop ;; label = @5
              local.get 6
              local.tee 5
              i32.const 1
              i32.add
              local.set 6
              local.get 10
              local.get 5
              i64.load8_u
              i64.xor
              i64.const 1099511628211
              i64.mul
              i64.const 9
              i64.rotl
              local.set 10
              local.get 7
              i32.const -1
              i32.add
              local.tee 7
              br_if 0 (;@5;)
            end
          end
          local.get 8
          i32.const 4
          i32.lt_u
          br_if 0 (;@3;)
          local.get 17
          local.get 8
          i32.add
          local.set 7
          loop ;; label = @4
            local.get 10
            local.get 6
            i64.load8_u
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 9
            i64.rotl
            local.get 6
            i32.const 1
            i32.add
            i64.load8_u
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 9
            i64.rotl
            local.get 6
            i32.const 2
            i32.add
            i64.load8_u
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 9
            i64.rotl
            local.get 6
            i32.const 3
            i32.add
            i64.load8_u
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 9
            i64.rotl
            local.set 10
            local.get 6
            i32.const 4
            i32.add
            local.tee 6
            local.get 7
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 9
        i64.load
        local.get 10
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 9
        i64.rotl
        local.set 10
        local.get 1
        local.get 1
        i32.const 136
        i32.add
        call $_RNvXsk_NtNtNtCseFzb18zECBI_5alloc11collections5btree3mapINtB5_4IterNtNtBb_6string6StringyENtNtNtNtCs8euEcNjxCEf_4core4iter6traits8iterator8Iterator4nextCsbgrmSDC215G_11collections
        local.get 1
        i32.load offset=4
        local.set 9
        local.get 1
        i32.load
        local.tee 6
        br_if 0 (;@2;)
      end
    end
    local.get 1
    local.get 18
    i64.extend_i32_u
    i64.store offset=136
    block ;; label = @1
      i32.const 0
      i32.const 16384
      i32.const 0
      i32.load offset=1067864
      local.tee 6
      i32.sub
      local.tee 7
      local.get 7
      i32.const 16384
      i32.gt_u
      select
      local.tee 7
      i32.const 8
      local.get 7
      i32.const 8
      i32.lt_u
      select
      local.tee 5
      local.get 6
      i32.add
      local.tee 7
      local.get 6
      i32.lt_u
      br_if 0 (;@1;)
      local.get 7
      i32.const 16385
      i32.ge_u
      br_if 0 (;@1;)
      local.get 15
      i64.extend_i32_u
      local.set 11
      block ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.const 1051480
        i32.add
        local.get 1
        i32.const 136
        i32.add
        local.get 5
        memory.copy
      end
      local.get 2
      local.get 11
      i64.xor
      local.set 11
      i32.const 0
      local.get 7
      i32.store offset=1067864
      local.get 1
      i32.const 0
      i32.store offset=144
      local.get 1
      i64.const 4294967296
      i64.store offset=136 align=4
      local.get 1
      i32.const 136
      i32.add
      i32.const 0
      i32.const 5
      i32.const 1
      i32.const 1
      call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
      local.get 1
      i32.load offset=140
      local.get 1
      i32.load offset=144
      local.tee 6
      i32.add
      local.tee 7
      i32.const 0
      i32.load offset=1050672 align=1
      i32.store align=1
      local.get 7
      i32.const 0
      i32.load8_u offset=1050676
      i32.store8 offset=4
      local.get 1
      local.get 6
      i32.const 5
      i32.add
      i32.store offset=144
      local.get 1
      i32.const 136
      i32.add
      local.get 0
      call $_RNvCsbgrmSDC215G_11collections8push_u64
      block ;; label = @2
        local.get 1
        i32.load offset=136
        local.get 1
        i32.load offset=144
        local.tee 6
        i32.sub
        i32.const 6
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 136
        i32.add
        local.get 6
        i32.const 7
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=144
        local.set 6
      end
      local.get 11
      i64.const 1099511628211
      i64.mul
      local.set 11
      local.get 1
      i32.load offset=140
      local.get 6
      i32.add
      local.tee 7
      i32.const 0
      i32.load offset=1050677 align=1
      i32.store align=1
      local.get 7
      i32.const 0
      i32.load offset=1050680 align=1
      i32.store offset=3 align=1
      local.get 1
      local.get 6
      i32.const 7
      i32.add
      i32.store offset=144
      local.get 1
      i32.const 136
      i32.add
      local.get 12
      call $_RNvCsbgrmSDC215G_11collections8push_u64
      block ;; label = @2
        local.get 1
        i32.load offset=136
        local.get 1
        i32.load offset=144
        local.tee 6
        i32.sub
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 136
        i32.add
        local.get 6
        i32.const 5
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=144
        local.set 6
      end
      local.get 11
      i64.const 9
      i64.rotl
      local.set 11
      local.get 1
      i32.load offset=140
      local.get 6
      i32.add
      local.tee 7
      i32.const 0
      i32.load offset=1050684 align=1
      i32.store align=1
      local.get 7
      i32.const 0
      i32.load8_u offset=1050688
      i32.store8 offset=4
      local.get 1
      local.get 6
      i32.const 5
      i32.add
      i32.store offset=144
      local.get 1
      i32.const 136
      i32.add
      local.get 23
      call $_RNvCsbgrmSDC215G_11collections8push_u64
      block ;; label = @2
        local.get 1
        i32.load offset=136
        local.get 1
        i32.load offset=144
        local.tee 6
        i32.sub
        i32.const 4
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 136
        i32.add
        local.get 6
        i32.const 5
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=144
        local.set 6
      end
      local.get 10
      local.get 11
      i64.xor
      local.set 26
      local.get 1
      i32.load offset=140
      local.get 6
      i32.add
      local.tee 7
      i32.const 0
      i32.load offset=1050689 align=1
      i32.store align=1
      local.get 7
      i32.const 0
      i32.load8_u offset=1050693
      i32.store8 offset=4
      local.get 1
      local.get 6
      i32.const 5
      i32.add
      i32.store offset=144
      local.get 1
      i32.const 136
      i32.add
      local.get 24
      call $_RNvCsbgrmSDC215G_11collections8push_u64
      block ;; label = @2
        local.get 1
        i32.load offset=136
        local.get 1
        i32.load offset=144
        local.tee 6
        i32.sub
        i32.const 7
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 136
        i32.add
        local.get 6
        i32.const 8
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=144
        local.set 6
      end
      local.get 1
      i32.load offset=140
      local.get 6
      i32.add
      i64.const 4428291212975891488
      i64.store align=1
      local.get 1
      local.get 6
      i32.const 8
      i32.add
      i32.store offset=144
      local.get 1
      i32.const 136
      i32.add
      local.get 26
      call $_RNvCsbgrmSDC215G_11collections8push_u64
      block ;; label = @2
        local.get 1
        i32.load offset=136
        local.tee 7
        local.get 1
        i32.load offset=144
        local.tee 6
        i32.sub
        i32.const 5
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 136
        i32.add
        local.get 6
        i32.const 6
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
        local.get 1
        i32.load offset=136
        local.set 7
        local.get 1
        i32.load offset=144
        local.set 6
      end
      local.get 1
      i32.load offset=140
      local.tee 8
      local.get 6
      i32.add
      local.tee 5
      i32.const 0
      i32.load offset=1050694 align=1
      i32.store align=1
      local.get 5
      i32.const 0
      i32.load16_u offset=1050698 align=1
      i32.store16 offset=4 align=1
      local.get 1
      local.get 6
      i32.const 6
      i32.add
      local.tee 6
      i32.store offset=144
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 16
            local.get 7
            local.get 6
            i32.sub
            i32.le_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 136
            i32.add
            local.get 6
            local.get 16
            i32.const 1
            i32.const 1
            call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsbgrmSDC215G_11collections.llvm.3859857528480129964
            local.get 1
            i32.load offset=140
            local.set 8
            local.get 1
            i32.load offset=144
            local.set 6
            br 1 (;@3;)
          end
          local.get 16
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 16
        i32.eqz
        br_if 0 (;@2;)
        local.get 8
        local.get 6
        i32.add
        local.get 29
        local.get 16
        memory.copy
      end
      block ;; label = @2
        block ;; label = @3
          local.get 6
          local.get 16
          i32.add
          local.tee 6
          i32.const 0
          i32.const 16384
          i32.const 0
          i32.load offset=1067864
          local.tee 7
          i32.sub
          local.tee 5
          local.get 5
          i32.const 16384
          i32.gt_u
          select
          local.tee 5
          local.get 6
          local.get 5
          i32.lt_u
          select
          local.tee 5
          local.get 7
          i32.add
          local.tee 6
          local.get 7
          i32.lt_u
          br_if 0 (;@3;)
          local.get 6
          i32.const 16385
          i32.lt_u
          br_if 1 (;@2;)
        end
        local.get 7
        local.get 6
        i32.const 16384
        i32.const 1050608
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      block ;; label = @2
        local.get 5
        i32.eqz
        br_if 0 (;@2;)
        local.get 7
        i32.const 1051480
        i32.add
        local.get 8
        local.get 5
        memory.copy
      end
      local.get 1
      local.get 10
      local.get 13
      local.get 14
      local.get 27
      local.get 0
      i64.const -7046029254386353131
      i64.add
      local.tee 26
      i64.const 30
      i64.shr_u
      local.get 26
      i64.xor
      i64.const -4658895280553007687
      i64.mul
      local.tee 26
      i64.const 27
      i64.shr_u
      local.get 26
      i64.xor
      i64.const -7723592293110705685
      i64.mul
      local.tee 26
      i64.const 31
      i64.shr_u
      local.get 28
      i64.xor
      local.get 26
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      local.get 11
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      local.get 30
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      local.get 25
      i64.xor
      i64.const 1099511628211
      i64.mul
      i64.const 9
      i64.rotl
      local.tee 10
      i64.store offset=136
      block ;; label = @2
        i32.const 16384
        local.get 6
        i32.sub
        local.tee 7
        i32.const 8
        local.get 7
        i32.const 8
        i32.lt_u
        select
        local.tee 7
        i32.eqz
        br_if 0 (;@2;)
        local.get 6
        i32.const 1051480
        i32.add
        local.get 1
        i32.const 136
        i32.add
        local.get 7
        memory.copy
      end
      i32.const 0
      local.get 7
      local.get 6
      i32.add
      i32.store offset=1067864
      block ;; label = @2
        local.get 19
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 20
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 20
              i32.const 7
              i32.and
              local.tee 7
              br_if 0 (;@5;)
              local.get 20
              local.set 6
              br 1 (;@4;)
            end
            local.get 20
            local.set 6
            loop ;; label = @5
              local.get 6
              i32.const -1
              i32.add
              local.set 6
              local.get 19
              i32.load offset=232
              local.set 19
              local.get 7
              i32.const -1
              i32.add
              local.tee 7
              br_if 0 (;@5;)
            end
          end
          local.get 20
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          loop ;; label = @4
            local.get 19
            i32.load offset=232
            i32.load offset=232
            i32.load offset=232
            i32.load offset=232
            i32.load offset=232
            i32.load offset=232
            i32.load offset=232
            i32.load offset=232
            local.set 19
            local.get 6
            i32.const -8
            i32.add
            local.tee 6
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.set 8
        block ;; label = @3
          local.get 19
          i32.load offset=88
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 8
          loop ;; label = @4
            local.get 8
            i32.const 1
            i32.add
            local.set 6
            block ;; label = @5
              block ;; label = @6
                local.get 19
                i32.load16_u offset=224
                local.tee 5
                local.get 7
                i32.load16_u offset=226
                i32.lt_u
                br_if 0 (;@6;)
                local.get 6
                local.set 8
                local.get 7
                local.set 19
                br 1 (;@5;)
              end
              local.get 7
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              i32.const 236
              i32.add
              local.set 7
              block ;; label = @6
                block ;; label = @7
                  local.get 6
                  i32.const 7
                  i32.and
                  local.tee 5
                  br_if 0 (;@7;)
                  br 1 (;@6;)
                end
                loop ;; label = @7
                  local.get 6
                  i32.const -1
                  i32.add
                  local.set 6
                  local.get 7
                  i32.load
                  local.tee 19
                  i32.const 232
                  i32.add
                  local.set 7
                  local.get 5
                  i32.const -1
                  i32.add
                  local.tee 5
                  br_if 0 (;@7;)
                end
              end
              local.get 8
              i32.const 7
              i32.lt_u
              local.set 5
              i32.const 0
              local.set 8
              local.get 5
              br_if 0 (;@5;)
              loop ;; label = @6
                local.get 7
                i32.load
                i32.load offset=232
                i32.load offset=232
                i32.load offset=232
                i32.load offset=232
                i32.load offset=232
                i32.load offset=232
                i32.load offset=232
                local.tee 19
                i32.const 232
                i32.add
                local.set 7
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
            end
            local.get 19
            i32.load offset=88
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.const 0
        i32.store offset=136
        local.get 1
        local.get 8
        i32.store offset=140
        local.get 1
        i32.const 136
        i32.add
        call $_RNvXNvMss_NtNtNtCseFzb18zECBI_5alloc11collections5btree8navigateINtNtBa_4node7NodeRefNtNtB13_6marker5DyingppNtB1n_14LeafOrInternalE9drop_treeINtB2_9DropGuardNtNtBe_6string6StringyNtNtBe_5alloc6GlobalENtNtNtCs8euEcNjxCEf_4core3ops4drop4Drop4dropCsbgrmSDC215G_11collections
      end
      block ;; label = @2
        local.get 1
        i32.load offset=112
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 1
          i32.load offset=116
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 8
              i32.const 7
              i32.and
              local.tee 7
              br_if 0 (;@5;)
              local.get 8
              local.set 6
              br 1 (;@4;)
            end
            local.get 8
            local.set 6
            loop ;; label = @5
              local.get 6
              i32.const -1
              i32.add
              local.set 6
              local.get 5
              i32.load offset=96
              local.set 5
              local.get 7
              i32.const -1
              i32.add
              local.tee 7
              br_if 0 (;@5;)
            end
          end
          local.get 8
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          loop ;; label = @4
            local.get 5
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            local.set 5
            local.get 6
            i32.const -8
            i32.add
            local.tee 6
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.set 9
        block ;; label = @3
          local.get 5
          i32.load offset=88
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 9
          loop ;; label = @4
            local.get 9
            i32.const 1
            i32.add
            local.set 6
            block ;; label = @5
              block ;; label = @6
                local.get 5
                i32.load16_u offset=92
                local.tee 5
                local.get 7
                i32.load16_u offset=94
                i32.lt_u
                br_if 0 (;@6;)
                local.get 6
                local.set 9
                local.get 7
                local.set 5
                br 1 (;@5;)
              end
              local.get 7
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              i32.const 100
              i32.add
              local.set 7
              block ;; label = @6
                block ;; label = @7
                  local.get 6
                  i32.const 7
                  i32.and
                  local.tee 8
                  br_if 0 (;@7;)
                  br 1 (;@6;)
                end
                loop ;; label = @7
                  local.get 6
                  i32.const -1
                  i32.add
                  local.set 6
                  local.get 7
                  i32.load
                  local.tee 5
                  i32.const 96
                  i32.add
                  local.set 7
                  local.get 8
                  i32.const -1
                  i32.add
                  local.tee 8
                  br_if 0 (;@7;)
                end
              end
              local.get 9
              i32.const 7
              i32.lt_u
              local.set 8
              i32.const 0
              local.set 9
              local.get 8
              br_if 0 (;@5;)
              loop ;; label = @6
                local.get 7
                i32.load
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                local.tee 5
                i32.const 96
                i32.add
                local.set 7
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
            end
            local.get 5
            i32.load offset=88
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.const 0
        i32.store offset=136
        local.get 1
        local.get 9
        i32.store offset=140
        local.get 1
        i32.const 136
        i32.add
        call $_RNvXNvMss_NtNtNtCseFzb18zECBI_5alloc11collections5btree8navigateINtNtBa_4node7NodeRefNtNtB13_6marker5DyingppNtB1n_14LeafOrInternalE9drop_treeINtB2_9DropGuardyNtNtBa_7set_val9SetValZSTNtNtBe_5alloc6GlobalENtNtNtCs8euEcNjxCEf_4core3ops4drop4Drop4dropCsbgrmSDC215G_11collections
      end
      block ;; label = @2
        local.get 1
        i32.load offset=100
        local.tee 5
        i32.eqz
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 1
          i32.load offset=104
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            block ;; label = @5
              local.get 8
              i32.const 7
              i32.and
              local.tee 7
              br_if 0 (;@5;)
              local.get 8
              local.set 6
              br 1 (;@4;)
            end
            local.get 8
            local.set 6
            loop ;; label = @5
              local.get 6
              i32.const -1
              i32.add
              local.set 6
              local.get 5
              i32.load offset=96
              local.set 5
              local.get 7
              i32.const -1
              i32.add
              local.tee 7
              br_if 0 (;@5;)
            end
          end
          local.get 8
          i32.const 8
          i32.lt_u
          br_if 0 (;@3;)
          loop ;; label = @4
            local.get 5
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            i32.load offset=96
            local.set 5
            local.get 6
            i32.const -8
            i32.add
            local.tee 6
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.set 9
        block ;; label = @3
          local.get 5
          i32.load offset=88
          local.tee 7
          i32.eqz
          br_if 0 (;@3;)
          i32.const 0
          local.set 9
          loop ;; label = @4
            local.get 9
            i32.const 1
            i32.add
            local.set 6
            block ;; label = @5
              block ;; label = @6
                local.get 5
                i32.load16_u offset=92
                local.tee 5
                local.get 7
                i32.load16_u offset=94
                i32.lt_u
                br_if 0 (;@6;)
                local.get 6
                local.set 9
                local.get 7
                local.set 5
                br 1 (;@5;)
              end
              local.get 7
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              i32.const 100
              i32.add
              local.set 7
              block ;; label = @6
                block ;; label = @7
                  local.get 6
                  i32.const 7
                  i32.and
                  local.tee 8
                  br_if 0 (;@7;)
                  br 1 (;@6;)
                end
                loop ;; label = @7
                  local.get 6
                  i32.const -1
                  i32.add
                  local.set 6
                  local.get 7
                  i32.load
                  local.tee 5
                  i32.const 96
                  i32.add
                  local.set 7
                  local.get 8
                  i32.const -1
                  i32.add
                  local.tee 8
                  br_if 0 (;@7;)
                end
              end
              local.get 9
              i32.const 7
              i32.lt_u
              local.set 8
              i32.const 0
              local.set 9
              local.get 8
              br_if 0 (;@5;)
              loop ;; label = @6
                local.get 7
                i32.load
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                i32.load offset=96
                local.tee 5
                i32.const 96
                i32.add
                local.set 7
                local.get 6
                i32.const -8
                i32.add
                local.tee 6
                br_if 0 (;@6;)
              end
            end
            local.get 5
            i32.load offset=88
            local.tee 7
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.const 0
        i32.store offset=136
        local.get 1
        local.get 9
        i32.store offset=140
        local.get 1
        i32.const 136
        i32.add
        call $_RNvXNvMss_NtNtNtCseFzb18zECBI_5alloc11collections5btree8navigateINtNtBa_4node7NodeRefNtNtB13_6marker5DyingppNtB1n_14LeafOrInternalE9drop_treeINtB2_9DropGuardyNtNtBa_7set_val9SetValZSTNtNtBe_5alloc6GlobalENtNtNtCs8euEcNjxCEf_4core3ops4drop4Drop4dropCsbgrmSDC215G_11collections
      end
      local.get 1
      i32.const 272
      i32.add
      global.set $__stack_pointer
      local.get 10
      return
    end
    local.get 6
    local.get 7
    i32.const 16384
    i32.const 1050608
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )

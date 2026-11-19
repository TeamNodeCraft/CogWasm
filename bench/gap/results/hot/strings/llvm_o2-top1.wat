(func $run (;8;) (type 8) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i64 i64 i64 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i32 i64)
    global.get $__stack_pointer
    i32.const 256
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1066376
          br_if 0 (;@3;)
          block ;; label = @4
            memory.size
            i32.const 16
            i32.shl
            local.tee 2
            br_if 0 (;@4;)
            i32.const 65536
            local.set 2
            i32.const 1
            memory.grow
            i32.const -1
            i32.eq
            br_if 3 (;@1;)
          end
          i32.const 0
          local.get 2
          i32.store offset=1066372
          i32.const 0
          local.get 2
          i32.store offset=1066368
          i32.const 0
          i32.const 1
          i32.store8 offset=1066376
          local.get 2
          i32.const 6144
          i32.or
          local.set 3
          local.get 2
          local.set 4
          br 1 (;@2;)
        end
        i32.const 0
        i32.load offset=1066368
        local.tee 2
        i32.const -6148
        i32.gt_u
        br_if 1 (;@1;)
        local.get 2
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.tee 4
        i32.const 6144
        i32.add
        local.set 3
        i32.const 0
        i32.load offset=1066372
        local.set 2
      end
      block ;; label = @2
        local.get 3
        local.get 2
        i32.le_u
        br_if 0 (;@2;)
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
        br_if 1 (;@1;)
        i32.const 0
        local.get 5
        local.get 2
        i32.add
        i32.const 16
        i32.shl
        i32.store offset=1066372
      end
      i32.const 0
      local.set 6
      i32.const 0
      local.get 3
      i32.store offset=1066368
      local.get 4
      i32.eqz
      br_if 0 (;@1;)
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
      local.set 7
      local.get 1
      i32.const 0
      i32.store offset=252
      local.get 1
      local.get 4
      i32.store offset=248
      local.get 1
      i32.const 512
      i32.store offset=244
      loop ;; label = @2
        i32.const 0
        local.set 2
        local.get 1
        i32.const 0
        i32.store offset=216
        local.get 1
        i64.const 4294967296
        i64.store offset=208 align=4
        local.get 7
        i64.const 4354685564936845354
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
        i64.const 6
        i64.rem_u
        i32.wrap_i64
        i32.const 2
        i32.add
        local.set 8
        local.get 7
        i64.const -7046029254386353131
        i64.add
        local.set 7
        i32.const 1
        local.set 9
        i32.const 0
        local.set 3
        loop ;; label = @3
          block ;; label = @4
            local.get 2
            local.tee 10
            i32.eqz
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 1
              i32.load offset=208
              local.get 3
              i32.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 208
              i32.add
              local.get 3
              i32.const 1
              call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
              local.get 1
              i32.load offset=212
              local.set 9
            end
            local.get 9
            local.get 3
            i32.add
            i32.const 32
            i32.store8
            local.get 1
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            i32.store offset=216
          end
          local.get 7
          i64.const -2691343689449507777
          i64.add
          local.tee 0
          i64.const 30
          i64.shr_u
          local.get 0
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
          local.set 12
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 7
                i64.const 4354685564936845354
                i64.add
                local.tee 11
                i64.const 30
                i64.shr_u
                local.get 11
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 13
                i64.const 27
                i64.shr_u
                local.get 13
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 13
                i64.const 31
                i64.shr_u
                local.get 13
                i64.xor
                i64.const 5
                i64.rem_u
                i64.const 0
                i64.ne
                br_if 0 (;@6;)
                local.get 12
                i64.const 2001
                i64.rem_u
                i64.const -1000
                i64.add
                local.set 13
                local.get 11
                local.set 7
                br 1 (;@5;)
              end
              local.get 12
              i64.const 20
              i64.rem_u
              i32.wrap_i64
              i32.const 3
              i32.shl
              local.tee 2
              i32.load offset=1049296
              local.set 5
              block ;; label = @6
                local.get 2
                i32.load offset=1049300
                local.tee 2
                local.get 1
                i32.load offset=208
                local.get 3
                i32.sub
                i32.le_u
                br_if 0 (;@6;)
                local.get 1
                i32.const 208
                i32.add
                local.get 3
                local.get 2
                call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
                local.get 1
                i32.load offset=216
                local.set 3
              end
              local.get 1
              i32.load offset=212
              local.set 9
              block ;; label = @6
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                local.get 9
                local.get 3
                i32.add
                local.get 5
                local.get 2
                memory.copy
              end
              local.get 1
              local.get 3
              local.get 2
              i32.add
              local.tee 3
              i32.store offset=216
              block ;; label = @6
                local.get 7
                i64.const 8709371129873690708
                i64.add
                local.tee 11
                i64.const 30
                i64.shr_u
                local.get 11
                i64.xor
                i64.const -4658895280553007687
                i64.mul
                local.tee 13
                i64.const 27
                i64.shr_u
                local.get 13
                i64.xor
                i64.const -7723592293110705685
                i64.mul
                local.tee 13
                i64.const 31
                i64.shr_u
                local.get 13
                i64.xor
                i64.const 7
                i64.and
                i64.eqz
                br_if 0 (;@6;)
                local.get 0
                local.set 7
                br 2 (;@4;)
              end
              block ;; label = @6
                local.get 1
                i32.load offset=208
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 1
                i32.const 208
                i32.add
                local.get 3
                i32.const 1
                call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
                local.get 1
                i32.load offset=212
                local.set 9
              end
              local.get 9
              local.get 3
              i32.add
              i32.const 95
              i32.store8
              local.get 1
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.store offset=216
              local.get 7
              i64.const 1663341875487337577
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
              i64.const 100
              i64.rem_u
              local.set 13
              local.get 11
              local.set 7
            end
            block ;; label = @5
              local.get 13
              i64.const 0
              i64.ne
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 1
                i32.load offset=208
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 1
                i32.const 208
                i32.add
                local.get 3
                i32.const 1
                call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
              end
              local.get 1
              i32.load offset=212
              local.tee 9
              local.get 3
              i32.add
              i32.const 48
              i32.store8
              local.get 1
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.store offset=216
              br 1 (;@4;)
            end
            i32.const 0
            local.set 2
            local.get 1
            i32.const 0
            i32.store offset=96
            local.get 1
            i64.const 0
            i64.store offset=88
            local.get 1
            i64.const 0
            i64.store offset=80
            local.get 13
            local.get 13
            i64.const 63
            i64.shr_s
            local.tee 0
            i64.xor
            local.get 0
            i64.sub
            local.set 0
            i32.const 1
            local.set 5
            block ;; label = @5
              loop ;; label = @6
                local.get 1
                i32.const 80
                i32.add
                local.get 5
                i32.add
                i32.const -1
                i32.add
                local.get 0
                local.get 0
                i64.const 10
                i64.div_u
                local.tee 11
                i64.const 10
                i64.mul
                i64.sub
                i32.wrap_i64
                i32.const 48
                i32.or
                i32.store8
                local.get 0
                i64.const 10
                i64.lt_u
                br_if 1 (;@5;)
                local.get 2
                i32.const 1
                i32.add
                local.set 2
                local.get 11
                local.set 0
                local.get 5
                i32.const 1
                i32.add
                local.tee 5
                i32.const 21
                i32.ne
                br_if 0 (;@6;)
              end
              i32.const 20
              i32.const 20
              i32.const 1049456
              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
              unreachable
            end
            local.get 1
            i32.load offset=208
            local.set 14
            block ;; label = @5
              local.get 13
              i64.const 0
              i64.ge_s
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 14
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                local.get 1
                i32.const 208
                i32.add
                local.get 3
                i32.const 1
                call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
                local.get 1
                i32.load offset=208
                local.set 14
              end
              local.get 1
              i32.load offset=212
              local.get 3
              i32.add
              i32.const 45
              i32.store8
              local.get 1
              local.get 3
              i32.const 1
              i32.add
              local.tee 3
              i32.store offset=216
            end
            loop ;; label = @5
              block ;; label = @6
                i32.const 1
                i32.const 2
                local.get 1
                i32.const 80
                i32.add
                local.get 2
                i32.add
                i32.load8_s
                local.tee 5
                i32.const -1
                i32.gt_s
                local.tee 15
                select
                local.tee 16
                local.get 14
                local.get 3
                i32.sub
                i32.le_u
                br_if 0 (;@6;)
                local.get 1
                i32.const 208
                i32.add
                local.get 3
                local.get 16
                call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
                local.get 1
                i32.load offset=208
                local.set 14
              end
              local.get 1
              i32.load offset=212
              local.tee 9
              local.get 3
              i32.add
              local.set 17
              block ;; label = @6
                local.get 15
                br_if 0 (;@6;)
                local.get 17
                local.get 5
                i32.const 191
                i32.and
                i32.store8 offset=1
                local.get 5
                i32.const 192
                i32.and
                i32.const 6
                i32.shr_u
                i32.const -64
                i32.or
                local.set 5
              end
              local.get 17
              local.get 5
              i32.store8
              local.get 1
              local.get 16
              local.get 3
              i32.add
              local.tee 3
              i32.store offset=216
              local.get 2
              i32.const -1
              i32.add
              local.tee 2
              i32.const -1
              i32.ne
              br_if 0 (;@5;)
            end
          end
          local.get 10
          i32.const 1
          i32.add
          local.set 2
          local.get 10
          local.get 8
          i32.ne
          br_if 0 (;@3;)
        end
        block ;; label = @3
          local.get 6
          local.get 1
          i32.load offset=244
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 244
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecNtNtB7_6string6StringE8grow_oneCs9hsE6Fh6sWE_7strings
          local.get 1
          i32.load offset=248
          local.set 4
        end
        local.get 1
        i64.load offset=208 align=4
        local.set 0
        local.get 4
        local.get 6
        i32.const 12
        i32.mul
        i32.add
        local.tee 2
        local.get 1
        i32.load offset=216
        i32.store offset=8
        local.get 2
        local.get 0
        i64.store align=4
        local.get 1
        local.get 6
        i32.const 1
        i32.add
        local.tee 6
        i32.store offset=252
        local.get 6
        i32.const 512
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 1
      i32.load offset=248
      local.set 18
      local.get 1
      i32.const 80
      i32.add
      i32.const 0
      i32.const 128
      memory.fill
      i64.const 4354685564936845355
      local.set 19
      i64.const -4942790177534073029
      local.set 0
      i64.const 7640891576956012809
      local.set 11
      i64.const 0
      local.set 20
      i64.const 0
      local.set 21
      i64.const 0
      local.set 13
      i64.const 0
      local.set 12
      i64.const 0
      local.set 22
      i64.const 0
      local.set 23
      i32.const 0
      local.set 15
      i32.const 0
      local.set 4
      loop ;; label = @2
        block ;; label = @3
          local.get 18
          local.get 4
          i32.add
          local.tee 24
          i32.load offset=8
          local.tee 9
          i32.const 0
          i32.ge_s
          br_if 0 (;@3;)
          i32.const 0
          local.get 9
          call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
          unreachable
        end
        local.get 24
        i32.load offset=4
        local.set 3
        block ;; label = @3
          block ;; label = @4
            local.get 9
            br_if 0 (;@4;)
            i32.const 1
            local.set 9
            i32.const 0
            local.set 2
            i32.const 0
            local.set 16
            br 1 (;@3;)
          end
          call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.load8_u offset=1066376
                  i32.eqz
                  br_if 0 (;@7;)
                  i32.const 0
                  i32.load offset=1066368
                  local.set 8
                  br 1 (;@6;)
                end
                block ;; label = @7
                  memory.size
                  i32.const 16
                  i32.shl
                  local.tee 8
                  br_if 0 (;@7;)
                  i32.const 1
                  local.set 17
                  i32.const 65536
                  local.set 8
                  i32.const 1
                  memory.grow
                  i32.const -1
                  i32.eq
                  br_if 2 (;@5;)
                end
                i32.const 0
                local.get 8
                i32.store offset=1066372
                i32.const 0
                local.get 8
                i32.store offset=1066368
                i32.const 0
                i32.const 1
                i32.store8 offset=1066376
              end
              i32.const 1
              local.set 17
              local.get 8
              local.get 9
              i32.add
              local.tee 2
              local.get 8
              i32.lt_u
              br_if 0 (;@5;)
              block ;; label = @6
                local.get 2
                i32.const 0
                i32.load offset=1066372
                local.tee 5
                i32.le_u
                br_if 0 (;@6;)
                local.get 2
                local.get 5
                i32.sub
                local.tee 5
                i32.const 16
                i32.shr_u
                local.get 5
                i32.const 65535
                i32.and
                i32.const 0
                i32.ne
                i32.add
                local.tee 5
                memory.grow
                local.tee 16
                i32.const -1
                i32.eq
                br_if 1 (;@5;)
                i32.const 0
                local.get 16
                local.get 5
                i32.add
                i32.const 16
                i32.shl
                i32.store offset=1066372
              end
              i32.const 0
              local.get 2
              i32.store offset=1066368
              local.get 8
              br_if 1 (;@4;)
            end
            local.get 17
            local.get 9
            call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
            unreachable
          end
          i32.const 0
          local.set 16
          local.get 1
          i32.const 0
          i32.store offset=216
          local.get 1
          local.get 8
          i32.store offset=212
          local.get 1
          local.get 9
          i32.store offset=208
          local.get 9
          local.set 6
          loop ;; label = @4
            i32.const 32
            i32.const 0
            local.get 3
            i32.load8_s
            local.tee 2
            i32.const -97
            i32.add
            i32.const 255
            i32.and
            i32.const 26
            i32.lt_u
            select
            local.set 5
            block ;; label = @5
              i32.const 1
              i32.const 2
              local.get 2
              i32.const -1
              i32.gt_s
              local.tee 10
              select
              local.tee 14
              local.get 6
              local.get 16
              i32.sub
              i32.le_u
              br_if 0 (;@5;)
              local.get 1
              i32.const 208
              i32.add
              local.get 16
              local.get 14
              call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs9hsE6Fh6sWE_7strings
              local.get 1
              i32.load offset=212
              local.set 8
              local.get 1
              i32.load offset=208
              local.set 6
            end
            local.get 5
            local.get 2
            i32.xor
            local.set 5
            local.get 8
            local.get 16
            i32.add
            local.set 17
            block ;; label = @5
              local.get 10
              br_if 0 (;@5;)
              local.get 17
              local.get 5
              i32.const 191
              i32.and
              i32.store8 offset=1
              local.get 2
              i32.const 192
              i32.and
              i32.const 6
              i32.shr_u
              i32.const -64
              i32.or
              local.set 5
            end
            local.get 17
            local.get 5
            i32.store8
            local.get 1
            local.get 14
            local.get 16
            i32.add
            local.tee 16
            i32.store offset=216
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            br_if 0 (;@4;)
          end
          local.get 24
          i32.load offset=8
          local.set 2
          local.get 24
          i32.load offset=4
          local.set 3
          local.get 1
          i32.load offset=212
          local.set 9
        end
        local.get 1
        i32.const 1
        i32.store16 offset=236
        local.get 1
        i32.const 0
        i32.store offset=232
        local.get 1
        local.get 3
        local.get 2
        i32.add
        i32.store offset=228
        local.get 1
        local.get 3
        i32.store offset=224
        local.get 1
        local.get 2
        i32.store offset=220
        local.get 1
        local.get 3
        i32.store offset=216
        local.get 1
        local.get 2
        i32.store offset=212
        local.get 1
        i32.const 0
        i32.store offset=208
        local.get 1
        i32.const 72
        i32.add
        local.get 1
        i32.const 208
        i32.add
        call $_RINvYINtNtNtCs8euEcNjxCEf_4core3str4iter5SplitNtB8_12IsWhitespaceENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvB12_4find5checkReQNtB8_10IsNotEmptyE0INtNtNtBa_3ops12control_flow11ControlFlowB2d_EECs9hsE6Fh6sWE_7strings
        block ;; label = @3
          local.get 1
          i32.load offset=72
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=76
          local.set 17
          loop ;; label = @4
            local.get 1
            i32.const 80
            i32.add
            local.get 17
            i32.const 15
            local.get 17
            i32.const 15
            i32.lt_u
            select
            i32.const 3
            i32.shl
            i32.add
            local.tee 2
            local.get 2
            i64.load
            i64.const 1
            i64.add
            i64.store
            i32.const 0
            local.set 2
            loop ;; label = @5
              local.get 5
              local.get 2
              i32.add
              i32.load8_u
              local.tee 3
              i64.extend_i32_u
              i64.const 255
              i64.and
              local.set 7
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const -48
                  i32.add
                  i32.const 255
                  i32.and
                  i32.const 10
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 13
                  local.get 3
                  i32.const 223
                  i32.and
                  i32.const -65
                  i32.add
                  i32.const 255
                  i32.and
                  i32.const 26
                  i32.lt_u
                  i64.extend_i32_u
                  i64.add
                  local.set 13
                  br 1 (;@6;)
                end
                local.get 12
                i64.const 1
                i64.add
                local.set 12
              end
              local.get 11
              i64.const 7
              i64.rotl
              local.get 7
              i64.xor
              local.set 11
              local.get 17
              local.get 2
              i32.const 1
              i32.add
              local.tee 2
              i32.ne
              br_if 0 (;@5;)
            end
            block ;; label = @5
              local.get 17
              i32.const 1
              i32.eq
              local.get 5
              i32.load8_u
              local.tee 2
              i32.const -58
              i32.add
              i32.const 255
              i32.and
              i32.const 246
              i32.lt_u
              local.get 2
              i32.const 45
              i32.eq
              select
              br_if 0 (;@5;)
              block ;; label = @6
                block ;; label = @7
                  local.get 2
                  i32.const 45
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 17
                  i32.const -1
                  i32.add
                  local.set 3
                  local.get 5
                  i32.const 1
                  i32.add
                  local.set 2
                  block ;; label = @8
                    local.get 17
                    i32.const 17
                    i32.lt_u
                    br_if 0 (;@8;)
                    i64.const 0
                    local.set 7
                    loop ;; label = @9
                      local.get 1
                      i32.const 32
                      i32.add
                      local.get 7
                      local.get 7
                      i64.const 63
                      i64.shr_s
                      i64.const 10
                      i64.const 0
                      call $__multi3
                      local.get 1
                      i64.load offset=40
                      local.get 1
                      i64.load offset=32
                      local.tee 7
                      i64.const 63
                      i64.shr_s
                      i64.ne
                      br_if 4 (;@5;)
                      local.get 2
                      i32.load8_u
                      i32.const -48
                      i32.add
                      local.tee 5
                      i32.const 9
                      i32.gt_u
                      br_if 4 (;@5;)
                      local.get 7
                      local.get 5
                      i64.extend_i32_u
                      local.tee 25
                      i64.lt_s
                      local.get 7
                      local.get 25
                      i64.sub
                      local.tee 7
                      i64.const 0
                      i64.lt_s
                      i32.xor
                      br_if 4 (;@5;)
                      local.get 2
                      i32.const 1
                      i32.add
                      local.set 2
                      local.get 3
                      i32.const -1
                      i32.add
                      local.tee 3
                      br_if 0 (;@9;)
                      br 3 (;@6;)
                    end
                  end
                  i64.const 0
                  local.set 7
                  local.get 3
                  i32.eqz
                  br_if 1 (;@6;)
                  loop ;; label = @8
                    local.get 2
                    i32.load8_u
                    i32.const -48
                    i32.add
                    local.tee 5
                    i32.const 9
                    i32.gt_u
                    br_if 3 (;@5;)
                    local.get 2
                    i32.const 1
                    i32.add
                    local.set 2
                    local.get 7
                    i64.const 10
                    i64.mul
                    local.get 5
                    i64.extend_i32_u
                    i64.sub
                    local.set 7
                    local.get 3
                    i32.const -1
                    i32.add
                    local.tee 3
                    br_if 0 (;@8;)
                    br 2 (;@6;)
                  end
                end
                block ;; label = @7
                  local.get 17
                  i32.const 16
                  i32.lt_u
                  br_if 0 (;@7;)
                  i64.const 0
                  local.set 7
                  local.get 17
                  local.set 2
                  loop ;; label = @8
                    local.get 1
                    i32.const 48
                    i32.add
                    local.get 7
                    local.get 7
                    i64.const 63
                    i64.shr_s
                    i64.const 10
                    i64.const 0
                    call $__multi3
                    local.get 1
                    i64.load offset=56
                    local.get 1
                    i64.load offset=48
                    local.tee 7
                    i64.const 63
                    i64.shr_s
                    i64.ne
                    br_if 3 (;@5;)
                    local.get 5
                    i32.load8_u
                    i32.const -48
                    i32.add
                    local.tee 3
                    i32.const 9
                    i32.gt_u
                    br_if 3 (;@5;)
                    local.get 7
                    i64.const 0
                    i64.lt_s
                    local.get 3
                    i64.extend_i32_u
                    local.tee 25
                    local.get 7
                    i64.add
                    local.tee 7
                    local.get 25
                    i64.lt_s
                    i32.xor
                    br_if 3 (;@5;)
                    local.get 5
                    i32.const 1
                    i32.add
                    local.set 5
                    local.get 2
                    i32.const -1
                    i32.add
                    local.tee 2
                    br_if 0 (;@8;)
                    br 2 (;@6;)
                  end
                end
                i64.const 0
                local.set 7
                local.get 17
                local.set 2
                loop ;; label = @7
                  local.get 5
                  i32.load8_u
                  i32.const -48
                  i32.add
                  local.tee 3
                  i32.const 9
                  i32.gt_u
                  br_if 2 (;@5;)
                  local.get 5
                  i32.const 1
                  i32.add
                  local.set 5
                  local.get 7
                  i64.const 10
                  i64.mul
                  local.get 3
                  i64.extend_i32_u
                  i64.add
                  local.set 7
                  local.get 2
                  i32.const -1
                  i32.add
                  local.tee 2
                  br_if 0 (;@7;)
                end
              end
              local.get 7
              local.get 23
              i64.add
              local.set 23
              local.get 22
              i64.const 1
              i64.add
              local.set 22
              local.get 7
              local.get 19
              i64.const 11
              i64.rotl
              i64.xor
              local.set 19
            end
            local.get 17
            local.get 15
            local.get 17
            local.get 15
            i32.gt_u
            select
            local.set 15
            local.get 20
            i64.const 1
            i64.add
            local.set 20
            local.get 21
            local.get 17
            i64.extend_i32_u
            i64.add
            local.set 21
            local.get 1
            i32.const 24
            i32.add
            local.get 1
            i32.const 208
            i32.add
            call $_RINvYINtNtNtCs8euEcNjxCEf_4core3str4iter5SplitNtB8_12IsWhitespaceENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvB12_4find5checkReQNtB8_10IsNotEmptyE0INtNtNtBa_3ops12control_flow11ControlFlowB2d_EECs9hsE6Fh6sWE_7strings
            local.get 1
            i32.load offset=28
            local.set 17
            local.get 1
            i32.load offset=24
            local.tee 5
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.const 1
        i32.store16 offset=236
        local.get 1
        i32.const 0
        i32.store offset=232
        local.get 1
        local.get 9
        local.get 16
        i32.add
        i32.store offset=228
        local.get 1
        local.get 9
        i32.store offset=224
        local.get 1
        local.get 16
        i32.store offset=220
        local.get 1
        local.get 9
        i32.store offset=216
        local.get 1
        local.get 16
        i32.store offset=212
        local.get 1
        i32.const 0
        i32.store offset=208
        local.get 1
        i32.const 16
        i32.add
        local.get 1
        i32.const 208
        i32.add
        call $_RINvYINtNtNtCs8euEcNjxCEf_4core3str4iter5SplitNtB8_12IsWhitespaceENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvB12_4find5checkReQNtB8_10IsNotEmptyE0INtNtNtBa_3ops12control_flow11ControlFlowB2d_EECs9hsE6Fh6sWE_7strings
        block ;; label = @3
          local.get 1
          i32.load offset=16
          local.tee 17
          i32.eqz
          br_if 0 (;@3;)
          local.get 1
          i32.load offset=20
          local.set 2
          loop ;; label = @4
            local.get 2
            i32.const 3
            i32.and
            local.set 3
            i32.const 0
            local.set 5
            block ;; label = @5
              block ;; label = @6
                local.get 2
                i32.const -1
                i32.add
                i32.const 3
                i32.lt_u
                br_if 0 (;@6;)
                local.get 2
                i32.const -4
                i32.and
                local.set 16
                i32.const 0
                local.set 5
                loop ;; label = @7
                  local.get 0
                  i64.const 5
                  i64.rotl
                  local.get 17
                  local.get 5
                  i32.add
                  local.tee 2
                  i64.load8_u
                  i64.xor
                  i64.const 5
                  i64.rotl
                  local.get 2
                  i32.const 1
                  i32.add
                  i64.load8_u
                  i64.xor
                  i64.const 5
                  i64.rotl
                  local.get 2
                  i32.const 2
                  i32.add
                  i64.load8_u
                  i64.xor
                  i64.const 5
                  i64.rotl
                  local.get 2
                  i32.const 3
                  i32.add
                  i64.load8_u
                  i64.xor
                  local.set 0
                  local.get 16
                  local.get 5
                  i32.const 4
                  i32.add
                  local.tee 5
                  i32.ne
                  br_if 0 (;@7;)
                end
                local.get 3
                i32.eqz
                br_if 1 (;@5;)
              end
              local.get 17
              local.get 5
              i32.add
              local.set 2
              loop ;; label = @6
                local.get 0
                i64.const 5
                i64.rotl
                local.get 2
                i64.load8_u
                i64.xor
                local.set 0
                local.get 2
                i32.const 1
                i32.add
                local.set 2
                local.get 3
                i32.const -1
                i32.add
                local.tee 3
                br_if 0 (;@6;)
              end
            end
            local.get 1
            i32.const 8
            i32.add
            local.get 1
            i32.const 208
            i32.add
            call $_RINvYINtNtNtCs8euEcNjxCEf_4core3str4iter5SplitNtB8_12IsWhitespaceENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvB12_4find5checkReQNtB8_10IsNotEmptyE0INtNtNtBa_3ops12control_flow11ControlFlowB2d_EECs9hsE6Fh6sWE_7strings
            local.get 1
            i32.load offset=12
            local.set 2
            local.get 1
            i32.load offset=8
            local.tee 17
            br_if 0 (;@4;)
          end
        end
        local.get 4
        i32.const 12
        i32.add
        local.tee 4
        i32.const 6144
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 1
      i64.const 512
      i64.store offset=208
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.const 16384
          i32.const 0
          i32.load offset=1066364
          local.tee 3
          i32.sub
          local.tee 2
          local.get 2
          i32.const 16384
          i32.gt_u
          select
          local.tee 2
          i32.const 8
          local.get 2
          i32.const 8
          i32.lt_u
          select
          local.tee 5
          local.get 3
          i32.add
          local.tee 2
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          i32.const 16385
          i32.ge_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 5
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 5
            memory.copy
          end
          local.get 1
          local.get 20
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 21
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 13
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 12
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 22
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 23
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 15
          i64.extend_i32_u
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          i32.const 0
          local.get 3
          local.get 2
          i32.add
          local.tee 2
          i32.store offset=1066364
          i32.const 0
          local.set 5
          loop ;; label = @4
            local.get 1
            local.get 1
            i32.const 80
            i32.add
            local.get 5
            i32.add
            i64.load
            i64.store offset=208
            i32.const 0
            i32.const 16384
            local.get 2
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
            local.tee 17
            local.get 2
            i32.add
            local.tee 3
            i32.const 16385
            i32.ge_u
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 17
              i32.eqz
              br_if 0 (;@5;)
              local.get 2
              i32.const 1049980
              i32.add
              local.get 1
              i32.const 208
              i32.add
              local.get 17
              memory.copy
            end
            i32.const 0
            local.get 3
            i32.store offset=1066364
            local.get 3
            local.set 2
            local.get 5
            i32.const 8
            i32.add
            local.tee 5
            i32.const 128
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 1
          local.get 11
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            i32.sub
            local.tee 2
            i32.const 8
            local.get 2
            i32.const 8
            i32.lt_u
            select
            local.tee 2
            i32.eqz
            br_if 0 (;@4;)
            local.get 3
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 2
            memory.copy
          end
          local.get 1
          local.get 0
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 2
            local.get 3
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          local.get 1
          local.get 19
          i64.store offset=208
          block ;; label = @4
            i32.const 16384
            local.get 3
            local.get 2
            i32.add
            local.tee 2
            i32.sub
            local.tee 3
            i32.const 8
            local.get 3
            i32.const 8
            i32.lt_u
            select
            local.tee 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 1049980
            i32.add
            local.get 1
            i32.const 208
            i32.add
            local.get 3
            memory.copy
          end
          i32.const 0
          local.get 3
          local.get 2
          i32.add
          i32.store offset=1066364
          local.get 1
          i32.const 256
          i32.add
          global.set $__stack_pointer
          local.get 19
          i64.const 29
          i64.rotl
          local.get 21
          i64.const 43
          i64.rotl
          i64.xor
          local.get 0
          i64.const 13
          i64.rotl
          i64.xor
          local.get 11
          i64.xor
          local.get 23
          i64.xor
          return
        end
        local.get 3
        local.get 2
        i32.const 16384
        i32.const 1049176
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      local.get 2
      local.get 3
      i32.const 16384
      i32.const 1049176
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    i32.const 4
    i32.const 6144
    call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
    unreachable
  )

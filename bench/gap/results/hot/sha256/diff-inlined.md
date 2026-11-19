# sha256::compress vs LLVM's inlined `run`

LLVM -O2 inlined the whole program into `run`; this is the hot function that survives as a separate body in our module.

## Ours (56,502,100 instructions, 15.6%)

```wat
(func $_RNvCs9103VBb1EJO_6sha2568compress (;5;) (type 7) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 528
    i32.sub
    local.tee 17
    global.set $__stack_pointer
    local.get 17
    i32.const 0
    i32.add
    local.set 3
    i32.const 0
    local.set 4
    loop ;; label = @1
      local.get 4
      i32.const 64
      i32.eq
      if ;; label = @2
        local.get 17
        i32.const 256
        i32.add
        local.tee 4
        i32.const 1
        i32.add
        local.set 5
        local.get 4
        i32.const 2
        i32.add
        local.set 6
        i32.const 0
        local.set 7
        loop ;; label = @3
          local.get 7
          i32.const 16
          i32.lt_u
          if ;; label = @4
            local.get 7
            i32.const 2
            i32.shl
            local.tee 13
            local.get 2
            i32.lt_u
            if ;; label = @5
              local.get 1
              local.get 13
              i32.add
              i32.load8_u
              i32.const 255
              i32.and
              local.set 12
              local.get 13
              i32.const 1
              i32.add
              local.tee 11
              local.get 2
              i32.lt_u
              if ;; label = @6
                local.get 1
                local.get 11
                i32.add
                i32.load8_u
                i32.const 255
                i32.and
                local.set 11
                local.get 13
                i32.const 2
                i32.add
                local.tee 10
                local.get 2
                i32.lt_u
                if ;; label = @7
                  local.get 1
                  local.get 10
                  i32.add
                  i32.load8_u
                  i32.const 255
                  i32.and
                  local.set 10
                  local.get 13
                  i32.const 3
                  i32.add
                  local.get 2
                  i32.lt_u
                  if ;; label = @8
                    local.get 1
                    local.get 13
                    i32.const 3
                    i32.add
                    i32.add
                    i32.load8_u
                    i32.const 255
                    i32.and
                    local.set 9
                    local.get 4
                    local.get 12
                    i32.store8
                    local.get 5
                    local.get 11
                    i32.store8
                    local.get 6
                    local.get 10
                    i32.store8
                    local.get 4
                    i32.const 3
                    i32.add
                    local.get 9
                    i32.store8
                    local.get 4
                    i32.load align=1
                    local.set 9
                    local.get 7
                    i32.const 64
                    i32.lt_u
                    if ;; label = @9
                      local.get 3
                      local.get 13
                      i32.add
                      local.get 9
                      local.tee 18
                      i32.const 16711935
                      i32.and
                      i32.const 8
                      i32.rotr
                      local.get 18
                      i32.const -16711936
                      i32.and
                      i32.const 8
                      i32.rotl
                      i32.or
                      i32.store align=1
                      local.get 7
                      i32.const 1
                      i32.add
                      local.set 7
                      br 6 (;@3;)
                    else
                      local.get 7
                      i32.const 64
                      i32.const 1065748
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  else
                    local.get 13
                    i32.const 3
                    i32.add
                    local.get 2
                    i32.const 1065844
                    call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                    unreachable
                  end
                else
                  local.get 10
                  local.get 2
                  i32.const 1065860
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 11
                local.get 2
                i32.const 1065876
                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                unreachable
              end
            else
              local.get 13
              local.get 2
              i32.const 1065892
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          else
            loop ;; label = @5
              local.get 7
              i32.const 64
              i32.lt_u
              local.tee 6
              if ;; label = @6
                local.get 7
                i32.const 15
                i32.sub
                local.tee 13
                i32.const 64
                i32.lt_u
                if ;; label = @7
                  local.get 3
                  local.get 13
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.load align=1
                  local.set 13
                  local.get 7
                  i32.const 2
                  i32.sub
                  local.tee 12
                  i32.const 64
                  i32.lt_u
                  if ;; label = @8
                    local.get 3
                    local.get 12
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load align=1
                    local.set 12
                    local.get 7
                    i32.const 16
                    i32.sub
                    local.tee 11
                    i32.const 64
                    i32.lt_u
                    if ;; label = @9
                      local.get 3
                      local.get 11
                      i32.const 2
                      i32.shl
                      i32.add
                      i32.load align=1
                      local.set 11
                      local.get 7
                      i32.const 7
                      i32.sub
                      local.tee 10
                      i32.const 64
                      i32.lt_u
                      if ;; label = @10
                        local.get 3
                        local.get 10
                        i32.const 2
                        i32.shl
                        i32.add
                        i32.load align=1
                        local.set 10
                        local.get 6
                        if ;; label = @11
                          local.get 3
                          local.get 7
                          i32.const 2
                          i32.shl
                          i32.add
                          local.get 12
                          i32.const 17
                          i32.rotr
                          local.get 12
                          i32.const 19
                          i32.rotr
                          i32.xor
                          local.get 12
                          i32.const 10
                          i32.shr_u
                          i32.xor
                          local.get 13
                          i32.const 7
                          i32.rotr
                          local.get 13
                          i32.const 18
                          i32.rotr
                          i32.xor
                          local.get 13
                          i32.const 3
                          i32.shr_u
                          i32.xor
                          local.get 11
                          i32.add
                          local.get 10
                          i32.add
                          i32.add
                          i32.store align=1
                          local.get 7
                          i32.const 1
                          i32.add
                          local.set 7
                          br 6 (;@5;)
                        else
                          local.get 7
                          i32.const 64
                          i32.const 1065780
                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                          unreachable
                        end
                      else
                        local.get 10
                        i32.const 64
                        i32.const 1065796
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 11
                      i32.const 64
                      i32.const 1065732
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  else
                    local.get 12
                    i32.const 64
                    i32.const 1065812
                    call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                    unreachable
                  end
                else
                  local.get 13
                  i32.const 64
                  i32.const 1065828
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 0
                i32.load align=1
                local.set 7
                local.get 0
                i32.const 4
                i32.add
                i32.load align=1
                local.set 6
                local.get 0
                i32.const 8
                i32.add
                i32.load align=1
                local.set 5
                local.get 0
                i32.const 12
                i32.add
                i32.load align=1
                local.set 4
                local.get 0
                i32.const 16
                i32.add
                local.set 2
                local.get 17
                i32.const 260
                i32.add
                local.set 1
                i32.const 1065476
                local.set 8
                i32.const 0
                local.get 2
                i32.load align=1
                local.get 7
                local.get 6
                local.get 5
                local.get 4
                local.get 0
                i32.const 20
                i32.add
                i32.load align=1
                local.get 0
                i32.const 24
                i32.add
                i32.load align=1
                local.get 0
                i32.const 28
                i32.add
                i32.load align=1
                local.set 13
                local.set 12
                local.set 11
                local.set 10
                local.set 9
                local.set 4
                local.set 5
                local.set 6
                local.set 7
                loop ;; label = @7
                  local.get 7
                  i32.const 64
                  i32.lt_u
                  local.tee 14
                  if ;; label = @8
                    local.get 1
                    local.get 8
                    i32.const 256
                    call $memcpy
                    drop
                    local.get 14
                    if ;; label = @9
                      local.get 7
                      i32.const 2
                      i32.shl
                      local.set 15
                      local.get 1
                      local.get 15
                      i32.add
                      i32.load align=1
                      local.set 16
                      local.get 14
                      if ;; label = @10
                        local.get 3
                        local.get 15
                        i32.add
                        i32.load align=1
                        local.set 15
                        local.get 6
                        local.get 12
                        i32.and
                        local.get 6
                        local.get 11
                        i32.and
                        local.get 12
                        i32.xor
                        i32.xor
                        local.get 6
                        i32.const 6
                        i32.rotr
                        local.get 6
                        i32.const 11
                        i32.rotr
                        i32.xor
                        local.get 6
                        i32.const 25
                        i32.rotr
                        i32.xor
                        local.get 13
                        i32.add
                        i32.add
                        local.get 16
                        i32.add
                        local.get 15
                        i32.add
                        local.set 16
                        local.get 7
                        i32.const 1
                        i32.add
                        local.get 16
                        local.get 10
                        i32.add
                        local.get 16
                        local.get 5
                        i32.const 2
                        i32.rotr
                        local.get 5
                        i32.const 13
                        i32.rotr
                        i32.xor
                        local.get 5
                        i32.const 22
                        i32.rotr
                        i32.xor
                        local.get 5
                        local.get 4
                        local.get 9
                        i32.xor
                        i32.and
                        local.get 4
                        local.get 9
                        i32.and
                        i32.xor
                        i32.add
                        i32.add
                        local.get 5
                        local.get 4
                        local.get 9
                        local.get 6
                        local.get 11
                        local.get 12
                        local.set 13
                        local.set 12
                        local.set 11
                        local.set 10
                        local.set 9
                        local.set 4
                        local.set 5
                        local.set 6
                        local.set 7
                        br 3 (;@7;)
                      else
                        local.get 7
                        i32.const 64
                        i32.const 1065764
                        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                        unreachable
                      end
                    else
                      local.get 7
                      i32.const 64
                      i32.const 1065352
                      call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                      unreachable
                    end
                  else
                    local.get 0
                    local.get 0
                    i32.load align=1
                    local.get 5
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 4
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 4
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 8
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 9
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 12
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 10
                    i32.add
                    i32.store align=1
                    local.get 2
                    local.get 2
                    i32.load align=1
                    local.get 6
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 20
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 11
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 24
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 12
                    i32.add
                    i32.store align=1
                    local.get 0
                    i32.const 28
                    i32.add
                    local.tee 14
                    local.get 14
                    i32.load align=1
                    local.get 13
                    i32.add
                    i32.store align=1
                    local.get 17
                    i32.const 528
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
        local.get 3
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        i32.const 0
        i32.store align=1
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        br 1 (;@1;)
      end
    end
    unreachable
  )
```

## LLVM -O2 `run` (42,340,124 instructions, 100.0%)

```wat
(func $run (;6;) (type 6) (param i64) (result i64)
    (local i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 272
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            i32.const 0
            i32.load8_u offset=1065940
            i32.eqz
            br_if 0 (;@4;)
            i32.const 0
            i32.load offset=1065932
            local.set 2
            br 1 (;@3;)
          end
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
            br_if 2 (;@2;)
          end
          i32.const 0
          local.get 2
          i32.store offset=1065936
          i32.const 0
          local.get 2
          i32.store offset=1065932
          i32.const 0
          i32.const 1
          i32.store8 offset=1065940
        end
        local.get 2
        i32.const -65609
        i32.gt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 2
          i32.const 65608
          i32.add
          local.tee 3
          i32.const 0
          i32.load offset=1065936
          local.tee 4
          i32.le_u
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.sub
          local.tee 4
          i32.const 16
          i32.shr_u
          local.get 4
          i32.const 65535
          i32.and
          i32.const 0
          i32.ne
          i32.add
          local.tee 4
          memory.grow
          local.tee 5
          i32.const -1
          i32.eq
          br_if 1 (;@2;)
          i32.const 0
          local.get 5
          local.get 4
          i32.add
          i32.const 16
          i32.shl
          i32.store offset=1065936
        end
        i32.const 0
        local.get 3
        i32.store offset=1065932
        local.get 2
        br_if 1 (;@1;)
      end
      i32.const 1
      i32.const 65608
      call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
      unreachable
    end
    i32.const 0
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=12
    local.get 1
    local.get 2
    i32.store offset=8
    local.get 1
    i32.const 65608
    i32.store offset=4
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
    i64.const 4354685564936845354
    i64.add
    local.set 6
    i64.const -8
    local.set 7
    i64.const 0
    local.set 8
    loop ;; label = @1
      local.get 8
      local.get 6
      i64.const 30
      i64.shr_u
      local.get 6
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
      i64.xor
      local.get 0
      i64.xor
      local.tee 0
      i32.wrap_i64
      local.set 2
      local.get 0
      i64.const 8
      i64.shr_u
      local.set 9
      block ;; label = @2
        local.get 3
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i64.const 16
      i64.shr_u
      local.set 10
      local.get 9
      i32.wrap_i64
      local.set 4
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      local.get 2
      i32.store8
      local.get 1
      local.get 3
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i64.const 24
      i64.shr_u
      local.set 9
      local.get 10
      i32.wrap_i64
      local.set 5
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 1
      i32.add
      local.get 4
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i64.const 32
      i64.shr_u
      local.set 10
      local.get 9
      i32.wrap_i64
      local.set 4
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 2
      i32.add
      local.get 5
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i64.const 40
      i64.shr_u
      local.set 9
      local.get 10
      i32.wrap_i64
      local.set 5
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 3
      i32.add
      local.get 4
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i64.const 48
      i64.shr_u
      local.set 10
      local.get 9
      i32.wrap_i64
      local.set 4
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 4
      i32.add
      local.get 5
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i64.const 56
      i64.shr_u
      local.set 0
      local.get 10
      i32.wrap_i64
      local.set 5
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 5
      i32.add
      local.get 4
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 0
      i32.wrap_i64
      local.set 4
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 6
      i32.add
      local.get 5
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 2
      i32.store offset=12
      block ;; label = @2
        local.get 2
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 7
      i32.add
      local.get 4
      i32.store8
      local.get 1
      local.get 2
      i32.const 1
      i32.add
      local.tee 3
      i32.store offset=12
      local.get 6
      i64.const -7046029254386353131
      i64.add
      local.set 6
      local.get 8
      i64.const -1028001813962170200
      i64.add
      local.set 8
      local.get 7
      i64.const 8
      i64.add
      local.tee 7
      i64.const 65528
      i64.lt_u
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 1
      i32.load offset=4
      i32.const 65536
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
    end
    local.get 1
    i32.load offset=8
    i32.const 128
    i32.store8 offset=65536
    i32.const 65537
    local.set 3
    local.get 1
    i32.const 65537
    i32.store offset=12
    loop ;; label = @1
      block ;; label = @2
        local.get 3
        local.get 1
        i32.load offset=4
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 4
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      end
      local.get 1
      i32.load offset=8
      local.get 3
      i32.add
      i32.const 0
      i32.store8
      local.get 1
      local.get 3
      i32.const 1
      i32.add
      local.tee 3
      i32.store offset=12
      local.get 3
      i32.const 65592
      i32.ne
      br_if 0 (;@1;)
    end
    block ;; label = @1
      local.get 1
      i32.load offset=4
      local.tee 3
      i32.const 65592
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
    end
    local.get 1
    i32.load offset=8
    local.tee 11
    i32.const 0
    i32.store8 offset=65592
    local.get 1
    i32.const 65593
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65593
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 11
    end
    local.get 11
    i32.const 0
    i32.store8 offset=65593
    local.get 1
    i32.const 65594
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65594
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 11
    end
    local.get 11
    i32.const 0
    i32.store8 offset=65594
    local.get 1
    i32.const 65595
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65595
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 11
    end
    local.get 11
    i32.const 0
    i32.store8 offset=65595
    local.get 1
    i32.const 65596
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65596
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 11
    end
    local.get 11
    i32.const 0
    i32.store8 offset=65596
    local.get 1
    i32.const 65597
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65597
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 11
    end
    local.get 11
    i32.const 8
    i32.store8 offset=65597
    local.get 1
    i32.const 65598
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65598
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=4
      local.set 3
      local.get 1
      i32.load offset=8
      local.set 11
    end
    i32.const 0
    local.set 12
    local.get 11
    i32.const 0
    i32.store8 offset=65598
    local.get 1
    i32.const 65599
    i32.store offset=12
    block ;; label = @1
      local.get 3
      i32.const 65599
      i32.ne
      br_if 0 (;@1;)
      local.get 1
      i32.const 4
      i32.add
      call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
      local.get 1
      i32.load offset=8
      local.set 11
    end
    local.get 11
    i32.const 0
    i32.store8 offset=65599
    local.get 1
    i32.const 65600
    i32.store offset=12
    local.get 1
    i32.const 16
    i32.add
    i32.const 64
    i32.add
    local.set 13
    i32.const 1779033703
    local.set 14
    i32.const -1150833019
    local.set 15
    i32.const 1013904242
    local.set 16
    i32.const -1521486534
    local.set 17
    i32.const 1359893119
    local.set 18
    i32.const -1694144372
    local.set 19
    i32.const 528734635
    local.set 20
    i32.const 1541459225
    local.set 21
    loop ;; label = @1
      i32.const 0
      local.set 2
      local.get 13
      i32.const 0
      i32.const 192
      memory.fill
      local.get 1
      local.get 11
      local.get 12
      i32.add
      local.tee 3
      i32.load align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=16
      local.get 1
      local.get 3
      i32.load offset=4 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=20
      local.get 1
      local.get 3
      i32.load offset=8 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=24
      local.get 1
      local.get 3
      i32.load offset=12 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=28
      local.get 1
      local.get 3
      i32.load offset=16 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=32
      local.get 1
      local.get 3
      i32.load offset=20 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=36
      local.get 1
      local.get 3
      i32.load offset=24 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=40
      local.get 1
      local.get 3
      i32.load offset=28 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=44
      local.get 1
      local.get 3
      i32.load offset=32 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=48
      local.get 1
      local.get 3
      i32.load offset=36 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=52
      local.get 1
      local.get 3
      i32.load offset=40 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=56
      local.get 1
      local.get 3
      i32.load offset=44 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=60
      local.get 1
      local.get 3
      i32.load offset=48 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=64
      local.get 1
      local.get 3
      i32.load offset=52 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=68
      local.get 1
      local.get 3
      i32.load offset=56 align=1
      local.tee 4
      local.get 4
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 4
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=72
      local.get 1
      local.get 3
      i32.load offset=60 align=1
      local.tee 3
      local.get 3
      i32.const 16
      i32.rotr
      i32.xor
      i32.const -16711936
      i32.and
      i32.const 8
      i32.shr_u
      local.get 3
      i32.const 8
      i32.rotr
      i32.xor
      i32.store offset=76
      local.get 12
      i32.const 64
      i32.add
      local.set 22
      loop ;; label = @2
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        i32.add
        local.tee 3
        i32.const 64
        i32.add
        local.get 3
        i32.const 4
        i32.add
        i32.load
        local.tee 4
        i32.const 25
        i32.rotl
        local.get 4
        i32.const 14
        i32.rotl
        i32.xor
        local.get 4
        i32.const 3
        i32.shr_u
        i32.xor
        local.get 3
        i32.load
        i32.add
        local.get 3
        i32.const 36
        i32.add
        i32.load
        i32.add
        local.get 3
        i32.const 56
        i32.add
        i32.load
        local.tee 3
        i32.const 15
        i32.rotl
        local.get 3
        i32.const 13
        i32.rotl
        i32.xor
        local.get 3
        i32.const 10
        i32.shr_u
        i32.xor
        i32.add
        i32.store
        local.get 2
        i32.const 4
        i32.add
        local.tee 2
        i32.const 192
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 4
      local.get 14
      local.set 23
      local.get 18
      local.set 24
      local.get 21
      local.set 25
      local.get 20
      local.set 26
      local.get 15
      local.set 3
      local.get 16
      local.set 5
      local.get 17
      local.set 27
      local.get 19
      local.set 2
      loop ;; label = @2
        local.get 2
        local.set 28
        local.get 27
        local.set 29
        local.get 5
        local.tee 27
        local.get 3
        local.tee 5
        i32.xor
        local.get 23
        local.tee 3
        i32.and
        local.get 27
        local.get 5
        i32.and
        i32.xor
        local.get 3
        i32.const 30
        i32.rotl
        local.get 3
        i32.const 19
        i32.rotl
        i32.xor
        local.get 3
        i32.const 10
        i32.rotl
        i32.xor
        i32.add
        local.get 24
        local.tee 2
        i32.const 26
        i32.rotl
        local.get 2
        i32.const 21
        i32.rotl
        i32.xor
        local.get 2
        i32.const 7
        i32.rotl
        i32.xor
        local.get 25
        i32.add
        local.get 4
        i32.const 1049036
        i32.add
        i32.load
        i32.add
        local.get 28
        local.get 26
        local.tee 30
        i32.xor
        local.get 2
        i32.and
        local.get 30
        i32.xor
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 4
        i32.add
        i32.load
        i32.add
        local.tee 24
        i32.add
        local.set 23
        local.get 24
        local.get 29
        i32.add
        local.set 24
        local.get 30
        local.set 25
        local.get 28
        local.set 26
        local.get 4
        i32.const 4
        i32.add
        local.tee 4
        i32.const 256
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 30
      local.get 21
      i32.add
      local.set 21
      local.get 28
      local.get 20
      i32.add
      local.set 20
      local.get 2
      local.get 19
      i32.add
      local.set 19
      local.get 24
      local.get 18
      i32.add
      local.set 18
      local.get 27
      local.get 17
      i32.add
      local.set 17
      local.get 5
      local.get 16
      i32.add
      local.set 16
      local.get 3
      local.get 15
      i32.add
      local.set 15
      local.get 23
      local.get 14
      i32.add
      local.set 14
      local.get 12
      i32.const 65536
      i32.lt_u
      local.set 3
      local.get 22
      local.set 12
      local.get 3
      br_if 0 (;@1;)
    end
    local.get 1
    i64.const 65536
    i64.store offset=16
    block ;; label = @1
      i32.const 0
      i32.const 16384
      i32.const 0
      i32.load offset=1065928
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
      local.tee 4
      local.get 2
      i32.add
      local.tee 3
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      local.get 3
      i32.const 16385
      i32.ge_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 4
        memory.copy
      end
      local.get 1
      local.get 14
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 15
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 16
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 17
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 18
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 19
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 20
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 21
      i32.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 4
        local.get 2
        i32.const 4
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      local.get 1
      local.get 14
      i64.extend_i32_u
      i64.const 27
      i64.shl
      local.get 15
      i64.extend_i32_u
      i64.const 18
      i64.shl
      local.get 16
      i64.extend_i32_u
      i64.const 9
      i64.shl
      i64.xor
      i64.xor
      local.get 17
      i64.extend_i32_u
      i64.xor
      i64.const 9
      i64.rotl
      local.get 18
      i64.extend_i32_u
      i64.xor
      i64.const 9
      i64.rotl
      local.get 19
      i64.extend_i32_u
      i64.xor
      i64.const 9
      i64.rotl
      local.get 20
      i64.extend_i32_u
      i64.xor
      i64.const 9
      i64.rotl
      local.get 21
      i64.extend_i32_u
      i64.xor
      local.tee 0
      i64.store offset=16
      block ;; label = @2
        i32.const 16384
        local.get 2
        local.get 3
        i32.add
        local.tee 3
        i32.sub
        local.tee 2
        i32.const 8
        local.get 2
        i32.const 8
        i32.lt_u
        select
        local.tee 2
        i32.eqz
        br_if 0 (;@2;)
        local.get 3
        i32.const 1049544
        i32.add
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        memory.copy
      end
      i32.const 0
      local.get 2
      local.get 3
      i32.add
      i32.store offset=1065928
      local.get 1
      i32.const 272
      i32.add
      global.set $__stack_pointer
      local.get 0
      return
    end
    local.get 2
    local.get 3
    i32.const 16384
    i32.const 1049020
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )
```

(func $run (;11;) (type 9) (param i64) (result i64)
    (local i32 i32 i64 i64 i64 i32 i32 i32 i64 i64 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    i32.const 0
    local.set 2
    local.get 1
    i32.const 0
    i32.store offset=20
    local.get 1
    i64.const 34359738368
    i64.store offset=12 align=4
    local.get 0
    i64.const -7046029254386353131
    i64.xor
    i64.const -7046029254386353131
    i64.add
    local.tee 3
    i64.const 30
    i64.shr_u
    local.get 3
    i64.xor
    i64.const -4658895280553007687
    i64.mul
    local.tee 3
    i64.const 27
    i64.shr_u
    local.get 3
    i64.xor
    i64.const -7723592293110705685
    i64.mul
    local.tee 3
    i64.const 31
    i64.shr_u
    local.get 3
    i64.xor
    i64.const 4354685564936845354
    i64.add
    local.set 3
    i64.const 0
    local.set 4
    i64.const 0
    local.set 5
    i32.const 1
    local.set 6
    i32.const 8
    local.set 7
    i32.const 0
    local.set 8
    i64.const 0
    local.set 9
    loop ;; label = @1
      local.get 5
      local.get 3
      i64.const 30
      i64.shr_u
      local.get 3
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
      i64.xor
      local.get 10
      i64.xor
      local.set 10
      block ;; label = @2
        local.get 6
        i32.const -1
        i32.add
        local.get 8
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 12
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecyE8grow_oneCs1KmEyLwV42l_5smoke
        local.get 1
        i32.load offset=16
        local.set 7
        local.get 1
        i32.load offset=12
        local.set 8
      end
      local.get 7
      local.get 2
      i32.add
      local.get 10
      i64.store
      local.get 3
      i64.const -7046029254386353131
      i64.add
      local.set 3
      local.get 5
      local.get 0
      i64.add
      local.set 5
      local.get 2
      i32.const 8
      i32.add
      local.set 2
      local.get 10
      local.get 4
      i64.rotl
      local.get 9
      i64.add
      local.set 9
      local.get 1
      local.get 6
      i32.store offset=20
      local.get 6
      i32.const 1
      i32.add
      local.set 6
      local.get 4
      i64.const 1
      i64.add
      local.tee 10
      local.set 4
      local.get 10
      i64.const 64
      i64.ne
      br_if 0 (;@1;)
    end
    local.get 1
    i32.load offset=16
    local.tee 11
    call $_RINvNtNtNtCs8euEcNjxCEf_4core5slice4sort8unstable7ipnsortyNvYyNtNtB8_3cmp10PartialOrd2ltECs1KmEyLwV42l_5smoke
    i32.const 0
    i32.load offset=1065860
    local.set 6
    i32.const 0
    local.set 7
    block ;; label = @1
      loop ;; label = @2
        local.get 1
        local.get 11
        local.get 7
        i32.add
        i64.load
        i64.store offset=24
        i32.const 0
        i32.const 16384
        local.get 6
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
        local.tee 8
        local.get 6
        i32.add
        local.tee 2
        local.get 6
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        i32.const 16385
        i32.ge_u
        br_if 1 (;@1;)
        block ;; label = @3
          local.get 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1049476
          i32.add
          local.get 1
          i32.const 24
          i32.add
          local.get 8
          memory.copy
        end
        i32.const 0
        local.get 2
        i32.store offset=1065860
        local.get 2
        local.set 6
        local.get 7
        i32.const 8
        i32.add
        local.tee 7
        i32.const 512
        i32.ne
        br_if 0 (;@2;)
      end
      i64.const 0
      local.set 3
      i32.const 0
      local.set 7
      loop ;; label = @2
        local.get 11
        local.get 7
        i32.add
        local.tee 6
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
        local.get 3
        i64.add
        i64.add
        i64.add
        i64.add
        local.set 3
        local.get 7
        i32.const 32
        i32.add
        local.tee 7
        i32.const 512
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 1
      local.get 9
      i64.store offset=24
      block ;; label = @2
        i32.const 16384
        local.get 2
        i32.sub
        local.tee 6
        i32.const 8
        local.get 6
        i32.const 8
        i32.lt_u
        select
        local.tee 6
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 1049476
        i32.add
        local.get 1
        i32.const 24
        i32.add
        local.get 6
        memory.copy
      end
      i32.const 0
      local.get 6
      local.get 2
      i32.add
      i32.store offset=1065860
      local.get 1
      i32.const 32
      i32.add
      global.set $__stack_pointer
      local.get 3
      local.get 9
      i64.xor
      return
    end
    local.get 6
    local.get 2
    i32.const 16384
    i32.const 1049116
    call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
    unreachable
  )

(func $run (;36;) (type 11) (param i64) (result i64)
    (local i32 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i64 i64 i64 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i64 i64 i32 i32 i32 i32 i32 i32 i32 i64 i32 i64)
    global.get $__stack_pointer
    i32.const 224
    i32.sub
    local.tee 1
    global.set $__stack_pointer
    local.get 1
    local.get 0
    i64.const -7046029254386353131
    i64.xor
    i64.const -7046029254386353131
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
    local.tee 2
    i64.const -7046029254386353131
    i64.add
    i64.store offset=112
    local.get 1
    i32.const 1
    i32.store offset=176
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i64.const 17179869184
    i64.store offset=8 align=4
    local.get 2
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
    local.tee 3
    local.get 3
    i64.const 3
    i64.div_u
    local.tee 4
    i64.const 3
    i64.mul
    i64.sub
    i64.const 2
    i64.add
    local.set 2
    i32.const 4
    local.set 5
    i32.const 4
    local.set 6
    i32.const 0
    local.set 7
    loop ;; label = @1
      local.get 1
      local.get 1
      i32.const 112
      i32.add
      local.get 1
      i32.const 176
      i32.add
      i32.const 0
      call $_RNvCs4gyxU9834UH_5scene9make_node
      local.get 1
      i32.load offset=4
      local.set 8
      local.get 1
      i32.load
      local.set 9
      block ;; label = @2
        local.get 7
        local.get 1
        i32.load offset=8
        i32.ne
        br_if 0 (;@2;)
        local.get 1
        i32.const 8
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecINtNtB7_5boxed3BoxDNtCs4gyxU9834UH_5scene4NodeEL_EE8grow_oneB17_
        local.get 1
        i32.load offset=12
        local.set 5
      end
      local.get 5
      local.get 6
      i32.add
      local.tee 10
      local.get 8
      i32.store
      local.get 10
      i32.const -4
      i32.add
      local.get 9
      i32.store
      local.get 1
      local.get 7
      i32.const 1
      i32.add
      local.tee 7
      i32.store offset=16
      local.get 6
      i32.const 8
      i32.add
      local.set 6
      local.get 2
      i64.const -1
      i64.add
      local.tee 2
      i64.const 0
      i64.ne
      br_if 0 (;@1;)
    end
    local.get 1
    i32.load offset=12
    local.set 11
    local.get 1
    i32.const 0
    i32.store offset=104
    local.get 1
    i64.const -9223372036854775808
    i64.store offset=32
    local.get 1
    i64.const -9223372036854775808
    i64.store offset=24
    local.get 1
    i64.const 9223372036854775807
    i64.store offset=16
    local.get 1
    i64.const 9223372036854775807
    i64.store offset=8
    i64.const 0
    local.set 2
    local.get 1
    i64.const 0
    i64.store offset=40
    local.get 1
    i64.const 0
    i64.store offset=48
    local.get 1
    i64.const 0
    i64.store offset=56
    local.get 1
    i64.const 0
    i64.store offset=64
    local.get 1
    i64.const 0
    i64.store offset=72
    local.get 1
    i64.const 0
    i64.store offset=80
    local.get 1
    i64.const 0
    i64.store offset=88
    local.get 1
    i64.const -3750763034362895579
    i64.store offset=96
    block ;; label = @1
      local.get 7
      i32.eqz
      br_if 0 (;@1;)
      i32.const 0
      i64.load offset=1049424
      local.set 2
      i32.const 0
      i64.load offset=1049416
      local.set 12
      i32.const 0
      i64.load offset=1049408
      local.set 13
      local.get 11
      local.set 6
      i32.const 0
      local.set 8
      loop ;; label = @2
        local.get 6
        i32.const 4
        i32.add
        i32.load
        i32.load offset=40
        local.set 9
        local.get 6
        i32.load
        local.set 10
        local.get 1
        local.get 2
        i64.store offset=128
        local.get 1
        local.get 12
        i64.store offset=120
        local.get 1
        local.get 13
        i64.store offset=112
        local.get 10
        local.get 1
        i32.const 8
        i32.add
        i32.const 1049432
        local.get 1
        i32.const 112
        i32.add
        i32.const 0
        local.get 9
        call_indirect (type 1)
        local.get 6
        i32.const 8
        i32.add
        local.set 6
        local.get 8
        i32.const 1
        i32.add
        local.tee 8
        local.get 7
        i32.lt_u
        br_if 0 (;@2;)
      end
      local.get 1
      i64.load offset=40
      local.set 2
    end
    local.get 1
    local.get 2
    i64.store offset=112
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.const 16384
          i32.const 0
          i32.load offset=1066220
          local.tee 8
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
          local.tee 9
          local.get 8
          i32.add
          local.tee 6
          local.get 8
          i32.lt_u
          br_if 0 (;@3;)
          local.get 6
          i32.const 16385
          i32.ge_u
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 9
            i32.eqz
            br_if 0 (;@4;)
            local.get 8
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 9
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=48
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=56
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=64
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=72
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load32_u offset=104
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=80
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=88
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=8
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=16
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=24
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=32
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          local.get 1
          local.get 1
          i64.load offset=96
          i64.store offset=112
          block ;; label = @4
            i32.const 16384
            local.get 8
            local.get 6
            i32.add
            local.tee 6
            i32.sub
            local.tee 8
            i32.const 8
            local.get 8
            i32.const 8
            i32.lt_u
            select
            local.tee 8
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 1049836
            i32.add
            local.get 1
            i32.const 112
            i32.add
            local.get 8
            memory.copy
          end
          i32.const 0
          local.get 8
          local.get 6
          i32.add
          i32.store offset=1066220
          i64.const 0
          local.set 14
          local.get 1
          i64.const 0
          i64.store offset=136
          local.get 1
          i64.const 0
          i64.store offset=128
          local.get 1
          i64.const 0
          i64.store offset=120
          local.get 1
          i64.const 0
          i64.store offset=112
          local.get 1
          i64.const 4
          i64.store offset=168
          local.get 1
          i64.const 0
          i64.store offset=160
          local.get 1
          i64.const -7046029254386353131
          i64.store offset=144
          local.get 1
          i64.const 0
          i64.store offset=152
          block ;; label = @4
            local.get 7
            br_if 0 (;@4;)
            i32.const 0
            local.set 5
            br 3 (;@1;)
          end
          i32.const 0
          i64.load offset=1049424
          local.set 2
          i32.const 0
          i64.load offset=1049416
          local.set 12
          i32.const 0
          i64.load offset=1049408
          local.set 13
          local.get 11
          local.set 6
          i32.const 0
          local.set 8
          loop ;; label = @4
            local.get 6
            i32.const 4
            i32.add
            i32.load
            i32.load offset=40
            local.set 9
            local.get 6
            i32.load
            local.set 10
            local.get 1
            local.get 2
            i64.store offset=192
            local.get 1
            local.get 12
            i64.store offset=184
            local.get 1
            local.get 13
            i64.store offset=176
            local.get 10
            local.get 1
            i32.const 112
            i32.add
            i32.const 1049384
            local.get 1
            i32.const 176
            i32.add
            i32.const 0
            local.get 9
            call_indirect (type 1)
            local.get 6
            i32.const 8
            i32.add
            local.set 6
            local.get 8
            i32.const 1
            i32.add
            local.tee 8
            local.get 7
            i32.lt_u
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        local.get 8
        local.get 6
        i32.const 16384
        i32.const 1049176
        call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
        unreachable
      end
      block ;; label = @2
        local.get 1
        i32.load offset=172
        local.tee 5
        br_if 0 (;@2;)
        i32.const 0
        local.set 5
        br 1 (;@1;)
      end
      local.get 1
      i32.load offset=168
      local.set 15
      local.get 5
      i32.const 3
      i32.and
      local.set 8
      block ;; label = @2
        block ;; label = @3
          local.get 5
          i32.const 4
          i32.ge_u
          br_if 0 (;@3;)
          i64.const 0
          local.set 14
          i32.const 0
          local.set 9
          br 1 (;@2;)
        end
        local.get 5
        i32.const -4
        i32.and
        local.set 10
        i64.const 0
        local.set 14
        i32.const 0
        local.set 9
        local.get 15
        local.set 6
        loop ;; label = @3
          local.get 14
          local.get 6
          i64.load32_u
          i64.add
          local.get 6
          i32.const 4
          i32.add
          i64.load32_u
          i64.add
          local.get 6
          i32.const 8
          i32.add
          i64.load32_u
          i64.add
          local.get 6
          i32.const 12
          i32.add
          i64.load32_u
          i64.add
          local.set 14
          local.get 6
          i32.const 16
          i32.add
          local.set 6
          local.get 10
          local.get 9
          i32.const 4
          i32.add
          local.tee 9
          i32.ne
          br_if 0 (;@3;)
        end
        local.get 8
        i32.eqz
        br_if 1 (;@1;)
      end
      local.get 15
      local.get 9
      i32.const 2
      i32.shl
      i32.add
      local.set 6
      loop ;; label = @2
        local.get 14
        local.get 6
        i64.load32_u
        i64.add
        local.set 14
        local.get 6
        i32.const 4
        i32.add
        local.set 6
        local.get 8
        i32.const -1
        i32.add
        local.tee 8
        br_if 0 (;@2;)
      end
    end
    local.get 1
    i32.load offset=140
    local.set 10
    local.get 1
    i32.load offset=136
    local.set 15
    local.get 1
    i32.load offset=132
    local.set 16
    local.get 1
    i32.load offset=128
    local.set 17
    local.get 1
    i32.load offset=124
    local.set 18
    local.get 1
    i32.load offset=120
    local.set 19
    local.get 1
    i32.load offset=116
    local.set 20
    local.get 1
    i32.load offset=112
    local.set 21
    local.get 1
    local.get 5
    i64.extend_i32_u
    i64.store offset=176
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  i32.const 0
                  i32.const 16384
                  i32.const 0
                  i32.load offset=1066220
                  local.tee 8
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
                  local.tee 9
                  local.get 8
                  i32.add
                  local.tee 6
                  local.get 8
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 6
                  i32.const 16385
                  i32.ge_u
                  br_if 0 (;@7;)
                  block ;; label = @8
                    local.get 9
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 1049836
                    i32.add
                    local.get 1
                    i32.const 176
                    i32.add
                    local.get 9
                    memory.copy
                  end
                  local.get 1
                  local.get 14
                  i64.store offset=176
                  block ;; label = @8
                    i32.const 16384
                    local.get 6
                    i32.sub
                    local.tee 8
                    i32.const 8
                    local.get 8
                    i32.const 8
                    i32.lt_u
                    select
                    local.tee 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 1049836
                    i32.add
                    local.get 1
                    i32.const 176
                    i32.add
                    local.get 8
                    memory.copy
                  end
                  local.get 1
                  local.get 1
                  i64.load offset=144
                  i64.store offset=176
                  block ;; label = @8
                    i32.const 16384
                    local.get 8
                    local.get 6
                    i32.add
                    local.tee 6
                    i32.sub
                    local.tee 8
                    i32.const 8
                    local.get 8
                    i32.const 8
                    i32.lt_u
                    select
                    local.tee 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 1049836
                    i32.add
                    local.get 1
                    i32.const 176
                    i32.add
                    local.get 8
                    memory.copy
                  end
                  local.get 1
                  local.get 1
                  i64.load offset=152
                  i64.store offset=176
                  block ;; label = @8
                    i32.const 16384
                    local.get 8
                    local.get 6
                    i32.add
                    local.tee 6
                    i32.sub
                    local.tee 8
                    i32.const 8
                    local.get 8
                    i32.const 8
                    i32.lt_u
                    select
                    local.tee 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 1049836
                    i32.add
                    local.get 1
                    i32.const 176
                    i32.add
                    local.get 8
                    memory.copy
                  end
                  local.get 1
                  local.get 1
                  i64.load32_u offset=160
                  i64.store offset=176
                  block ;; label = @8
                    i32.const 16384
                    local.get 8
                    local.get 6
                    i32.add
                    local.tee 6
                    i32.sub
                    local.tee 8
                    i32.const 8
                    local.get 8
                    i32.const 8
                    i32.lt_u
                    select
                    local.tee 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 1049836
                    i32.add
                    local.get 1
                    i32.const 176
                    i32.add
                    local.get 8
                    memory.copy
                  end
                  local.get 1
                  local.get 20
                  local.get 21
                  i32.add
                  local.get 19
                  i32.add
                  local.get 18
                  i32.add
                  local.get 17
                  i32.add
                  local.get 16
                  i32.add
                  local.get 15
                  i32.add
                  local.get 10
                  i32.add
                  i64.extend_i32_u
                  i64.store offset=176
                  block ;; label = @8
                    i32.const 16384
                    local.get 8
                    local.get 6
                    i32.add
                    local.tee 6
                    i32.sub
                    local.tee 8
                    i32.const 8
                    local.get 8
                    i32.const 8
                    i32.lt_u
                    select
                    local.tee 8
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 6
                    i32.const 1049836
                    i32.add
                    local.get 1
                    i32.const 176
                    i32.add
                    local.get 8
                    memory.copy
                  end
                  i32.const 0
                  local.get 8
                  local.get 6
                  i32.add
                  i32.store offset=1066220
                  local.get 0
                  local.get 0
                  i64.const 13
                  i64.div_u
                  local.tee 22
                  i64.const 13
                  i64.mul
                  i64.sub
                  i32.wrap_i64
                  local.tee 23
                  i32.const 20
                  i32.add
                  local.tee 5
                  i32.const 3
                  i32.shl
                  local.set 8
                  call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
                  i32.const 0
                  i32.load8_u offset=1066232
                  br_if 1 (;@6;)
                  block ;; label = @8
                    memory.size
                    i32.const 16
                    i32.shl
                    local.tee 6
                    br_if 0 (;@8;)
                    i32.const 65536
                    local.set 6
                    i32.const 1
                    memory.grow
                    i32.const -1
                    i32.eq
                    br_if 4 (;@4;)
                  end
                  i32.const 0
                  local.get 6
                  i32.store offset=1066228
                  i32.const 0
                  local.get 6
                  i32.store offset=1066224
                  i32.const 0
                  i32.const 1
                  i32.store8 offset=1066232
                  br 2 (;@5;)
                end
                local.get 8
                local.get 6
                i32.const 16384
                i32.const 1049176
                call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
                unreachable
              end
              i32.const 0
              i32.load offset=1066224
              local.tee 6
              i32.const -8
              i32.gt_u
              br_if 1 (;@4;)
            end
            local.get 6
            i32.const 7
            i32.add
            i32.const -8
            i32.and
            local.tee 15
            local.get 8
            i32.add
            local.tee 6
            local.get 15
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 6
              i32.const 0
              i32.load offset=1066228
              local.tee 9
              i32.le_u
              br_if 0 (;@5;)
              local.get 6
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
              local.tee 10
              i32.const -1
              i32.eq
              br_if 1 (;@4;)
              i32.const 0
              local.get 10
              local.get 9
              i32.add
              i32.const 16
              i32.shl
              i32.store offset=1066228
            end
            i32.const 0
            local.get 6
            i32.store offset=1066224
            local.get 15
            i32.eqz
            br_if 0 (;@4;)
            local.get 1
            local.get 15
            i32.store offset=204
            local.get 1
            local.get 5
            i32.store offset=200
            local.get 1
            i32.const 0
            i32.store offset=208
            call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
            block ;; label = @5
              i32.const 0
              i32.load8_u offset=1066232
              i32.eqz
              br_if 0 (;@5;)
              i32.const 0
              i32.load offset=1066224
              local.set 6
              br 2 (;@3;)
            end
            block ;; label = @5
              memory.size
              i32.const 16
              i32.shl
              local.tee 6
              br_if 0 (;@5;)
              i32.const 65536
              local.set 6
              i32.const 1
              memory.grow
              i32.const -1
              i32.eq
              br_if 3 (;@2;)
            end
            i32.const 0
            local.get 6
            i32.store offset=1066228
            i32.const 0
            local.get 6
            i32.store offset=1066224
            i32.const 0
            i32.const 1
            i32.store8 offset=1066232
            br 1 (;@3;)
          end
          i32.const 8
          local.get 8
          call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
          unreachable
        end
        local.get 6
        local.get 5
        i32.add
        local.tee 8
        local.get 6
        i32.lt_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 8
          i32.const 0
          i32.load offset=1066228
          local.tee 9
          i32.le_u
          br_if 0 (;@3;)
          local.get 8
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
          local.tee 10
          i32.const -1
          i32.eq
          br_if 1 (;@2;)
          i32.const 0
          local.get 10
          local.get 9
          i32.add
          i32.const 16
          i32.shl
          i32.store offset=1066228
        end
        i32.const 0
        local.get 8
        i32.store offset=1066224
        local.get 6
        br_if 1 (;@1;)
      end
      i32.const 1
      local.get 5
      call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
      unreachable
    end
    local.get 1
    local.get 6
    i32.store offset=216
    local.get 1
    local.get 5
    i32.store offset=212
    local.get 5
    i32.const 12
    i32.mul
    local.set 9
    local.get 1
    i32.const 0
    i32.store offset=220
    call $_RNvCs7c4fZWoqHJ1_7___rustc35___rust_no_alloc_shim_is_unstable_v2
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          i32.const 0
          i32.load8_u offset=1066232
          br_if 0 (;@3;)
          block ;; label = @4
            memory.size
            i32.const 16
            i32.shl
            local.tee 6
            br_if 0 (;@4;)
            i32.const 65536
            local.set 6
            i32.const 1
            memory.grow
            i32.const -1
            i32.eq
            br_if 3 (;@1;)
          end
          i32.const 0
          local.get 6
          i32.store offset=1066228
          i32.const 0
          local.get 6
          i32.store offset=1066224
          i32.const 0
          i32.const 1
          i32.store8 offset=1066232
          br 1 (;@2;)
        end
        i32.const 0
        i32.load offset=1066224
        local.tee 6
        i32.const -4
        i32.gt_u
        br_if 1 (;@1;)
      end
      local.get 6
      i32.const 3
      i32.add
      i32.const -4
      i32.and
      local.tee 6
      local.get 9
      i32.add
      local.tee 8
      local.get 6
      i32.lt_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 8
        i32.const 0
        i32.load offset=1066228
        local.tee 10
        i32.le_u
        br_if 0 (;@2;)
        local.get 8
        local.get 10
        i32.sub
        local.tee 10
        i32.const 16
        i32.shr_u
        local.get 10
        i32.const 65535
        i32.and
        i32.const 0
        i32.ne
        i32.add
        local.tee 10
        memory.grow
        local.tee 16
        i32.const -1
        i32.eq
        br_if 1 (;@1;)
        i32.const 0
        local.get 16
        local.get 10
        i32.add
        i32.const 16
        i32.shl
        i32.store offset=1066228
      end
      i32.const 0
      local.get 8
      i32.store offset=1066224
      local.get 6
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i64.const -4550206216604228509
      i64.xor
      i64.const -7046029254386353131
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
      local.set 12
      local.get 1
      i32.const 0
      i32.store offset=184
      local.get 1
      local.get 6
      i32.store offset=180
      local.get 1
      local.get 5
      i32.store offset=176
      i32.const 0
      local.set 8
      local.get 5
      local.set 16
      i32.const 0
      local.set 6
      loop ;; label = @2
        local.get 12
        i64.const 4354685564936845354
        i64.add
        local.tee 2
        i64.const 30
        i64.shr_u
        local.get 2
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
        i64.const 21
        i64.rem_u
        i64.const -10
        i64.add
        local.set 13
        block ;; label = @3
          local.get 6
          local.get 16
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 200
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecxE8grow_oneCs4gyxU9834UH_5scene
          local.get 1
          i32.load offset=204
          local.set 15
          local.get 1
          i32.load offset=200
          local.set 16
        end
        local.get 15
        local.get 8
        i32.add
        local.get 13
        i64.store
        local.get 1
        local.get 6
        i32.const 1
        i32.add
        local.tee 6
        i32.store offset=208
        local.get 12
        i64.const -2691343689449507777
        i64.add
        local.tee 12
        i64.const 30
        i64.shr_u
        local.get 12
        i64.xor
        i64.const -4658895280553007687
        i64.mul
        local.tee 12
        i64.const 27
        i64.shr_u
        local.get 12
        i64.xor
        i64.const -7723592293110705685
        i64.mul
        local.tee 12
        i64.const 31
        i64.shr_u
        local.get 12
        i64.xor
        i32.wrap_i64
        i32.const 3
        i32.and
        local.set 10
        block ;; label = @3
          local.get 1
          i32.load offset=220
          local.tee 9
          local.get 1
          i32.load offset=212
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 212
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_
        end
        local.get 1
        i32.load offset=216
        local.get 9
        i32.add
        local.get 10
        i32.store8
        local.get 1
        local.get 9
        i32.const 1
        i32.add
        i32.store offset=220
        block ;; label = @3
          local.get 1
          i32.load offset=184
          local.tee 24
          local.get 1
          i32.load offset=176
          i32.ne
          br_if 0 (;@3;)
          local.get 1
          i32.const 176
          i32.add
          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecINtNtB7_3vec3VecjEE8grow_oneCs4gyxU9834UH_5scene
        end
        local.get 1
        i32.load offset=180
        local.tee 25
        local.get 24
        i32.const 12
        i32.mul
        i32.add
        local.tee 9
        i32.const 0
        i32.store offset=8
        local.get 9
        i64.const 17179869184
        i64.store align=4
        local.get 1
        local.get 24
        i32.const 1
        i32.add
        local.tee 17
        i32.store offset=184
        local.get 8
        i32.const 8
        i32.add
        local.set 8
        local.get 2
        local.set 12
        local.get 5
        local.get 6
        i32.ne
        br_if 0 (;@2;)
      end
      i32.const 0
      local.set 15
      loop ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 15
                local.tee 8
                i32.const 1
                i32.add
                local.tee 15
                local.get 5
                i32.ge_u
                br_if 0 (;@6;)
                local.get 8
                local.get 17
                i32.ge_u
                br_if 1 (;@5;)
                block ;; label = @7
                  local.get 25
                  local.get 8
                  i32.const 12
                  i32.mul
                  i32.add
                  local.tee 6
                  i32.load offset=8
                  local.tee 9
                  local.get 6
                  i32.load
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 6
                  call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCs4gyxU9834UH_5scene
                end
                local.get 6
                i32.load offset=4
                local.get 9
                i32.const 2
                i32.shl
                i32.add
                local.get 15
                i32.store
                local.get 6
                local.get 9
                i32.const 1
                i32.add
                i32.store offset=8
              end
              local.get 2
              i64.const 4354685564936845354
              i64.add
              local.set 12
              local.get 2
              i64.const -7046029254386353131
              i64.add
              local.set 2
              local.get 12
              local.get 12
              i64.const 30
              i64.shr_u
              i64.xor
              i64.const -4658895280553007687
              i64.mul
              local.tee 12
              i64.const 27
              i64.shr_u
              local.get 12
              i64.xor
              i64.const -7723592293110705685
              i64.mul
              local.tee 12
              i64.const 31
              i64.shr_u
              local.get 12
              i64.xor
              i64.const 3
              i64.rem_u
              local.tee 12
              i64.eqz
              br_if 2 (;@3;)
              local.get 25
              local.get 8
              i32.const 12
              i32.mul
              i32.add
              local.set 6
              loop ;; label = @6
                block ;; label = @7
                  local.get 2
                  i64.const 4354685564936845354
                  i64.add
                  local.tee 13
                  i64.const 30
                  i64.shr_u
                  local.get 13
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
                  i32.wrap_i64
                  local.get 5
                  i32.rem_u
                  local.tee 9
                  local.get 8
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 8
                  local.get 17
                  i32.ge_u
                  br_if 3 (;@4;)
                  block ;; label = @8
                    local.get 6
                    i32.load offset=8
                    local.tee 10
                    local.get 6
                    i32.load
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 6
                    call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCs4gyxU9834UH_5scene
                  end
                  local.get 6
                  i32.load offset=4
                  local.get 10
                  i32.const 2
                  i32.shl
                  i32.add
                  local.get 9
                  i32.store
                  local.get 6
                  local.get 10
                  i32.const 1
                  i32.add
                  i32.store offset=8
                end
                local.get 2
                i64.const -7046029254386353131
                i64.add
                local.set 2
                local.get 12
                i64.const -1
                i64.add
                local.tee 12
                i64.eqz
                br_if 3 (;@3;)
                br 0 (;@6;)
              end
            end
            local.get 8
            local.get 17
            i32.const 1049192
            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
            unreachable
          end
          local.get 8
          local.get 17
          i32.const 1049208
          call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
          unreachable
        end
        local.get 15
        local.get 5
        i32.ne
        br_if 0 (;@2;)
      end
      local.get 1
      i32.load offset=204
      local.set 26
      i64.const 0
      local.set 2
      i64.const 0
      local.set 12
      i64.const 0
      local.set 13
      i64.const 0
      local.set 27
      i64.const 0
      local.set 28
      block ;; label = @2
        local.get 17
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load offset=220
        local.set 29
        local.get 1
        i32.load offset=216
        local.set 30
        local.get 1
        i32.const 0
        i32.store offset=208
        local.get 1
        i64.const 4294967296
        i64.store offset=200 align=4
        local.get 1
        i32.const 200
        i32.add
        local.get 17
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs4gyxU9834UH_5scene
        local.get 1
        i32.load offset=204
        local.tee 19
        local.get 1
        i32.load offset=208
        local.tee 18
        i32.add
        local.set 6
        block ;; label = @3
          local.get 24
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 24
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 0
            local.get 24
            memory.fill
          end
          local.get 19
          local.get 18
          local.get 24
          i32.add
          local.tee 18
          i32.add
          local.set 6
        end
        local.get 6
        i32.const 0
        i32.store8
        local.get 1
        local.get 18
        i32.const 1
        i32.add
        local.tee 31
        i32.store offset=208
        local.get 1
        i32.const 0
        i32.store offset=220
        local.get 1
        i64.const 34359738368
        i64.store offset=212 align=4
        local.get 1
        i32.const 212
        i32.add
        local.get 17
        i32.const 8
        i32.const 8
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs4gyxU9834UH_5scene
        local.get 1
        i32.load offset=216
        local.tee 32
        local.get 1
        i32.load offset=220
        local.tee 8
        i32.const 3
        i32.shl
        i32.add
        local.set 6
        block ;; label = @3
          block ;; label = @4
            local.get 24
            br_if 0 (;@4;)
            local.get 8
            local.set 33
            br 1 (;@3;)
          end
          block ;; label = @4
            local.get 17
            i32.const 3
            i32.shl
            i32.const -8
            i32.add
            local.tee 9
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 0
            local.get 9
            memory.fill
          end
          local.get 8
          local.get 24
          i32.add
          local.set 33
          local.get 32
          local.get 8
          local.get 17
          i32.add
          i32.const 3
          i32.shl
          i32.add
          i32.const -8
          i32.add
          local.set 6
        end
        i64.const 0
        local.set 13
        local.get 6
        i64.const 0
        i64.store
        local.get 1
        local.get 33
        i32.const 1
        i32.add
        local.tee 34
        i32.store offset=220
        local.get 19
        i32.const 1
        i32.store8
        local.get 1
        i64.const 17179869184
        i64.store offset=176 align=4
        local.get 1
        i64.const 0
        i64.store offset=184 align=4
        local.get 1
        i32.const 176
        i32.add
        call $_RNvMs4_NtNtCseFzb18zECBI_5alloc11collections9vec_dequeINtB5_8VecDequejE4growCs4gyxU9834UH_5scene
        local.get 1
        i32.load offset=180
        local.tee 21
        local.get 1
        i32.load offset=184
        local.tee 20
        local.get 1
        i32.load offset=188
        local.tee 6
        i32.add
        local.tee 8
        i32.const 0
        local.get 1
        i32.load offset=176
        local.tee 15
        local.get 8
        local.get 15
        i32.lt_u
        select
        i32.sub
        i32.const 2
        i32.shl
        i32.add
        i32.const 0
        i32.store
        i64.const 0
        local.set 12
        i64.const 0
        local.set 27
        block ;; label = @3
          local.get 6
          i32.const 1
          i32.add
          local.tee 9
          i32.eqz
          br_if 0 (;@3;)
          i64.const 0
          local.set 13
          i64.const 0
          local.set 12
          i64.const 0
          local.set 27
          loop ;; label = @4
            local.get 1
            local.get 9
            i32.const -1
            i32.add
            local.tee 9
            i32.store offset=188
            local.get 1
            local.get 20
            i32.const 1
            i32.add
            local.tee 6
            i32.const 0
            local.get 15
            local.get 6
            local.get 15
            i32.lt_u
            select
            i32.sub
            local.tee 6
            i32.store offset=184
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      block ;; label = @10
                        block ;; label = @11
                          local.get 21
                          local.get 20
                          i32.const 2
                          i32.shl
                          i32.add
                          i32.load
                          local.tee 35
                          local.get 29
                          i32.ge_u
                          br_if 0 (;@11;)
                          local.get 35
                          local.get 5
                          i32.ge_u
                          br_if 1 (;@10;)
                          local.get 35
                          local.get 33
                          i32.gt_u
                          br_if 2 (;@9;)
                          block ;; label = @12
                            block ;; label = @13
                              local.get 35
                              local.get 17
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 30
                              local.get 35
                              i32.add
                              i64.load8_u
                              local.set 28
                              local.get 26
                              local.get 35
                              i32.const 3
                              i32.shl
                              local.tee 8
                              i32.add
                              i64.load
                              local.set 36
                              local.get 32
                              local.get 8
                              i32.add
                              local.tee 37
                              i64.load
                              local.set 38
                              local.get 25
                              local.get 35
                              i32.const 12
                              i32.mul
                              i32.add
                              local.tee 8
                              i32.load offset=8
                              local.tee 10
                              br_if 1 (;@12;)
                              local.get 6
                              local.set 20
                              br 8 (;@5;)
                            end
                            local.get 35
                            local.get 17
                            i32.const 1049272
                            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                            unreachable
                          end
                          local.get 10
                          i32.const 2
                          i32.shl
                          local.set 10
                          local.get 8
                          i32.load offset=4
                          local.set 8
                          local.get 6
                          local.set 20
                          loop ;; label = @12
                            local.get 8
                            i32.load
                            local.tee 6
                            local.get 18
                            i32.gt_u
                            br_if 4 (;@8;)
                            block ;; label = @13
                              local.get 19
                              local.get 6
                              i32.add
                              local.tee 16
                              i32.load8_u
                              br_if 0 (;@13;)
                              local.get 16
                              i32.const 1
                              i32.store8
                              local.get 6
                              local.get 33
                              i32.gt_u
                              br_if 6 (;@7;)
                              local.get 32
                              local.get 6
                              i32.const 3
                              i32.shl
                              i32.add
                              local.get 37
                              i64.load
                              i64.const 1
                              i64.add
                              i64.store
                              block ;; label = @14
                                local.get 9
                                local.get 15
                                i32.ne
                                br_if 0 (;@14;)
                                local.get 1
                                i32.const 176
                                i32.add
                                call $_RNvMs4_NtNtCseFzb18zECBI_5alloc11collections9vec_dequeINtB5_8VecDequejE4growCs4gyxU9834UH_5scene
                                local.get 1
                                i32.load offset=180
                                local.set 21
                                local.get 1
                                i32.load offset=184
                                local.set 20
                                local.get 1
                                i32.load offset=176
                                local.set 15
                                local.get 1
                                i32.load offset=188
                                local.set 9
                              end
                              local.get 21
                              local.get 9
                              local.get 20
                              i32.add
                              local.tee 16
                              i32.const 0
                              local.get 15
                              local.get 16
                              local.get 15
                              i32.lt_u
                              select
                              i32.sub
                              i32.const 2
                              i32.shl
                              i32.add
                              local.get 6
                              i32.store
                              local.get 1
                              local.get 9
                              i32.const 1
                              i32.add
                              local.tee 9
                              i32.store offset=188
                            end
                            local.get 8
                            i32.const 4
                            i32.add
                            local.set 8
                            local.get 10
                            i32.const -4
                            i32.add
                            local.tee 10
                            i32.eqz
                            br_if 6 (;@6;)
                            br 0 (;@12;)
                          end
                        end
                        local.get 35
                        local.get 29
                        i32.const 1049224
                        call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                        unreachable
                      end
                      local.get 35
                      local.get 5
                      i32.const 1049240
                      call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                      unreachable
                    end
                    local.get 35
                    local.get 34
                    i32.const 1049256
                    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                    unreachable
                  end
                  local.get 6
                  local.get 31
                  i32.const 1049288
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 6
                local.get 34
                i32.const 1049304
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 1
              i32.load offset=188
              local.set 9
            end
            local.get 13
            i64.const 1
            i64.add
            local.set 13
            local.get 12
            local.get 35
            i64.extend_i32_u
            i64.xor
            local.get 28
            i64.const 8
            i64.shl
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 7
            i64.rotl
            local.set 12
            local.get 36
            local.get 27
            i64.add
            local.get 38
            i64.add
            local.set 27
            local.get 9
            br_if 0 (;@4;)
          end
        end
        local.get 1
        i32.const 0
        i32.store offset=220
        local.get 1
        i64.const 4294967296
        i64.store offset=212 align=4
        local.get 1
        i32.const 212
        i32.add
        local.get 17
        i32.const 1
        i32.const 1
        call $_RINvNvMs5_NtCseFzb18zECBI_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs4gyxU9834UH_5scene
        local.get 1
        i32.load offset=216
        local.tee 18
        local.get 1
        i32.load offset=220
        local.tee 16
        i32.add
        local.set 6
        block ;; label = @3
          local.get 24
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 24
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 0
            local.get 24
            memory.fill
          end
          local.get 18
          local.get 16
          local.get 24
          i32.add
          local.tee 16
          i32.add
          local.set 6
        end
        local.get 6
        i32.const 0
        i32.store8
        local.get 1
        local.get 16
        i32.const 1
        i32.add
        local.tee 20
        i32.store offset=220
        local.get 1
        i32.const 0
        i32.store offset=184
        local.get 1
        i64.const 17179869184
        i64.store offset=176 align=4
        local.get 1
        i32.const 176
        i32.add
        call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCs4gyxU9834UH_5scene
        local.get 1
        i32.load offset=180
        local.tee 19
        i32.const 0
        i32.store
        i64.const 0
        local.set 28
        i32.const 1
        local.set 10
        loop ;; label = @3
          local.get 1
          local.get 10
          i32.const -1
          i32.add
          local.tee 10
          i32.store offset=184
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  block ;; label = @8
                    local.get 19
                    local.get 10
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.load
                    local.tee 6
                    local.get 16
                    i32.gt_u
                    br_if 0 (;@8;)
                    local.get 18
                    local.get 6
                    i32.add
                    local.tee 8
                    i32.load8_u
                    br_if 4 (;@4;)
                    local.get 8
                    i32.const 1
                    i32.store8
                    local.get 6
                    local.get 17
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 6
                    i64.extend_i32_u
                    i64.const -7046029254386353131
                    i64.mul
                    local.get 28
                    i64.xor
                    i64.const 1099511628211
                    i64.mul
                    i64.const 7
                    i64.rotl
                    local.set 28
                    local.get 25
                    local.get 6
                    i32.const 12
                    i32.mul
                    i32.add
                    local.tee 15
                    i32.load offset=8
                    local.tee 8
                    i32.eqz
                    br_if 4 (;@4;)
                    local.get 8
                    i32.const -1
                    i32.add
                    local.set 6
                    local.get 8
                    i32.const 2
                    i32.shl
                    i32.const -4
                    i32.add
                    local.set 8
                    loop ;; label = @9
                      local.get 6
                      local.get 15
                      i32.load offset=8
                      local.tee 9
                      i32.ge_u
                      br_if 3 (;@6;)
                      local.get 15
                      i32.load offset=4
                      local.get 8
                      i32.add
                      i32.load
                      local.tee 9
                      local.get 16
                      i32.gt_u
                      br_if 4 (;@5;)
                      block ;; label = @10
                        local.get 18
                        local.get 9
                        i32.add
                        i32.load8_u
                        br_if 0 (;@10;)
                        block ;; label = @11
                          local.get 10
                          local.get 1
                          i32.load offset=176
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 1
                          i32.const 176
                          i32.add
                          call $_RNvMs7_NtCseFzb18zECBI_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCs4gyxU9834UH_5scene
                          local.get 1
                          i32.load offset=180
                          local.set 19
                        end
                        local.get 19
                        local.get 10
                        i32.const 2
                        i32.shl
                        i32.add
                        local.get 9
                        i32.store
                        local.get 1
                        local.get 10
                        i32.const 1
                        i32.add
                        local.tee 10
                        i32.store offset=184
                      end
                      local.get 8
                      i32.const -4
                      i32.add
                      local.set 8
                      local.get 6
                      i32.const -1
                      i32.add
                      local.tee 6
                      i32.const -1
                      i32.eq
                      br_if 5 (;@4;)
                      br 0 (;@9;)
                    end
                  end
                  local.get 6
                  local.get 20
                  i32.const 1049320
                  call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                  unreachable
                end
                local.get 6
                local.get 17
                i32.const 1049336
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 6
              local.get 9
              i32.const 1049352
              call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
              unreachable
            end
            local.get 9
            local.get 20
            i32.const 1049368
            call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
            unreachable
          end
          local.get 10
          br_if 0 (;@3;)
        end
      end
      local.get 23
      i32.const 3
      i32.and
      local.set 8
      i32.const 0
      local.set 9
      block ;; label = @2
        block ;; label = @3
          local.get 0
          i32.wrap_i64
          local.get 22
          i32.wrap_i64
          i32.const -13
          i32.mul
          i32.add
          i32.const 19
          i32.add
          i32.const 3
          i32.lt_u
          br_if 0 (;@3;)
          local.get 5
          i32.const 60
          i32.and
          local.set 10
          i64.const 0
          local.set 2
          i32.const 0
          local.set 9
          local.get 26
          local.set 6
          loop ;; label = @4
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
            i64.add
            i64.add
            i64.add
            i64.add
            local.set 2
            local.get 6
            i32.const 32
            i32.add
            local.set 6
            local.get 10
            local.get 9
            i32.const 4
            i32.add
            local.tee 9
            i32.ne
            br_if 0 (;@4;)
          end
          local.get 8
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 26
        local.get 9
        i32.const 3
        i32.shl
        i32.add
        local.set 6
        loop ;; label = @3
          local.get 6
          i64.load
          local.get 2
          i64.add
          local.set 2
          local.get 6
          i32.const 8
          i32.add
          local.set 6
          local.get 8
          i32.const -1
          i32.add
          local.tee 8
          br_if 0 (;@3;)
        end
      end
      local.get 1
      local.get 12
      i64.store offset=176
      block ;; label = @2
        i32.const 0
        i32.const 16384
        i32.const 0
        i32.load offset=1066220
        local.tee 8
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
        local.tee 9
        local.get 8
        i32.add
        local.tee 6
        local.get 8
        i32.lt_u
        br_if 0 (;@2;)
        local.get 6
        i32.const 16385
        i32.ge_u
        br_if 0 (;@2;)
        block ;; label = @3
          local.get 9
          i32.eqz
          br_if 0 (;@3;)
          local.get 8
          i32.const 1049836
          i32.add
          local.get 1
          i32.const 176
          i32.add
          local.get 9
          memory.copy
        end
        local.get 1
        local.get 13
        i64.store offset=176
        block ;; label = @3
          i32.const 16384
          local.get 6
          i32.sub
          local.tee 8
          i32.const 8
          local.get 8
          i32.const 8
          i32.lt_u
          select
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1049836
          i32.add
          local.get 1
          i32.const 176
          i32.add
          local.get 8
          memory.copy
        end
        local.get 1
        local.get 27
        i64.store offset=176
        block ;; label = @3
          i32.const 16384
          local.get 8
          local.get 6
          i32.add
          local.tee 6
          i32.sub
          local.tee 8
          i32.const 8
          local.get 8
          i32.const 8
          i32.lt_u
          select
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1049836
          i32.add
          local.get 1
          i32.const 176
          i32.add
          local.get 8
          memory.copy
        end
        local.get 1
        local.get 28
        i64.store offset=176
        block ;; label = @3
          i32.const 16384
          local.get 8
          local.get 6
          i32.add
          local.tee 6
          i32.sub
          local.tee 8
          i32.const 8
          local.get 8
          i32.const 8
          i32.lt_u
          select
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1049836
          i32.add
          local.get 1
          i32.const 176
          i32.add
          local.get 8
          memory.copy
        end
        local.get 1
        local.get 2
        i64.store offset=176
        block ;; label = @3
          i32.const 16384
          local.get 8
          local.get 6
          i32.add
          local.tee 6
          i32.sub
          local.tee 8
          i32.const 8
          local.get 8
          i32.const 8
          i32.lt_u
          select
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1049836
          i32.add
          local.get 1
          i32.const 176
          i32.add
          local.get 8
          memory.copy
        end
        local.get 1
        local.get 1
        i64.load offset=64
        local.get 1
        i64.load offset=40
        i64.xor
        local.get 12
        local.get 1
        i64.load offset=144
        local.get 0
        i64.const -7046029254386353131
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
        local.get 1
        i64.load offset=96
        i64.xor
        local.get 2
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        i64.xor
        local.get 14
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        i64.xor
        local.get 28
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        local.get 13
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        local.get 27
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        local.tee 2
        i64.store offset=176
        block ;; label = @3
          i32.const 16384
          local.get 8
          local.get 6
          i32.add
          local.tee 6
          i32.sub
          local.tee 8
          i32.const 8
          local.get 8
          i32.const 8
          i32.lt_u
          select
          local.tee 9
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 1049836
          i32.add
          local.get 1
          i32.const 176
          i32.add
          local.get 9
          memory.copy
        end
        i32.const 0
        local.set 8
        i32.const 0
        local.get 9
        local.get 6
        i32.add
        i32.store offset=1066220
        block ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          block ;; label = @4
            local.get 3
            i32.wrap_i64
            local.get 4
            i32.wrap_i64
            i32.const -3
            i32.mul
            i32.add
            i32.const 2
            i32.add
            local.tee 6
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 6
            i32.const 1
            i32.and
            local.set 5
            local.get 6
            i32.const -2
            i32.and
            local.set 10
            i32.const 0
            local.set 8
            local.get 11
            local.set 6
            loop ;; label = @5
              block ;; label = @6
                local.get 6
                i32.const 4
                i32.add
                i32.load
                i32.load
                local.tee 9
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.load
                local.get 9
                call_indirect (type 0)
              end
              block ;; label = @6
                local.get 6
                i32.const 12
                i32.add
                i32.load
                i32.load
                local.tee 9
                i32.eqz
                br_if 0 (;@6;)
                local.get 6
                i32.const 8
                i32.add
                i32.load
                local.get 9
                call_indirect (type 0)
              end
              local.get 6
              i32.const 16
              i32.add
              local.set 6
              local.get 10
              local.get 8
              i32.const 2
              i32.add
              local.tee 8
              i32.ne
              br_if 0 (;@5;)
            end
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
          end
          local.get 11
          local.get 8
          i32.const 3
          i32.shl
          i32.add
          local.tee 6
          i32.load offset=4
          i32.load
          local.tee 8
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.load
          local.get 8
          call_indirect (type 0)
        end
        local.get 1
        i32.const 224
        i32.add
        global.set $__stack_pointer
        local.get 2
        return
      end
      local.get 8
      local.get 6
      i32.const 16384
      i32.const 1049176
      call $_RNvNtNtCs8euEcNjxCEf_4core5slice5index16slice_index_fail
      unreachable
    end
    i32.const 4
    local.get 9
    call $_RNvNtCseFzb18zECBI_5alloc7raw_vec12handle_error
    unreachable
  )

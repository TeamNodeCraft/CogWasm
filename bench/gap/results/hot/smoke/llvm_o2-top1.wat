(func $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort8unstable9quicksort9quicksortyNvYyNtNtBa_3cmp10PartialOrd2ltECs1KmEyLwV42l_5smoke (;1;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i64 i64 i64 i32 i32 i32 i32 i32 i32 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64)
    global.get $__stack_pointer
    i32.const 256
    i32.sub
    local.tee 4
    global.set $__stack_pointer
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            local.get 1
            i32.const 33
            i32.lt_u
            br_if 0 (;@4;)
            block ;; label = @5
              local.get 3
              i32.eqz
              br_if 0 (;@5;)
              loop ;; label = @6
                local.get 0
                local.get 1
                i32.const 3
                i32.shr_u
                local.tee 5
                i32.const 56
                i32.mul
                i32.add
                local.set 6
                local.get 0
                local.get 5
                i32.const 5
                i32.shl
                i32.add
                local.set 7
                block ;; label = @7
                  block ;; label = @8
                    local.get 1
                    i32.const 64
                    i32.lt_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 7
                    local.get 6
                    local.get 5
                    call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort6shared5pivot11median3_recyNvYyNtNtBa_3cmp10PartialOrd2ltECs1KmEyLwV42l_5smoke
                    local.set 5
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 6
                  local.get 7
                  local.get 0
                  i64.load
                  local.tee 8
                  local.get 7
                  i64.load
                  local.tee 9
                  i64.lt_u
                  local.tee 5
                  local.get 9
                  local.get 6
                  i64.load
                  local.tee 10
                  i64.lt_u
                  i32.xor
                  select
                  local.get 5
                  local.get 8
                  local.get 10
                  i64.lt_u
                  i32.xor
                  select
                  local.set 5
                end
                local.get 3
                i32.const -1
                i32.add
                local.set 3
                local.get 5
                local.get 0
                i32.sub
                local.set 5
                block ;; label = @7
                  block ;; label = @8
                    block ;; label = @9
                      local.get 2
                      br_if 0 (;@9;)
                      local.get 0
                      local.get 5
                      i32.add
                      i64.load
                      local.set 8
                      local.get 0
                      i64.load
                      local.set 9
                      br 1 (;@8;)
                    end
                    local.get 0
                    i64.load
                    local.set 9
                    local.get 2
                    i64.load
                    local.get 0
                    local.get 5
                    i32.add
                    local.tee 6
                    i64.load
                    local.tee 8
                    i64.lt_u
                    br_if 0 (;@8;)
                    local.get 0
                    local.get 8
                    i64.store
                    local.get 6
                    local.get 9
                    i64.store
                    local.get 0
                    i32.const 8
                    i32.add
                    local.set 11
                    local.get 0
                    i64.load
                    local.set 8
                    local.get 0
                    i64.load offset=8
                    local.set 10
                    i32.const 0
                    local.set 6
                    block ;; label = @9
                      block ;; label = @10
                        local.get 0
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 0
                        local.get 1
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee 12
                        i32.const -8
                        i32.add
                        local.tee 13
                        i32.lt_u
                        br_if 0 (;@10;)
                        local.get 11
                        local.set 7
                        br 1 (;@9;)
                      end
                      i32.const 0
                      local.set 6
                      loop ;; label = @10
                        local.get 5
                        i32.const -8
                        i32.add
                        local.get 11
                        local.get 6
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee 7
                        i64.load
                        i64.store
                        local.get 7
                        local.get 5
                        i64.load
                        local.tee 9
                        i64.store
                        local.get 5
                        local.get 11
                        local.get 6
                        local.get 8
                        local.get 9
                        i64.ge_u
                        i32.add
                        local.tee 6
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee 7
                        i64.load
                        i64.store
                        local.get 7
                        local.get 5
                        i32.const 8
                        i32.add
                        i64.load
                        local.tee 9
                        i64.store
                        local.get 6
                        local.get 8
                        local.get 9
                        i64.ge_u
                        i32.add
                        local.set 6
                        local.get 5
                        i32.const 16
                        i32.add
                        local.tee 5
                        local.get 13
                        i32.lt_u
                        br_if 0 (;@10;)
                      end
                      local.get 5
                      i32.const -8
                      i32.add
                      local.set 7
                    end
                    block ;; label = @9
                      local.get 12
                      local.get 5
                      i32.eq
                      br_if 0 (;@9;)
                      loop ;; label = @10
                        local.get 7
                        local.get 11
                        local.get 6
                        i32.const 3
                        i32.shl
                        i32.add
                        local.tee 13
                        i64.load
                        i64.store
                        local.get 13
                        local.get 5
                        local.tee 7
                        i64.load
                        local.tee 9
                        i64.store
                        local.get 6
                        local.get 8
                        local.get 9
                        i64.ge_u
                        i32.add
                        local.set 6
                        local.get 7
                        i32.const 8
                        i32.add
                        local.tee 5
                        local.get 12
                        i32.ne
                        br_if 0 (;@10;)
                      end
                      local.get 5
                      i32.const -8
                      i32.add
                      local.set 7
                    end
                    local.get 7
                    local.get 11
                    local.get 6
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 5
                    i64.load
                    i64.store
                    local.get 5
                    local.get 10
                    i64.store
                    local.get 6
                    local.get 8
                    local.get 10
                    i64.ge_u
                    i32.add
                    local.tee 5
                    local.get 1
                    i32.ge_u
                    br_if 6 (;@2;)
                    local.get 0
                    i64.load
                    local.set 8
                    local.get 0
                    local.get 0
                    local.get 5
                    i32.const 3
                    i32.shl
                    i32.add
                    local.tee 6
                    i64.load
                    i64.store
                    local.get 6
                    local.get 8
                    i64.store
                    local.get 1
                    local.get 5
                    i32.const 1
                    i32.add
                    local.tee 5
                    i32.sub
                    local.set 1
                    local.get 0
                    local.get 5
                    i32.const 3
                    i32.shl
                    i32.add
                    local.set 0
                    i32.const 0
                    local.set 2
                    br 1 (;@7;)
                  end
                  local.get 0
                  local.get 8
                  i64.store
                  local.get 0
                  local.get 5
                  i32.add
                  local.get 9
                  i64.store
                  local.get 0
                  i32.const 8
                  i32.add
                  local.set 11
                  local.get 0
                  i64.load
                  local.set 8
                  local.get 0
                  i64.load offset=8
                  local.set 10
                  i32.const 0
                  local.set 6
                  block ;; label = @8
                    block ;; label = @9
                      local.get 0
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 0
                      local.get 1
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 12
                      i32.const -8
                      i32.add
                      local.tee 13
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 11
                      local.set 7
                      br 1 (;@8;)
                    end
                    i32.const 0
                    local.set 6
                    loop ;; label = @9
                      local.get 5
                      i32.const -8
                      i32.add
                      local.get 11
                      local.get 6
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 7
                      i64.load
                      i64.store
                      local.get 7
                      local.get 5
                      i64.load
                      local.tee 9
                      i64.store
                      local.get 5
                      local.get 11
                      local.get 6
                      local.get 9
                      local.get 8
                      i64.lt_u
                      i32.add
                      local.tee 6
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 7
                      i64.load
                      i64.store
                      local.get 7
                      local.get 5
                      i32.const 8
                      i32.add
                      i64.load
                      local.tee 9
                      i64.store
                      local.get 6
                      local.get 9
                      local.get 8
                      i64.lt_u
                      i32.add
                      local.set 6
                      local.get 5
                      i32.const 16
                      i32.add
                      local.tee 5
                      local.get 13
                      i32.lt_u
                      br_if 0 (;@9;)
                    end
                    local.get 5
                    i32.const -8
                    i32.add
                    local.set 7
                  end
                  block ;; label = @8
                    local.get 12
                    local.get 5
                    i32.eq
                    br_if 0 (;@8;)
                    loop ;; label = @9
                      local.get 7
                      local.get 11
                      local.get 6
                      i32.const 3
                      i32.shl
                      i32.add
                      local.tee 13
                      i64.load
                      i64.store
                      local.get 13
                      local.get 5
                      local.tee 7
                      i64.load
                      local.tee 9
                      i64.store
                      local.get 6
                      local.get 9
                      local.get 8
                      i64.lt_u
                      i32.add
                      local.set 6
                      local.get 7
                      i32.const 8
                      i32.add
                      local.tee 5
                      local.get 12
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 5
                    i32.const -8
                    i32.add
                    local.set 7
                  end
                  local.get 7
                  local.get 11
                  local.get 6
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 5
                  i64.load
                  i64.store
                  local.get 5
                  local.get 10
                  i64.store
                  local.get 6
                  local.get 10
                  local.get 8
                  i64.lt_u
                  i32.add
                  local.tee 5
                  local.get 1
                  i32.ge_u
                  br_if 5 (;@2;)
                  local.get 0
                  i64.load
                  local.set 8
                  local.get 0
                  local.get 0
                  local.get 5
                  i32.const 3
                  i32.shl
                  i32.add
                  local.tee 6
                  i64.load
                  i64.store
                  local.get 6
                  local.get 8
                  i64.store
                  local.get 0
                  local.get 5
                  local.get 2
                  local.get 3
                  call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort8unstable9quicksort9quicksortyNvYyNtNtBa_3cmp10PartialOrd2ltECs1KmEyLwV42l_5smoke
                  local.get 1
                  local.get 5
                  i32.const -1
                  i32.xor
                  i32.add
                  local.set 1
                  local.get 6
                  i32.const 8
                  i32.add
                  local.set 0
                  local.get 6
                  local.set 2
                end
                local.get 1
                i32.const 33
                i32.lt_u
                br_if 2 (;@4;)
                local.get 3
                br_if 0 (;@6;)
              end
            end
            local.get 0
            local.get 1
            call $_RINvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort8unstable8heapsort8heapsortyNvYyNtNtBa_3cmp10PartialOrd2ltECs1KmEyLwV42l_5smoke
            br 1 (;@3;)
          end
          local.get 1
          i32.const 2
          i32.lt_u
          br_if 0 (;@3;)
          local.get 1
          local.get 1
          i32.const 1
          i32.shr_u
          local.tee 14
          local.get 1
          i32.const 18
          i32.lt_u
          local.tee 15
          select
          local.set 5
          local.get 1
          local.get 14
          i32.sub
          local.set 16
          local.get 0
          local.get 14
          i32.const 3
          i32.shl
          i32.add
          local.set 2
          local.get 0
          local.set 11
          loop ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 5
                i32.const 12
                i32.gt_u
                br_if 0 (;@6;)
                i32.const 1
                local.set 6
                local.get 5
                i32.const 8
                i32.le_u
                br_if 1 (;@5;)
                local.get 11
                local.get 11
                i64.load offset=64
                local.tee 8
                local.get 11
                i64.load offset=32
                local.tee 9
                local.get 8
                local.get 9
                i64.gt_u
                select
                local.tee 10
                local.get 11
                i64.load offset=24
                local.tee 17
                local.get 11
                i64.load
                local.tee 18
                local.get 17
                local.get 18
                i64.gt_u
                select
                local.tee 19
                local.get 10
                local.get 19
                i64.gt_u
                select
                local.tee 20
                local.get 11
                i64.load offset=56
                local.tee 21
                local.get 11
                i64.load offset=8
                local.tee 22
                local.get 21
                local.get 22
                i64.gt_u
                select
                local.tee 23
                local.get 17
                local.get 18
                local.get 17
                local.get 18
                i64.lt_u
                select
                local.tee 17
                local.get 23
                local.get 17
                i64.gt_u
                select
                local.tee 18
                local.get 20
                local.get 18
                i64.gt_u
                select
                local.tee 24
                local.get 11
                i64.load offset=48
                local.tee 25
                local.get 11
                i64.load offset=40
                local.tee 26
                local.get 11
                i64.load offset=16
                local.tee 27
                local.get 26
                local.get 27
                i64.gt_u
                select
                local.tee 28
                local.get 25
                local.get 28
                i64.gt_u
                select
                local.tee 29
                local.get 10
                local.get 19
                local.get 10
                local.get 19
                i64.lt_u
                select
                local.tee 10
                local.get 21
                local.get 22
                local.get 21
                local.get 22
                i64.lt_u
                select
                local.tee 19
                local.get 10
                local.get 19
                i64.gt_u
                select
                local.tee 21
                local.get 29
                local.get 21
                i64.gt_u
                select
                local.tee 22
                local.get 24
                local.get 22
                i64.gt_u
                select
                i64.store offset=64
                local.get 11
                local.get 25
                local.get 28
                local.get 25
                local.get 28
                i64.lt_u
                select
                local.tee 25
                local.get 8
                local.get 9
                local.get 8
                local.get 9
                i64.lt_u
                select
                local.tee 8
                local.get 26
                local.get 27
                local.get 26
                local.get 27
                i64.lt_u
                select
                local.tee 9
                local.get 8
                local.get 9
                i64.gt_u
                select
                local.tee 26
                local.get 25
                local.get 26
                i64.lt_u
                select
                local.tee 27
                local.get 10
                local.get 19
                local.get 10
                local.get 19
                i64.lt_u
                select
                local.tee 10
                local.get 27
                local.get 10
                i64.lt_u
                select
                local.tee 19
                local.get 8
                local.get 9
                local.get 8
                local.get 9
                i64.lt_u
                select
                local.tee 8
                local.get 23
                local.get 17
                local.get 23
                local.get 17
                i64.lt_u
                select
                local.tee 9
                local.get 8
                local.get 9
                i64.lt_u
                select
                local.tee 17
                local.get 19
                local.get 17
                i64.lt_u
                select
                i64.store
                local.get 11
                local.get 20
                local.get 18
                local.get 20
                local.get 18
                i64.lt_u
                select
                local.tee 18
                local.get 25
                local.get 26
                local.get 25
                local.get 26
                i64.gt_u
                select
                local.tee 20
                local.get 18
                local.get 20
                i64.gt_u
                select
                local.tee 23
                local.get 24
                local.get 22
                local.get 24
                local.get 22
                i64.lt_u
                select
                local.tee 22
                local.get 23
                local.get 22
                i64.gt_u
                select
                i64.store offset=56
                local.get 11
                local.get 23
                local.get 22
                local.get 23
                local.get 22
                i64.lt_u
                select
                local.tee 22
                local.get 18
                local.get 20
                local.get 18
                local.get 20
                i64.lt_u
                select
                local.tee 18
                local.get 29
                local.get 21
                local.get 29
                local.get 21
                i64.lt_u
                select
                local.tee 20
                local.get 18
                local.get 20
                i64.gt_u
                select
                local.tee 21
                local.get 27
                local.get 10
                local.get 27
                local.get 10
                i64.gt_u
                select
                local.tee 10
                local.get 8
                local.get 9
                local.get 8
                local.get 9
                i64.gt_u
                select
                local.tee 8
                local.get 10
                local.get 8
                i64.gt_u
                select
                local.tee 9
                local.get 21
                local.get 9
                i64.gt_u
                select
                local.tee 23
                local.get 22
                local.get 23
                i64.gt_u
                select
                i64.store offset=48
                local.get 11
                local.get 22
                local.get 23
                local.get 22
                local.get 23
                i64.lt_u
                select
                i64.store offset=40
                local.get 11
                local.get 21
                local.get 9
                local.get 21
                local.get 9
                i64.lt_u
                select
                local.tee 9
                local.get 18
                local.get 20
                local.get 18
                local.get 20
                i64.lt_u
                select
                local.tee 18
                local.get 10
                local.get 8
                local.get 10
                local.get 8
                i64.lt_u
                select
                local.tee 8
                local.get 18
                local.get 8
                i64.gt_u
                select
                local.tee 10
                local.get 9
                local.get 10
                i64.gt_u
                select
                i64.store offset=32
                local.get 11
                local.get 9
                local.get 10
                local.get 9
                local.get 10
                i64.lt_u
                select
                i64.store offset=24
                local.get 11
                local.get 18
                local.get 8
                local.get 18
                local.get 8
                i64.lt_u
                select
                local.tee 8
                local.get 19
                local.get 17
                local.get 19
                local.get 17
                i64.gt_u
                select
                local.tee 9
                local.get 8
                local.get 9
                i64.gt_u
                select
                i64.store offset=16
                local.get 11
                local.get 8
                local.get 9
                local.get 8
                local.get 9
                i64.lt_u
                select
                i64.store offset=8
                i32.const 9
                local.set 6
                br 1 (;@5;)
              end
              local.get 11
              local.get 11
              i64.load offset=96
              local.tee 8
              local.get 11
              i64.load
              local.tee 9
              local.get 8
              local.get 9
              i64.gt_u
              select
              local.tee 10
              local.get 11
              i64.load offset=88
              local.tee 17
              local.get 11
              i64.load offset=40
              local.tee 18
              local.get 17
              local.get 18
              i64.gt_u
              select
              local.tee 19
              local.get 11
              i64.load offset=32
              local.tee 20
              local.get 19
              local.get 20
              i64.gt_u
              select
              local.tee 21
              local.get 10
              local.get 21
              i64.gt_u
              select
              local.tee 22
              local.get 11
              i64.load offset=80
              local.tee 23
              local.get 11
              i64.load offset=8
              local.tee 24
              local.get 23
              local.get 24
              i64.gt_u
              select
              local.tee 25
              local.get 11
              i64.load offset=64
              local.tee 26
              local.get 11
              i64.load offset=48
              local.tee 27
              local.get 26
              local.get 27
              i64.gt_u
              select
              local.tee 28
              local.get 25
              local.get 28
              i64.gt_u
              select
              local.tee 29
              local.get 11
              i64.load offset=72
              local.tee 30
              local.get 11
              i64.load offset=16
              local.tee 31
              local.get 30
              local.get 31
              i64.gt_u
              select
              local.tee 32
              local.get 11
              i64.load offset=56
              local.tee 33
              local.get 11
              i64.load offset=24
              local.tee 34
              local.get 33
              local.get 34
              i64.gt_u
              select
              local.tee 35
              local.get 32
              local.get 35
              i64.gt_u
              select
              local.tee 36
              local.get 29
              local.get 36
              i64.gt_u
              select
              local.tee 37
              local.get 22
              local.get 37
              i64.gt_u
              select
              i64.store offset=96
              local.get 11
              local.get 10
              local.get 21
              local.get 10
              local.get 21
              i64.lt_u
              select
              local.tee 10
              local.get 25
              local.get 28
              local.get 25
              local.get 28
              i64.lt_u
              select
              local.tee 21
              local.get 32
              local.get 35
              local.get 32
              local.get 35
              i64.lt_u
              select
              local.tee 25
              local.get 21
              local.get 25
              i64.gt_u
              select
              local.tee 28
              local.get 10
              local.get 28
              i64.gt_u
              select
              local.tee 32
              local.get 26
              local.get 27
              local.get 26
              local.get 27
              i64.lt_u
              select
              local.tee 26
              local.get 23
              local.get 24
              local.get 23
              local.get 24
              i64.lt_u
              select
              local.tee 23
              local.get 26
              local.get 23
              i64.gt_u
              select
              local.tee 24
              local.get 33
              local.get 34
              local.get 33
              local.get 34
              i64.lt_u
              select
              local.tee 27
              local.get 30
              local.get 31
              local.get 30
              local.get 31
              i64.lt_u
              select
              local.tee 30
              local.get 27
              local.get 30
              i64.gt_u
              select
              local.tee 31
              local.get 24
              local.get 31
              i64.gt_u
              select
              local.tee 33
              local.get 19
              local.get 20
              local.get 19
              local.get 20
              i64.lt_u
              select
              local.tee 19
              local.get 8
              local.get 9
              local.get 8
              local.get 9
              i64.lt_u
              select
              local.tee 8
              local.get 19
              local.get 8
              i64.gt_u
              select
              local.tee 9
              local.get 33
              local.get 9
              i64.gt_u
              select
              local.tee 20
              local.get 32
              local.get 20
              i64.gt_u
              select
              local.tee 34
              local.get 22
              local.get 37
              local.get 22
              local.get 37
              i64.lt_u
              select
              local.tee 22
              local.get 29
              local.get 36
              local.get 29
              local.get 36
              i64.lt_u
              select
              local.tee 29
              local.get 17
              local.get 18
              local.get 17
              local.get 18
              i64.lt_u
              select
              local.tee 17
              local.get 29
              local.get 17
              i64.gt_u
              select
              local.tee 18
              local.get 22
              local.get 18
              i64.gt_u
              select
              local.tee 35
              local.get 34
              local.get 35
              i64.gt_u
              select
              i64.store offset=88
              local.get 11
              local.get 27
              local.get 30
              local.get 27
              local.get 30
              i64.lt_u
              select
              local.tee 27
              local.get 26
              local.get 23
              local.get 26
              local.get 23
              i64.lt_u
              select
              local.tee 23
              local.get 27
              local.get 23
              i64.lt_u
              select
              local.tee 26
              local.get 29
              local.get 17
              local.get 29
              local.get 17
              i64.lt_u
              select
              local.tee 17
              local.get 19
              local.get 8
              local.get 19
              local.get 8
              i64.lt_u
              select
              local.tee 8
              local.get 17
              local.get 8
              i64.lt_u
              select
              local.tee 19
              local.get 26
              local.get 19
              i64.lt_u
              select
              i64.store
              local.get 11
              local.get 34
              local.get 35
              local.get 34
              local.get 35
              i64.lt_u
              select
              local.tee 29
              local.get 22
              local.get 18
              local.get 22
              local.get 18
              i64.lt_u
              select
              local.tee 18
              local.get 32
              local.get 20
              local.get 32
              local.get 20
              i64.lt_u
              select
              local.tee 20
              local.get 18
              local.get 20
              i64.gt_u
              select
              local.tee 22
              local.get 29
              local.get 22
              i64.gt_u
              select
              i64.store offset=80
              local.get 11
              local.get 21
              local.get 25
              local.get 21
              local.get 25
              i64.lt_u
              select
              local.tee 21
              local.get 33
              local.get 9
              local.get 33
              local.get 9
              i64.lt_u
              select
              local.tee 9
              local.get 21
              local.get 9
              i64.lt_u
              select
              local.tee 25
              local.get 17
              local.get 8
              local.get 17
              local.get 8
              i64.gt_u
              select
              local.tee 8
              local.get 27
              local.get 23
              local.get 27
              local.get 23
              i64.gt_u
              select
              local.tee 17
              local.get 8
              local.get 17
              i64.lt_u
              select
              local.tee 23
              local.get 25
              local.get 23
              i64.lt_u
              select
              local.tee 27
              local.get 10
              local.get 28
              local.get 10
              local.get 28
              i64.lt_u
              select
              local.tee 10
              local.get 24
              local.get 31
              local.get 24
              local.get 31
              i64.lt_u
              select
              local.tee 24
              local.get 10
              local.get 24
              i64.lt_u
              select
              local.tee 28
              local.get 26
              local.get 19
              local.get 26
              local.get 19
              i64.gt_u
              select
              local.tee 19
              local.get 28
              local.get 19
              i64.lt_u
              select
              local.tee 26
              local.get 27
              local.get 26
              i64.lt_u
              select
              i64.store offset=8
              local.get 11
              local.get 29
              local.get 22
              local.get 29
              local.get 22
              i64.lt_u
              select
              local.tee 22
              local.get 10
              local.get 24
              local.get 10
              local.get 24
              i64.gt_u
              select
              local.tee 10
              local.get 21
              local.get 9
              local.get 21
              local.get 9
              i64.gt_u
              select
              local.tee 9
              local.get 10
              local.get 9
              i64.gt_u
              select
              local.tee 21
              local.get 18
              local.get 20
              local.get 18
              local.get 20
              i64.lt_u
              select
              local.tee 18
              local.get 8
              local.get 17
              local.get 8
              local.get 17
              i64.gt_u
              select
              local.tee 8
              local.get 18
              local.get 8
              i64.gt_u
              select
              local.tee 17
              local.get 21
              local.get 17
              i64.gt_u
              select
              local.tee 20
              local.get 22
              local.get 20
              i64.gt_u
              select
              i64.store offset=72
              local.get 11
              local.get 22
              local.get 20
              local.get 22
              local.get 20
              i64.lt_u
              select
              i64.store offset=64
              local.get 11
              local.get 10
              local.get 9
              local.get 10
              local.get 9
              i64.lt_u
              select
              local.tee 9
              local.get 18
              local.get 8
              local.get 18
              local.get 8
              i64.lt_u
              select
              local.tee 8
              local.get 9
              local.get 8
              i64.gt_u
              select
              local.tee 10
              local.get 21
              local.get 17
              local.get 21
              local.get 17
              i64.lt_u
              select
              local.tee 17
              local.get 10
              local.get 17
              i64.gt_u
              select
              i64.store offset=56
              local.get 11
              local.get 25
              local.get 23
              local.get 25
              local.get 23
              i64.gt_u
              select
              local.tee 18
              local.get 28
              local.get 19
              local.get 28
              local.get 19
              i64.gt_u
              select
              local.tee 19
              local.get 18
              local.get 19
              i64.lt_u
              select
              local.tee 20
              local.get 27
              local.get 26
              local.get 27
              local.get 26
              i64.gt_u
              select
              local.tee 21
              local.get 20
              local.get 21
              i64.lt_u
              select
              i64.store offset=16
              local.get 11
              local.get 10
              local.get 17
              local.get 10
              local.get 17
              i64.lt_u
              select
              local.tee 10
              local.get 9
              local.get 8
              local.get 9
              local.get 8
              i64.lt_u
              select
              local.tee 8
              local.get 18
              local.get 19
              local.get 18
              local.get 19
              i64.gt_u
              select
              local.tee 9
              local.get 8
              local.get 9
              i64.gt_u
              select
              local.tee 17
              local.get 10
              local.get 17
              i64.gt_u
              select
              i64.store offset=48
              local.get 11
              local.get 10
              local.get 17
              local.get 10
              local.get 17
              i64.lt_u
              select
              i64.store offset=40
              local.get 11
              local.get 8
              local.get 9
              local.get 8
              local.get 9
              i64.lt_u
              select
              local.tee 8
              local.get 20
              local.get 21
              local.get 20
              local.get 21
              i64.gt_u
              select
              local.tee 9
              local.get 8
              local.get 9
              i64.gt_u
              select
              i64.store offset=32
              local.get 11
              local.get 8
              local.get 9
              local.get 8
              local.get 9
              i64.lt_u
              select
              i64.store offset=24
              i32.const 13
              local.set 6
            end
            local.get 6
            local.get 5
            i32.gt_u
            br_if 2 (;@2;)
            block ;; label = @5
              local.get 5
              local.get 6
              i32.eq
              br_if 0 (;@5;)
              local.get 11
              local.get 5
              i32.const 3
              i32.shl
              i32.add
              local.set 3
              local.get 11
              local.get 6
              i32.const 3
              i32.shl
              local.tee 12
              i32.add
              local.set 13
              loop ;; label = @6
                block ;; label = @7
                  local.get 13
                  i64.load
                  local.tee 9
                  local.get 13
                  i32.const -8
                  i32.add
                  local.tee 7
                  i64.load
                  local.tee 8
                  i64.ge_u
                  br_if 0 (;@7;)
                  local.get 13
                  local.get 8
                  i64.store
                  local.get 12
                  local.set 5
                  local.get 11
                  local.set 6
                  block ;; label = @8
                    local.get 11
                    local.get 7
                    i32.eq
                    br_if 0 (;@8;)
                    loop ;; label = @9
                      local.get 11
                      local.get 5
                      i32.add
                      local.tee 7
                      i32.const -8
                      i32.add
                      local.set 6
                      local.get 9
                      local.get 7
                      i32.const -16
                      i32.add
                      i64.load
                      local.tee 8
                      i64.ge_u
                      br_if 1 (;@8;)
                      local.get 6
                      local.get 8
                      i64.store
                      local.get 5
                      i32.const -8
                      i32.add
                      local.tee 5
                      i32.const 8
                      i32.ne
                      br_if 0 (;@9;)
                    end
                    local.get 11
                    local.set 6
                  end
                  local.get 6
                  local.get 9
                  i64.store
                end
                local.get 12
                i32.const 8
                i32.add
                local.set 12
                local.get 3
                local.get 13
                i32.const 8
                i32.add
                local.tee 13
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 15
            br_if 1 (;@3;)
            local.get 11
            local.get 0
            i32.eq
            local.set 6
            local.get 16
            local.set 5
            local.get 2
            local.set 11
            local.get 6
            br_if 0 (;@4;)
          end
          local.get 2
          i32.const -8
          i32.add
          local.set 6
          local.get 0
          local.get 1
          i32.const 3
          i32.shl
          i32.const -8
          i32.add
          local.tee 5
          i32.add
          local.set 7
          local.get 4
          local.get 5
          i32.add
          local.set 13
          local.get 4
          local.set 11
          local.get 0
          local.set 5
          loop ;; label = @4
            local.get 11
            local.get 2
            i64.load
            local.tee 10
            local.get 5
            i64.load
            local.tee 17
            local.get 10
            local.get 17
            i64.lt_u
            local.tee 12
            select
            i64.store
            local.get 13
            local.get 7
            i64.load
            local.tee 8
            local.get 6
            i64.load
            local.tee 9
            local.get 8
            local.get 9
            i64.gt_u
            select
            i64.store
            local.get 13
            i32.const -8
            i32.add
            local.set 13
            local.get 11
            i32.const 8
            i32.add
            local.set 11
            local.get 6
            i32.const -8
            i32.const 0
            local.get 8
            local.get 9
            i64.lt_u
            select
            i32.add
            local.set 6
            local.get 7
            i32.const -8
            i32.const 0
            local.get 8
            local.get 9
            i64.ge_u
            select
            i32.add
            local.set 7
            local.get 5
            local.get 10
            local.get 17
            i64.ge_u
            i32.const 3
            i32.shl
            i32.add
            local.set 5
            local.get 2
            local.get 12
            i32.const 3
            i32.shl
            i32.add
            local.set 2
            local.get 14
            i32.const -1
            i32.add
            local.tee 14
            br_if 0 (;@4;)
          end
          local.get 6
          i32.const 8
          i32.add
          local.set 6
          block ;; label = @4
            local.get 1
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 11
            local.get 5
            local.get 2
            local.get 5
            local.get 6
            i32.lt_u
            local.tee 13
            select
            i64.load
            i64.store
            local.get 2
            local.get 5
            local.get 6
            i32.ge_u
            i32.const 3
            i32.shl
            i32.add
            local.set 2
            local.get 5
            local.get 13
            i32.const 3
            i32.shl
            i32.add
            local.set 5
          end
          local.get 6
          local.get 5
          i32.ne
          br_if 2 (;@1;)
          local.get 7
          i32.const 8
          i32.add
          local.get 2
          i32.ne
          br_if 2 (;@1;)
          local.get 1
          i32.const 3
          i32.shl
          local.tee 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          local.get 5
          memory.copy
        end
        local.get 4
        i32.const 256
        i32.add
        global.set $__stack_pointer
        return
      end
      unreachable
    end
    call $_RNvNtNtNtNtCs8euEcNjxCEf_4core5slice4sort6shared9smallsort22panic_on_ord_violation
    unreachable
  )

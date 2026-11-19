(func $_RNvMsc_NtNtCs408MSACshni_11miniz_oxide7deflate4coreNtB5_9DictOxide10find_match (;64;) (type 16) (param i32 i32 i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 32
    i32.sub
    local.tee 23
    global.set $__stack_pointer
    local.get 4
    local.tee 24
    i32.const 258
    local.tee 25
    local.get 24
    local.get 25
    i32.lt_u
    select
    local.set 4
    local.get 6
    local.tee 24
    i32.const 1
    local.tee 25
    local.get 24
    local.get 25
    i32.gt_u
    select
    local.set 6
    local.get 4
    local.get 6
    i32.le_u
    if ;; label = @1
      local.get 0
      local.get 5
      i32.store
      local.get 0
      local.get 6
      i32.store offset=4
      local.get 23
      i32.const 32
      i32.add
      global.set $__stack_pointer
      return
    else
      block ;; label = @2
        local.get 6
        i32.const 32
        i32.lt_u
        if ;; label = @3
          local.get 1
          i32.load offset=12 align=1
          local.set 7
          br 1 (;@2;)
        else
          local.get 1
          i32.const 4
          i32.add
          i32.load offset=12 align=1
          local.set 7
          br 1 (;@2;)
        end
      end
      local.get 1
      i32.load align=1
      local.set 8
      local.get 23
      i32.const 0
      i32.add
      local.get 8
      i32.store align=1
      local.get 2
      i32.const 32767
      i32.and
      local.get 6
      i32.add
      local.tee 9
      i32.const 1
      i32.sub
      local.tee 10
      i32.const 33026
      i32.lt_u
      if ;; label = @2
        local.get 8
        local.get 10
        i32.add
        i32.load8_u
        i32.const 255
        i32.and
        local.set 10
        local.get 9
        i32.const 33026
        i32.lt_u
        if ;; label = @3
          local.get 8
          local.get 9
          i32.add
          i32.load8_u
          i32.const 255
          i32.and
          local.set 9
          local.get 1
          i32.load align=1
          local.set 8
          local.get 23
          i32.const 4
          i32.add
          local.get 8
          i32.store align=1
          local.get 2
          i32.const 32767
          i32.and
          i32.const 33026
          i32.lt_u
          if ;; label = @4
            local.get 2
            i32.const 32767
            i32.and
            local.set 11
            local.get 8
            local.get 11
            i32.add
            i32.load8_u
            i32.const 255
            i32.and
            local.set 12
            local.get 11
            i32.const 1
            i32.add
            local.tee 11
            i32.const 33026
            i32.lt_u
            if ;; label = @5
              local.get 2
              i32.const 32767
              i32.and
              local.set 13
              local.get 23
              i32.const 16
              i32.add
              local.set 14
              local.get 12
              local.get 8
              local.get 11
              i32.add
              i32.load8_u
              i32.const 255
              i32.and
              i32.const 8
              i32.const 15
              i32.and
              i32.shl
              i32.const 65535
              i32.and
              i32.or
              local.set 12
              local.get 13
              i32.const 2
              i32.add
              local.set 11
              local.get 23
              i32.const 20
              i32.add
              local.set 8
              local.get 10
              local.get 9
              i32.const 8
              i32.const 15
              i32.and
              i32.shl
              i32.const 65535
              i32.and
              i32.or
              local.set 10
              loop ;; label = @6
                loop ;; label = @7
                  block ;; label = @8
                    local.get 7
                    i32.const 1
                    i32.sub
                    local.tee 7
                    if ;; label = @9
                      local.get 23
                      i32.const 8
                      i32.add
                      local.set 9
                      local.get 23
                      i32.const 12
                      i32.add
                      local.set 15
                      i32.const 0
                      local.get 13
                      local.get 10
                      local.set 16
                      local.set 10
                      local.set 13
                      loop ;; label = @10
                        local.get 13
                        i32.const 3
                        i32.lt_s
                        if ;; label = @11
                          block ;; label = @12
                            local.get 13
                            i32.const 1
                            i32.add
                            local.tee 17
                            local.get 13
                            i32.lt_s
                            if ;; label = @13
                              br 1 (;@12;)
                            else
                              local.get 10
                              i32.const 32768
                              i32.lt_u
                              if ;; label = @14
                                block ;; label = @15
                                  local.get 1
                                  i32.load offset=4 align=1
                                  local.set 13
                                  local.get 9
                                  local.get 13
                                  i32.store align=1
                                  local.get 13
                                  local.get 10
                                  local.get 10
                                  i32.add
                                  i32.add
                                  i32.load16_u align=1
                                  i32.const 65535
                                  i32.and
                                  local.tee 13
                                  if ;; label = @16
                                    local.get 13
                                    local.set 13
                                    local.get 2
                                    local.get 13
                                    i32.sub
                                    i32.const 65535
                                    i32.and
                                    local.tee 10
                                    local.get 3
                                    i32.gt_u
                                    if ;; label = @17
                                      br 2 (;@15;)
                                    else
                                      local.get 6
                                      i32.const 1
                                      i32.sub
                                      i32.const 258
                                      i32.ge_u
                                      if ;; label = @18
                                        br 3 (;@15;)
                                      else
                                        local.get 1
                                        i32.load align=1
                                        local.set 18
                                        local.get 15
                                        local.get 18
                                        i32.store align=1
                                        local.get 13
                                        i32.const 32767
                                        i32.and
                                        local.tee 13
                                        local.get 6
                                        i32.add
                                        local.tee 19
                                        i32.const 1
                                        i32.sub
                                        local.tee 20
                                        i32.const 33026
                                        i32.lt_u
                                        if ;; label = @19
                                          local.get 18
                                          local.get 20
                                          i32.add
                                          i32.load8_u
                                          i32.const 255
                                          i32.and
                                          local.set 20
                                          local.get 19
                                          i32.const 33026
                                          i32.lt_u
                                          if ;; label = @20
                                            local.get 20
                                            local.get 18
                                            local.get 19
                                            i32.add
                                            i32.load8_u
                                            i32.const 255
                                            i32.and
                                            i32.const 8
                                            i32.const 15
                                            i32.and
                                            i32.shl
                                            i32.const 65535
                                            i32.and
                                            i32.or
                                            local.get 16
                                            i32.eq
                                            if ;; label = @21
                                              local.get 10
                                              if ;; label = @22
                                                local.get 1
                                                i32.load align=1
                                                local.set 15
                                                local.get 14
                                                local.get 15
                                                i32.store align=1
                                                local.get 13
                                                i32.const 33026
                                                i32.lt_u
                                                if ;; label = @23
                                                  local.get 15
                                                  local.get 13
                                                  i32.add
                                                  i32.load8_u
                                                  i32.const 255
                                                  i32.and
                                                  local.set 9
                                                  local.get 13
                                                  i32.const 1
                                                  i32.add
                                                  local.tee 17
                                                  i32.const 33026
                                                  i32.lt_u
                                                  if ;; label = @24
                                                    block ;; label = @25
                                                      local.get 12
                                                      local.get 9
                                                      local.get 15
                                                      local.get 17
                                                      i32.add
                                                      i32.load8_u
                                                      i32.const 255
                                                      i32.and
                                                      i32.const 8
                                                      i32.const 15
                                                      i32.and
                                                      i32.shl
                                                      i32.const 65535
                                                      i32.and
                                                      i32.or
                                                      i32.ne
                                                      if ;; label = @26
                                                        br 1 (;@25;)
                                                      else
                                                        local.get 1
                                                        i32.load8_u offset=36
                                                        i32.const 255
                                                        i32.and
                                                        local.set 17
                                                        i32.const 0
                                                        local.get 11
                                                        local.get 13
                                                        i32.const 2
                                                        i32.add
                                                        local.set 20
                                                        local.set 9
                                                        local.set 15
                                                        loop ;; label = @27
                                                          block ;; label = @28
                                                            local.get 15
                                                            local.get 17
                                                            i32.lt_s
                                                            if ;; label = @29
                                                              local.get 15
                                                              i32.const 1
                                                              i32.add
                                                              local.tee 19
                                                              local.get 15
                                                              i32.lt_s
                                                              if ;; label = @30
                                                                br 18 (;@12;)
                                                              else
                                                                local.get 1
                                                                local.get 9
                                                                call $_RNvMsc_NtNtCs408MSACshni_11miniz_oxide7deflate4coreNtB5_9DictOxide18read_unaligned_u64
                                                                local.set 21
                                                                local.get 1
                                                                local.get 20
                                                                call $_RNvMsc_NtNtCs408MSACshni_11miniz_oxide7deflate4coreNtB5_9DictOxide18read_unaligned_u64
                                                                local.set 22
                                                                local.get 21
                                                                local.get 22
                                                                i64.xor
                                                                local.tee 22
                                                                i64.eqz
                                                                if ;; label = @31
                                                                  local.get 19
                                                                  local.get 9
                                                                  i32.const 8
                                                                  i32.add
                                                                  local.get 20
                                                                  i32.const 8
                                                                  i32.add
                                                                  local.set 20
                                                                  local.set 9
                                                                  local.set 15
                                                                  br 4 (;@27;)
                                                                else
                                                                  block ;; label = @32
                                                                    local.get 9
                                                                    local.get 2
                                                                    i32.const 32767
                                                                    i32.and
                                                                    i32.sub
                                                                    local.get 22
                                                                    i64.ctz
                                                                    i32.wrap_i64
                                                                    i32.const 3
                                                                    i32.shr_u
                                                                    i32.add
                                                                    local.tee 9
                                                                    local.get 6
                                                                    i32.gt_u
                                                                    if ;; label = @33
                                                                      local.get 4
                                                                      local.tee 24
                                                                      local.get 9
                                                                      local.tee 25
                                                                      local.get 24
                                                                      local.get 25
                                                                      i32.lt_u
                                                                      select
                                                                      local.tee 6
                                                                      local.get 4
                                                                      i32.ge_u
                                                                      if ;; label = @34
                                                                        local.get 6
                                                                        local.set 4
                                                                        br 6 (;@28;)
                                                                      else
                                                                        local.get 1
                                                                        i32.load align=1
                                                                        local.set 9
                                                                        local.get 8
                                                                        local.get 9
                                                                        i32.store align=1
                                                                        local.get 2
                                                                        i32.const 32767
                                                                        i32.and
                                                                        local.get 6
                                                                        i32.add
                                                                        local.set 20
                                                                        i32.const 0
                                                                        local.get 20
                                                                        i32.const 1
                                                                        i32.sub
                                                                        local.get 20
                                                                        i32.const 0
                                                                        i32.eq
                                                                        select
                                                                        local.tee 20
                                                                        i32.const 33026
                                                                        i32.lt_u
                                                                        if ;; label = @35
                                                                          local.get 9
                                                                          local.get 20
                                                                          i32.add
                                                                          i32.load8_u
                                                                          i32.const 255
                                                                          i32.and
                                                                          local.set 17
                                                                          local.get 20
                                                                          i32.const 1
                                                                          i32.add
                                                                          local.tee 20
                                                                          i32.const 33026
                                                                          i32.lt_u
                                                                          if ;; label = @36
                                                                            local.get 10
                                                                            local.get 17
                                                                            local.get 9
                                                                            local.get 20
                                                                            i32.add
                                                                            i32.load8_u
                                                                            i32.const 255
                                                                            i32.and
                                                                            i32.const 8
                                                                            i32.const 15
                                                                            i32.and
                                                                            i32.shl
                                                                            i32.const 65535
                                                                            i32.and
                                                                            i32.or
                                                                            local.set 16
                                                                            local.set 5
                                                                            br 4 (;@32;)
                                                                          else
                                                                            local.get 20
                                                                            i32.const 33026
                                                                            i32.const 1068616
                                                                            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                                                            unreachable
                                                                          end
                                                                        else
                                                                          local.get 20
                                                                          i32.const 33026
                                                                          i32.const 1068748
                                                                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                                                          unreachable
                                                                        end
                                                                      end
                                                                    else
                                                                      br 1 (;@32;)
                                                                    end
                                                                  end
                                                                  br 6 (;@25;)
                                                                end
                                                              end
                                                            else
                                                              br 1 (;@28;)
                                                            end
                                                          end
                                                          local.get 10
                                                          local.get 4
                                                          local.set 6
                                                          local.set 5
                                                          br 19 (;@8;)
                                                        end
                                                      end
                                                    end
                                                    local.get 16
                                                    local.set 10
                                                    br 18 (;@6;)
                                                  else
                                                    local.get 17
                                                    i32.const 33026
                                                    i32.const 1068764
                                                    call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                                    unreachable
                                                  end
                                                else
                                                  local.get 13
                                                  i32.const 33026
                                                  i32.const 1068860
                                                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                                  unreachable
                                                end
                                              else
                                                br 14 (;@8;)
                                              end
                                            else
                                              local.get 17
                                              local.get 13
                                              local.set 10
                                              local.set 13
                                              br 11 (;@10;)
                                            end
                                          else
                                            local.get 19
                                            i32.const 33026
                                            i32.const 1068780
                                            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                            unreachable
                                          end
                                        else
                                          local.get 20
                                          i32.const 33026
                                          i32.const 1068876
                                          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                          unreachable
                                        end
                                      end
                                    end
                                  else
                                    br 1 (;@15;)
                                  end
                                end
                                br 6 (;@8;)
                              else
                                local.get 10
                                i32.const 32768
                                i32.const 1068796
                                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                                unreachable
                              end
                            end
                          end
                          unreachable
                        else
                          local.get 10
                          local.get 16
                          local.set 10
                          local.set 13
                          br 4 (;@7;)
                        end
                      end
                    else
                      br 1 (;@8;)
                    end
                  end
                  local.get 0
                  local.get 5
                  i32.store
                  local.get 0
                  local.get 6
                  i32.store offset=4
                  local.get 23
                  i32.const 32
                  i32.add
                  global.set $__stack_pointer
                  return
                end
              end
            else
              local.get 11
              i32.const 33026
              i32.const 1068812
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          else
            local.get 2
            i32.const 32767
            i32.and
            i32.const 33026
            i32.const 1068828
            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
            unreachable
          end
        else
          local.get 9
          i32.const 33026
          i32.const 1068844
          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
          unreachable
        end
      else
        local.get 10
        i32.const 33026
        i32.const 1068892
        call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
        unreachable
      end
    end
    unreachable
  )

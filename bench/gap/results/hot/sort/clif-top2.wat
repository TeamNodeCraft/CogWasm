(func $_RNvCsknsHYGPguHZ_4sort9sift_down (;50;) (type 3) (param i32 i32 i32 i32)
    (local i32 i32 i32 i64 i32)
    i32.const 1067144
    local.set 4
    loop ;; label = @1
      local.get 2
      local.get 2
      i32.add
      local.tee 5
      i32.const 1
      i32.add
      local.tee 6
      local.get 3
      i32.ge_u
      if ;; label = @2
        return
      else
        block ;; label = @3
          local.get 5
          i32.const 2
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          if ;; label = @4
            local.get 6
            local.get 1
            i32.lt_u
            if ;; label = @5
              local.get 0
              local.get 6
              i32.const 3
              i32.shl
              i32.add
              i64.load align=1
              local.set 7
              local.get 5
              local.get 1
              i32.lt_u
              if ;; label = @6
                local.get 7
                local.get 0
                local.get 5
                i32.const 3
                i32.shl
                i32.add
                i64.load align=1
                i64.lt_u
                if ;; label = @7
                  local.get 5
                  local.set 6
                  br 4 (;@3;)
                else
                  br 4 (;@3;)
                end
              else
                local.get 5
                local.get 1
                i32.const 1067328
                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                unreachable
              end
            else
              local.get 6
              local.get 1
              i32.const 1067344
              call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
              unreachable
            end
          else
            br 1 (;@3;)
          end
        end
        local.get 2
        local.get 1
        i32.lt_u
        local.tee 5
        if ;; label = @3
          local.get 0
          local.get 2
          i32.const 3
          i32.shl
          i32.add
          i64.load align=1
          local.set 7
          local.get 6
          local.get 1
          i32.lt_u
          local.tee 8
          if ;; label = @4
            local.get 7
            local.get 0
            local.get 6
            i32.const 3
            i32.shl
            i32.add
            i64.load align=1
            i64.ge_u
            if ;; label = @5
              return
            else
              local.get 5
              if ;; label = @6
                local.get 8
                if ;; label = @7
                  local.get 0
                  local.get 1
                  local.get 2
                  local.get 6
                  local.get 4
                  call $_RNvMNtCsk7Mwp4gGzxp_4core5sliceSy14swap_uncheckedCsknsHYGPguHZ_4sort
                  local.get 6
                  local.set 2
                  br 6 (;@1;)
                else
                  local.get 6
                  local.get 1
                  i32.const 1067264
                  call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                  unreachable
                end
              else
                local.get 2
                local.get 1
                i32.const 1067280
                call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
                unreachable
              end
            end
          else
            local.get 6
            local.get 1
            i32.const 1067296
            call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
            unreachable
          end
        else
          local.get 2
          local.get 1
          i32.const 1067312
          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
          unreachable
        end
      end
    end
    unreachable
  )

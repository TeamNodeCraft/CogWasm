(func $_RNvCs3PrGj7GHZpq_4sort9sift_down (;4;) (type 6) (param i32 i32 i32)
    (local i32 i32 i32 i64 i64)
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          local.get 1
          i32.const 1
          i32.shl
          local.tee 3
          i32.const 1
          i32.or
          local.tee 4
          local.get 2
          i32.ge_u
          br_if 0 (;@3;)
          loop ;; label = @4
            block ;; label = @5
              block ;; label = @6
                local.get 3
                i32.const 2
                i32.add
                local.tee 5
                local.get 2
                i32.lt_u
                br_if 0 (;@6;)
                local.get 4
                local.set 4
                br 1 (;@5;)
              end
              block ;; label = @6
                local.get 4
                i32.const 20000
                i32.lt_u
                br_if 0 (;@6;)
                local.get 4
                i32.const 20000
                i32.const 1049284
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              block ;; label = @6
                local.get 3
                i32.const 19998
                i32.lt_u
                br_if 0 (;@6;)
                local.get 5
                i32.const 20000
                i32.const 1049300
                call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
                unreachable
              end
              local.get 5
              local.get 4
              local.get 0
              local.get 4
              i32.const 3
              i32.shl
              i32.add
              i64.load
              local.get 0
              local.get 5
              i32.const 3
              i32.shl
              i32.add
              i64.load
              i64.lt_u
              select
              local.set 4
            end
            local.get 1
            i32.const 20000
            i32.ge_u
            br_if 2 (;@2;)
            local.get 4
            i32.const 20000
            i32.ge_u
            br_if 3 (;@1;)
            local.get 0
            local.get 1
            i32.const 3
            i32.shl
            i32.add
            local.tee 1
            i64.load
            local.tee 6
            local.get 0
            local.get 4
            i32.const 3
            i32.shl
            i32.add
            local.tee 3
            i64.load
            local.tee 7
            i64.ge_u
            br_if 1 (;@3;)
            local.get 1
            local.get 7
            i64.store
            local.get 3
            local.get 6
            i64.store
            local.get 4
            local.set 1
            local.get 4
            i32.const 1
            i32.shl
            local.tee 3
            i32.const 1
            i32.or
            local.tee 4
            local.get 2
            i32.lt_u
            br_if 0 (;@4;)
          end
        end
        return
      end
      local.get 1
      i32.const 20000
      i32.const 1049316
      call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
      unreachable
    end
    local.get 4
    i32.const 20000
    i32.const 1049332
    call $_RNvNtCs8euEcNjxCEf_4core9panicking18panic_bounds_check
    unreachable
  )

(func $_RNvCs2LeA2mZavAK_16generated_stress4body (;10;) (type 3) (param i64) (result i64)
    (local i32 i32 i32 i64 i32 i64 i32)
    global.get $__stack_pointer
    i32.const 16
    i32.sub
    local.tee 7
    global.set $__stack_pointer
    i32.const 1065256
    local.set 1
    local.get 7
    i32.const 0
    i32.add
    local.set 2
    i32.const 0
    i64.const 0
    local.set 4
    local.set 3
    loop ;; label = @1
      local.get 3
      i32.const 1024
      i32.lt_u
      local.tee 5
      if ;; label = @2
        local.get 5
        if ;; label = @3
          local.get 0
          local.get 1
          local.get 3
          i32.const 2
          i32.shl
          i32.add
          i32.load align=1
          call_indirect (type 3)
          local.set 6
          block ;; label = @4
            local.get 3
            i32.const 63
            i32.and
            if ;; label = @5
              br 1 (;@4;)
            else
              local.get 2
              local.get 4
              local.get 6
              i64.add
              i64.store align=1
              local.get 2
              i32.const 8
              call $_RNvCs2LeA2mZavAK_16generated_stress10emit_bytes
              br 1 (;@4;)
            end
          end
          local.get 3
          i32.const 1
          i32.add
          local.get 4
          local.get 6
          i64.add
          local.set 4
          local.set 3
          br 2 (;@1;)
        else
          local.get 3
          i32.const 1024
          i32.const 1065112
          call $_RNvNtCsk7Mwp4gGzxp_4core9panicking18panic_bounds_check
          unreachable
        end
      else
        local.get 7
        i32.const 8
        i32.add
        local.tee 5
        local.get 4
        i64.store align=1
        local.get 5
        i32.const 8
        call $_RNvCs2LeA2mZavAK_16generated_stress10emit_bytes
        local.get 4
        local.get 7
        i32.const 16
        i32.add
        global.set $__stack_pointer
        return
      end
    end
    unreachable
  )

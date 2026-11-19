(func $_RNvXs2_CsizV4RGem7kw_4exprNtB5_4BumpNtNtNtCsk7Mwp4gGzxp_4core5alloc6global11GlobalAlloc5alloc (;66;) (type 19) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 48
    i32.sub
    local.tee 5
    global.set $__stack_pointer
    local.get 5
    i32.const 0
    i32.add
    local.tee 3
    local.get 0
    i32.store align=1
    local.get 5
    i32.const 4
    i32.add
    local.tee 0
    local.get 1
    i32.store align=1
    local.get 5
    i32.const 8
    i32.add
    local.get 2
    i32.store align=1
    local.get 0
    i32.load align=1
    local.set 0
    local.get 5
    i32.const 36
    i32.add
    local.get 0
    i32.store align=1
    block ;; label = @1
      block ;; label = @2
        local.get 3
        i32.load align=1
        local.tee 3
        i32.load8_u offset=8
        i32.const 255
        i32.and
        if ;; label = @3
          br 1 (;@2;)
        else
          memory.size
          local.set 1
          block ;; label = @4
            local.get 1
            i32.const 16
            i32.shl
            local.tee 1
            if ;; label = @5
              br 1 (;@4;)
            else
              i32.const 1
              memory.grow
              local.tee 1
              i32.const -1
              i32.eq
              if ;; label = @6
                i32.const 0
                local.set 3
                br 5 (;@1;)
              else
                i32.const 65536
                local.set 1
                br 2 (;@4;)
              end
            end
          end
          local.get 3
          local.get 1
          i32.store align=1
          local.get 3
          local.get 1
          i32.store offset=4 align=1
          local.get 3
          i32.const 1
          i32.store8 offset=8
          br 1 (;@2;)
        end
      end
      local.get 3
      i32.load align=1
      local.set 1
      local.get 0
      local.tee 6
      i32.const 1
      local.tee 7
      local.get 6
      local.get 7
      i32.gt_u
      select
      local.set 0
      local.get 1
      local.get 0
      i32.const 1
      i32.sub
      i32.add
      local.tee 4
      local.get 1
      i32.lt_u
      if ;; label = @2
        i32.const 0
        local.set 3
        br 1 (;@1;)
      else
        block ;; label = @3
          local.get 5
          i32.const 24
          i32.add
          local.tee 1
          local.get 4
          i32.store align=1
          local.get 5
          i32.const 20
          i32.add
          i32.const 1
          i32.store align=1
          local.get 5
          i32.const 16
          i32.add
          local.tee 4
          local.get 1
          i32.load align=1
          local.get 0
          i32.const -1
          i32.mul
          i32.and
          i32.store align=1
          local.get 5
          i32.const 12
          i32.add
          i32.const 1
          i32.store align=1
          local.get 4
          i32.load align=1
          local.set 4
          local.get 2
          local.tee 6
          i32.const 1
          local.tee 7
          local.get 6
          local.get 7
          i32.gt_u
          select
          local.tee 2
          local.get 4
          i32.add
          local.tee 1
          local.get 2
          i32.lt_u
          if ;; label = @4
            i32.const 0
            local.set 3
            br 1 (;@3;)
          else
            block ;; label = @5
              local.get 5
              i32.const 32
              i32.add
              local.tee 2
              local.get 1
              i32.store align=1
              local.get 5
              i32.const 28
              i32.add
              i32.const 1
              i32.store align=1
              local.get 2
              i32.load align=1
              local.set 2
              local.get 3
              i32.load offset=4 align=1
              local.set 1
              local.get 2
              local.get 1
              i32.gt_u
              if ;; label = @6
                block ;; label = @7
                  local.get 2
                  local.get 1
                  i32.sub
                  local.tee 1
                  i32.const 65535
                  i32.and
                  if ;; label = @8
                    local.get 1
                    i32.const 16
                    i32.shr_u
                    i32.const 1
                    i32.add
                    local.set 0
                    br 1 (;@7;)
                  else
                    local.get 1
                    i32.const 16
                    i32.shr_u
                    local.set 0
                    br 1 (;@7;)
                  end
                end
                local.get 0
                memory.grow
                local.tee 1
                i32.const -1
                i32.eq
                if ;; label = @7
                  i32.const 0
                  local.set 3
                  br 4 (;@3;)
                else
                  local.get 3
                  local.get 1
                  local.get 0
                  i32.add
                  i32.const 16
                  i32.shl
                  i32.store offset=4 align=1
                  br 2 (;@5;)
                end
              else
                br 1 (;@5;)
              end
            end
            local.get 3
            local.get 2
            i32.store align=1
            local.get 4
            local.get 5
            i32.const 48
            i32.add
            global.set $__stack_pointer
            return
          end
        end
        br 1 (;@1;)
      end
    end
    local.get 3
    local.get 5
    i32.const 48
    i32.add
    global.set $__stack_pointer
    return
    unreachable
  )

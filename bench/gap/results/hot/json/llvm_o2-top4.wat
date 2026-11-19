(func $_RNvCskpYeTZolUQS_4json4walk (;21;) (type 9) (param i32 i64 i32)
    (local i32 i32 i64 i32 i32 i32)
    local.get 2
    local.get 2
    i64.load
    i64.const 1
    i64.add
    i64.store
    block ;; label = @1
      local.get 1
      local.get 2
      i64.load offset=88
      i64.le_u
      br_if 0 (;@1;)
      local.get 2
      local.get 1
      i64.store offset=88
    end
    local.get 2
    local.get 2
    i64.load offset=96
    local.get 0
    i32.load
    local.tee 3
    i32.const 255
    i32.and
    i32.const 5
    local.get 3
    i32.const 0
    i32.lt_s
    local.tee 4
    select
    i64.extend_i32_u
    i64.xor
    i64.const 1099511628211
    i64.mul
    i64.const 7
    i64.rotl
    local.get 1
    i64.xor
    i64.const 1099511628211
    i64.mul
    i64.const 7
    i64.rotl
    local.tee 5
    i64.store offset=96
    block ;; label = @1
      block ;; label = @2
        block ;; label = @3
          block ;; label = @4
            block ;; label = @5
              block ;; label = @6
                block ;; label = @7
                  local.get 3
                  i32.const -2147483648
                  i32.xor
                  i32.const 5
                  local.get 4
                  select
                  br_table 5 (;@2;) 0 (;@7;) 4 (;@3;) 1 (;@6;) 2 (;@5;) 3 (;@4;) 5 (;@2;)
                end
                local.get 2
                local.get 2
                i64.load offset=40
                i64.const 1
                i64.add
                i64.store offset=40
                local.get 2
                local.get 5
                local.get 0
                i64.load8_u offset=4
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                i64.store offset=96
                return
              end
              local.get 2
              local.get 2
              i64.load offset=24
              i64.const 1
              i64.add
              i64.store offset=24
              local.get 2
              local.get 2
              i64.load offset=80
              local.get 0
              i32.load offset=12
              local.tee 4
              i64.extend_i32_u
              local.tee 1
              i64.add
              i64.store offset=80
              local.get 2
              local.get 5
              local.get 1
              i64.xor
              i64.const 1099511628211
              i64.mul
              i64.const 7
              i64.rotl
              local.tee 1
              i64.store offset=96
              local.get 4
              i32.eqz
              br_if 4 (;@1;)
              local.get 0
              i32.load offset=8
              local.set 6
              block ;; label = @6
                block ;; label = @7
                  local.get 4
                  i32.const 3
                  i32.and
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 6
                  local.set 0
                  br 1 (;@6;)
                end
                local.get 6
                local.set 0
                loop ;; label = @7
                  local.get 2
                  local.get 1
                  local.get 0
                  i64.load8_u
                  i64.xor
                  i64.const 1099511628211
                  i64.mul
                  i64.const 7
                  i64.rotl
                  local.tee 1
                  i64.store offset=96
                  local.get 0
                  i32.const 1
                  i32.add
                  local.set 0
                  local.get 3
                  i32.const -1
                  i32.add
                  local.tee 3
                  br_if 0 (;@7;)
                end
              end
              local.get 4
              i32.const 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 6
              local.get 4
              i32.add
              local.set 3
              loop ;; label = @6
                local.get 2
                local.get 1
                local.get 0
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 2
                local.get 1
                local.get 0
                i32.const 1
                i32.add
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 2
                local.get 1
                local.get 0
                i32.const 2
                i32.add
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 2
                local.get 1
                local.get 0
                i32.const 3
                i32.add
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 0
                i32.const 4
                i32.add
                local.tee 0
                local.get 3
                i32.ne
                br_if 0 (;@6;)
                br 5 (;@1;)
              end
            end
            local.get 2
            local.get 2
            i64.load offset=16
            i64.const 1
            i64.add
            i64.store offset=16
            local.get 2
            local.get 5
            local.get 0
            i32.load offset=12
            local.tee 3
            i64.extend_i32_u
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 7
            i64.rotl
            i64.store offset=96
            local.get 3
            i32.eqz
            br_if 3 (;@1;)
            local.get 3
            i32.const 4
            i32.shl
            local.set 3
            local.get 0
            i32.load offset=8
            local.set 0
            local.get 1
            i64.const 1
            i64.add
            local.set 1
            loop ;; label = @5
              local.get 0
              local.get 1
              local.get 2
              call $_RNvCskpYeTZolUQS_4json4walk
              local.get 0
              i32.const 16
              i32.add
              local.set 0
              local.get 3
              i32.const -16
              i32.add
              local.tee 3
              br_if 0 (;@5;)
              br 4 (;@1;)
            end
          end
          local.get 2
          local.get 2
          i64.load offset=8
          i64.const 1
          i64.add
          i64.store offset=8
          local.get 2
          local.get 5
          local.get 0
          i32.load offset=8
          local.tee 3
          i64.extend_i32_u
          i64.xor
          i64.const 1099511628211
          i64.mul
          i64.const 7
          i64.rotl
          i64.store offset=96
          local.get 3
          i32.eqz
          br_if 2 (;@1;)
          local.get 0
          i32.load offset=4
          local.tee 4
          local.get 3
          i32.const 5
          i32.shl
          i32.add
          local.set 7
          local.get 1
          i64.const 1
          i64.add
          local.set 5
          loop ;; label = @4
            local.get 2
            local.get 2
            i64.load offset=96
            local.get 4
            i64.load32_u offset=8
            i64.xor
            i64.const 1099511628211
            i64.mul
            i64.const 7
            i64.rotl
            local.tee 1
            i64.store offset=96
            block ;; label = @5
              local.get 4
              i32.load offset=8
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 4
              i32.load offset=4
              local.set 8
              block ;; label = @6
                block ;; label = @7
                  local.get 6
                  i32.const 3
                  i32.and
                  local.tee 3
                  br_if 0 (;@7;)
                  local.get 8
                  local.set 0
                  br 1 (;@6;)
                end
                local.get 8
                local.set 0
                loop ;; label = @7
                  local.get 2
                  local.get 1
                  local.get 0
                  i64.load8_u
                  i64.xor
                  i64.const 1099511628211
                  i64.mul
                  i64.const 7
                  i64.rotl
                  local.tee 1
                  i64.store offset=96
                  local.get 0
                  i32.const 1
                  i32.add
                  local.set 0
                  local.get 3
                  i32.const -1
                  i32.add
                  local.tee 3
                  br_if 0 (;@7;)
                end
              end
              local.get 6
              i32.const 4
              i32.lt_u
              br_if 0 (;@5;)
              local.get 8
              local.get 6
              i32.add
              local.set 3
              loop ;; label = @6
                local.get 2
                local.get 1
                local.get 0
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 2
                local.get 1
                local.get 0
                i32.const 1
                i32.add
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 2
                local.get 1
                local.get 0
                i32.const 2
                i32.add
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 2
                local.get 1
                local.get 0
                i32.const 3
                i32.add
                i64.load8_u
                i64.xor
                i64.const 1099511628211
                i64.mul
                i64.const 7
                i64.rotl
                local.tee 1
                i64.store offset=96
                local.get 0
                i32.const 4
                i32.add
                local.tee 0
                local.get 3
                i32.ne
                br_if 0 (;@6;)
              end
            end
            local.get 4
            i32.const 16
            i32.add
            local.get 5
            local.get 2
            call $_RNvCskpYeTZolUQS_4json4walk
            local.get 4
            i32.const 32
            i32.add
            local.tee 4
            local.get 7
            i32.ne
            br_if 0 (;@4;)
            br 3 (;@1;)
          end
        end
        local.get 2
        local.get 2
        i64.load offset=32
        i64.const 1
        i64.add
        i64.store offset=32
        local.get 2
        local.get 0
        i64.load offset=8
        local.tee 1
        local.get 2
        i64.load offset=56
        i64.add
        i64.store offset=56
        block ;; label = @3
          local.get 1
          local.get 2
          i64.load offset=64
          i64.ge_s
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i64.store offset=64
        end
        block ;; label = @3
          local.get 1
          local.get 2
          i64.load offset=72
          i64.le_s
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i64.store offset=72
        end
        local.get 2
        local.get 1
        local.get 5
        i64.xor
        i64.const 1099511628211
        i64.mul
        i64.const 7
        i64.rotl
        i64.store offset=96
        return
      end
      local.get 2
      local.get 2
      i64.load offset=48
      i64.const 1
      i64.add
      i64.store offset=48
    end
  )

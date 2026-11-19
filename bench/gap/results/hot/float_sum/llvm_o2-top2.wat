(func $_RNvCsacnkOpZMulW_9float_sum8pairwise (;1;) (type 3) (param i32 i32) (result f64)
    (local f64 i32)
    block ;; label = @1
      local.get 1
      i32.const 4
      i32.gt_u
      br_if 0 (;@1;)
      block ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        f64.const 0x0p+0 (;=0;)
        return
      end
      local.get 0
      f64.load
      f64.const 0x0p+0 (;=0;)
      f64.add
      local.set 2
      block ;; label = @2
        local.get 1
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        f64.load offset=8
        f64.add
        local.set 2
        local.get 1
        i32.const 2
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        f64.load offset=16
        f64.add
        local.set 2
        local.get 1
        i32.const 3
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        local.get 0
        f64.load offset=24
        f64.add
        local.set 2
      end
      local.get 2
      return
    end
    local.get 0
    local.get 1
    i32.const 1
    i32.shr_u
    local.tee 3
    call $_RNvCsacnkOpZMulW_9float_sum8pairwise
    local.get 0
    local.get 3
    i32.const 3
    i32.shl
    i32.add
    local.get 1
    local.get 3
    i32.sub
    call $_RNvCsacnkOpZMulW_9float_sum8pairwise
    f64.add
  )

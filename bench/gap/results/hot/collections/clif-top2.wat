(func $_RINvNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6stable14driftsort_mainyNvYyNtNtB8_3cmp10PartialOrd2ltINtNtCs45LlgTF0LZI_5alloc3vec3VecyEECsaeq2Ur5fVa2_11collections (;142;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get $__stack_pointer
    i32.const 8240
    i32.sub
    local.tee 7
    global.set $__stack_pointer
    i32.const 1052788
    local.set 3
    local.get 7
    i32.const 4120
    i32.add
    local.set 4
    i32.const 0
    local.set 5
    loop ;; label = @1
      local.get 5
      i32.const 4096
      i32.eq
      if ;; label = @2
        local.get 7
        local.tee 5
        local.get 4
        i32.const 4096
        call $memcpy
        drop
        local.get 7
        i32.const 8224
        i32.add
        local.tee 4
        local.get 5
        call $_RNvMNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6stableINtB2_14AlignedStorageyKj1000_E19as_uninit_slice_mutCsaeq2Ur5fVa2_11collections
        local.get 4
        i32.load
        local.set 5
        local.get 4
        i32.load offset=4
        local.set 4
        local.get 1
        local.get 1
        i32.const 1
        i32.shr_u
        i32.sub
        local.tee 10
        local.get 1
        local.tee 12
        i32.const 1000000
        local.tee 13
        local.get 12
        local.get 13
        i32.lt_u
        select
        local.tee 11
        local.get 10
        local.get 11
        i32.gt_u
        select
        local.tee 8
        i32.const 48
        local.tee 9
        local.get 8
        local.get 9
        i32.gt_u
        select
        local.set 3
        local.get 4
        local.get 3
        i32.ge_u
        if ;; label = @3
          local.get 0
          local.get 1
          local.get 5
          local.get 4
          local.get 1
          i32.const 64
          i32.le_u
          local.get 2
          call $_RINvNtNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6stable5drift4sortyNvYyNtNtBa_3cmp10PartialOrd2ltECsaeq2Ur5fVa2_11collections
          local.get 7
          i32.const 8240
          i32.add
          global.set $__stack_pointer
          return
        else
          local.get 7
          i32.const 4108
          i32.add
          local.tee 5
          local.get 3
          call $_RNvXs8_NtCs45LlgTF0LZI_5alloc5sliceINtNtB7_3vec3VecyEINtNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6stable8BufGuardyE13with_capacityCsaeq2Ur5fVa2_11collections
          local.get 5
          i32.load
          local.set 3
          local.get 5
          i32.load offset=4
          local.set 4
          local.get 5
          i32.load offset=8
          local.set 5
          local.get 7
          i32.const 4096
          i32.add
          local.tee 6
          local.get 3
          i32.store
          local.get 6
          local.get 4
          i32.store offset=4
          local.get 6
          local.get 5
          i32.store offset=8
          local.get 7
          i32.const 8224
          i32.add
          local.tee 3
          local.get 6
          call $_RNvXs8_NtCs45LlgTF0LZI_5alloc5sliceINtNtB7_3vec3VecyEINtNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6stable8BufGuardyE19as_uninit_slice_mutCsaeq2Ur5fVa2_11collections
          local.get 3
          i32.load
          local.set 5
          local.get 3
          i32.load offset=4
          local.set 3
          local.get 0
          local.get 1
          local.get 5
          local.get 3
          local.get 1
          i32.const 64
          i32.le_u
          local.get 2
          call $_RINvNtNtNtNtCsk7Mwp4gGzxp_4core5slice4sort6stable5drift4sortyNvYyNtNtBa_3cmp10PartialOrd2ltECsaeq2Ur5fVa2_11collections
          local.get 6
          call $_RINvNtCsk7Mwp4gGzxp_4core3ptr9drop_glueINtNtCs45LlgTF0LZI_5alloc3vec3VecyEECsaeq2Ur5fVa2_11collections
          local.get 7
          i32.const 8240
          i32.add
          global.set $__stack_pointer
          return
        end
      else
        local.get 4
        local.get 5
        i32.add
        local.get 3
        i32.load8_u
        i32.store8
        local.get 5
        i32.const 1
        i32.add
        local.set 5
        br 1 (;@1;)
      end
    end
    unreachable
  )

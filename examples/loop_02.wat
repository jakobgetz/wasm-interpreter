(module (func (local i32 i32)
  loop
    i32.const 1
    local.get 0
    i32.add
    local.tee 0
    loop
      i32.const 1
      local.get 1
      i32.add
      local.tee 1
      i32.const 10
      i32.lt_s
      br_if 1
    end
    i32.const 10
    i32.lt_s
    br_if 0
  end
))

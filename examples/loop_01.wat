(module (func
  (local i32)
  (loop $loop
    local.get 0
    i32.const 1
    i32.add
    local.set 0

    local.get 0
    i32.const 10
    i32.lt_s
    br_if $loop
  )
))

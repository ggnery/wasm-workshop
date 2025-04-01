(module
  ;; Define a function to sum 1 and 3
  (func (export "sum") (result i32)
    i32.const 10  ;; Push 1 onto the stack
    i32.const -3  ;; Push 3 onto the stack
    i32.add      ;; Add the two numbers
  )
)

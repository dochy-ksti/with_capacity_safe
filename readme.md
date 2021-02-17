Vec::with_capacity(capacity) panics / aborts when the capacity is too large.

This is a safer alternative which reports Error using try_reserve

```
//let's pretend this is an arbitrary number read from a broken file
let number_from_file : usize = 100_000_000_000_000;

//try to create a 100TB Vec
let result : Result<Vec<u8>, _> = vec_with_capacity_safe(number_from_file);

//An error is reported
assert_eq!(result, Err(WcsError::new(WcsErrorType::AllocError)));
```
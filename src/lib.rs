

use std::collections::{TryReserveError};

#[cfg(test)]
mod testing;


/// Vec::with_capacity(capacity) panics/aborts when the capacity is too large.
/// This is a safer alternative which reports Error using try_reserve_exact.
///
/// ```
/// use with_capacity_safe::{vec_with_capacity_safe };
///
/// //let's pretend this is an arbitrary number read from a broken file
/// let number_from_file : usize = 100_000_000_000_000;
///
/// //try to create a 100TB Vec
/// let result : Result<Vec<u8>, _> = vec_with_capacity_safe(number_from_file);
///
/// assert!(result.is_err());
/// ```
pub fn vec_with_capacity_safe<T>(capacity : usize) -> Result<Vec<T>, TryReserveError>{
    let mut vec = Vec::new();
    vec.try_reserve_exact(capacity)?;
    Ok(vec)
}
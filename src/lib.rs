#![feature(try_reserve)]

use std::fmt::{Debug, Display, Formatter};
use std::collections::TryReserveError;

#[cfg(test)]
mod testing;

/// The error type for this crate. It implements std::error::Error
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WcsError{
    /// the computed capacity exceeding the usize::MAX_VALUE
    CapacityOverflow(TryReserveError),
    ///The memory allocator returned an error
    AllocError(TryReserveError)
}

impl WcsError{
    /// The source error
    pub fn try_reserve_error(&self) -> &TryReserveError{
        match self {
            WcsError::CapacityOverflow(e) => e,
            WcsError::AllocError(e) => e
        }
    }
}

impl Display for WcsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.try_reserve_error(), f)
    }
}

impl std::error::Error for WcsError{

}

/// Vec::with_capacity(capacity) panics/aborts when the capacity is too large.
/// This is a safer alternative which reports Error using try_reserve_exact.
///
/// ```
/// use with_capacity_safe::{vec_with_capacity_safe, WcsError };
///
/// //let's pretend this is an arbitrary number read from a broken file
/// let number_from_file : usize = 100_000_000_000_000;
///
/// //try to create a 100TB Vec
/// let result : Result<Vec<u8>, _> = vec_with_capacity_safe(number_from_file);
///
/// assert!(result.is_err());
/// ```
pub fn vec_with_capacity_safe<T>(capacity : usize) -> Result<Vec<T>, WcsError>{
    let mut vec = Vec::new();
    match vec.try_reserve_exact(capacity){
        Ok(_) => Ok(vec),
        Err(e) => {
            match e {
                TryReserveError::AllocError { .. } => {
                    Err(WcsError::AllocError(e))
                },
                TryReserveError::CapacityOverflow => {
                    Err(WcsError::CapacityOverflow(e))
                }
            }
        }
    }
}
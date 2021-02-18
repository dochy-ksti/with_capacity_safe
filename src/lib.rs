#![feature(try_reserve)]

use std::fmt::{Debug, Display, Formatter};
use std::collections::TryReserveError;
use std::error::Error;

#[cfg(test)]
mod testing;

/// The cause of the error
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WcsErrorType{
    /// the computed capacity exceeding the usize::MAX_VALUE
    CapacityOverflow,
    ///The memory allocator returned an error
    AllocError
}

/// The error type for this crate. It implements std::error::Error
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WcsError{
    error_type : WcsErrorType,
    source: TryReserveError,
}

impl WcsError{
    pub fn error_type(&self) -> &WcsErrorType{ &self.error_type }
    pub fn source(&self) -> &TryReserveError{ &self.source }
}

impl Error for WcsError{

}

impl Display for WcsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error_type, f)
    }
}

/// Vec::with_capacity(capacity) panics/aborts when the capacity is too large.
/// This is a safer alternative which reports Error using try_reserve_exact
///
/// ```
/// use with_capacity_safe::{vec_with_capacity_safe, WcsError, WcsErrorType};
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
        Err(e) =>
            match e{
                TryReserveError::AllocError{ .. } =>{
                    Err(WcsError{ error_type : WcsErrorType::AllocError, source : e })
                },
                TryReserveError::CapacityOverflow =>{
                    Err(WcsError{ error_type : WcsErrorType::CapacityOverflow, source : e })
                }
            }
    }
}
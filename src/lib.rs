#![feature(try_reserve)]

use std::fmt::{Debug, Display, Formatter};
use std::collections::TryReserveError;

#[cfg(test)]
mod testing;

#[derive(Debug, PartialEq)]
pub enum WcsErrorType{
    CapacityOverflow,
    AllocError
}

#[derive(Debug, PartialEq)]
pub struct WcsError{
    error_type : WcsErrorType
}

impl WcsError{
    pub fn error_type(&self) -> &WcsErrorType{ &self.error_type }
    pub fn new(error_type : WcsErrorType) -> WcsError{ WcsError{ error_type } }
}

impl std::error::Error for WcsError{}

impl Display for WcsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

/// Vec::with_capacity(capacity) panics / aborts when the capacity is too large.
/// This is a safer alternative which reports Error using try_reserve
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
/// assert_eq!(result, Err(WcsError::new(WcsErrorType::AllocError)));
/// ```
pub fn vec_with_capacity_safe<T>(capacity : usize) -> Result<Vec<T>, WcsError>{
    let mut vec = Vec::new();
    match vec.try_reserve(capacity){
        Ok(_) => Ok(vec),
        Err(e) =>
            match e{
                TryReserveError::AllocError{ .. } =>{
                    Err(WcsError{ error_type : WcsErrorType::AllocError })
                },
                TryReserveError::CapacityOverflow =>{
                    Err(WcsError{ error_type : WcsErrorType::CapacityOverflow })
                }
            }
    }
}
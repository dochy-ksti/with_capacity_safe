

use std::collections::{TryReserveError};
use std::mem::ManuallyDrop;

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

/// A copy of Vec::into_raw_parts which is currently unstable.
/// The documentation below is also copy-pasted.
///
/// Decomposes a `Vec<T>` into its raw components.
///
/// Returns the raw pointer to the underlying data, the length of
/// the vector (in elements), and the allocated capacity of the
/// data (in elements). These are the same arguments in the same
/// order as the arguments to [`from_raw_parts`].
///
/// After calling this function, the caller is responsible for the
/// memory previously managed by the `Vec`. The only way to do
/// this is to convert the raw pointer, length, and capacity back
/// into a `Vec` with the [`from_raw_parts`] function, allowing
/// the destructor to perform the cleanup.
///
/// [`from_raw_parts`]: Vec::from_raw_parts
///
/// # Examples
///
/// ```
/// use with_capacity_safe::vec_into_raw_parts;
/// let v: Vec<i32> = vec![-1, 0, 1];
///
/// let (ptr, len, cap) = vec_into_raw_parts(v);
///
/// let rebuilt = unsafe {
///     // We can now make changes to the components, such as
///     // transmuting the raw pointer to a compatible type.
///     let ptr = ptr as *mut u32;
///
///     Vec::from_raw_parts(ptr, len, cap)
/// };
/// assert_eq!(rebuilt, [4294967295, 0, 1]);
/// ```
pub fn vec_into_raw_parts<T>(vec : Vec<T>) -> (*mut T, usize, usize) {
    let mut me = ManuallyDrop::new(vec);
    (me.as_mut_ptr(), me.len(), me.capacity())
}
use crate::{vec_with_capacity_safe, WcsError, WcsErrorType};

#[test]
fn it_works(){
    assert_eq!(vec_with_capacity_safe::<i32>(usize::max_value()), Err(WcsError{ error_type : WcsErrorType::CapacityOverflow}));
    assert_eq!(vec_with_capacity_safe::<u8>(usize::max_value()), Err(WcsError{ error_type : WcsErrorType::AllocError }));
}
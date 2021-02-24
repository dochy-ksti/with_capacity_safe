use crate::{vec_with_capacity_safe,  WcsError};

#[test]
fn it_works(){
    let r = vec_with_capacity_safe::<i32>(usize::max_value());
    match r{
        Err(WcsError::CapacityOverflow(_)) =>{},
        _ => unreachable!(),
    };
    //isize_max以上でCapacityOverlowって書いてあるけど、そうでもないみたいね https://doc.rust-lang.org/std/collections/enum.TryReserveError.html
    let r = vec_with_capacity_safe::<u8>(usize::max_value());
    match r {
        Err(WcsError::AllocError(_)) => {}
        _ => unreachable!(),
    };

}
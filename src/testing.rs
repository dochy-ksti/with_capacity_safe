use crate::{vec_with_capacity_safe,  WcsErrorType};

#[test]
fn it_works(){
    let r = vec_with_capacity_safe::<i32>(usize::max_value());
    match r{
        Ok(_) => unreachable!(),
        Err(e) => match e.error_type(){
            WcsErrorType::CapacityOverflow =>{},
            _ => unreachable!(),
        }
    };
    //isize_max以上でCapacityOverlowって書いてあるけど、そうでもないみたいね https://doc.rust-lang.org/std/collections/enum.TryReserveError.html
    let r = vec_with_capacity_safe::<u8>(usize::max_value());
    match r{
        Ok(_) => unreachable!(),
        Err(e) => match e.error_type(){
            WcsErrorType::AllocError =>{},
            _ => unreachable!(),
        }
    };

}
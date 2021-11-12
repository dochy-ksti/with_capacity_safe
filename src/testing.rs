use crate::{vec_with_capacity_safe};

#[test]
fn it_works(){
    let r = vec_with_capacity_safe::<i32>(usize::MAX);
    match r{
        Err(_) =>{},
        _ => unreachable!(),
    };
    //isize_max以上でCapacityOverlowって書いてあるけど、そうでもないみたいね https://doc.rust-lang.org/std/collections/enum.TryReserveError.html
    let r = vec_with_capacity_safe::<u8>(usize::MAX);
    match r {
        Err(_) => {}
        _ => unreachable!(),
    };

}
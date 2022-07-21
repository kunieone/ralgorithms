use ralgorithms::sort::*;
use std::time::Instant;

fn main() {
    let arr = gen_vec(10000000);
    let start = Instant::now();
    let _new_arr = counting::counting(arr);
    let duration = start.elapsed();
    // println!("Sorted Array is: {:?}", new_arr);
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let mut test1: (i32, Option<i32>) = (2, Some(123));
    test1.1 = None;
    println!("{:?}", test1);
}

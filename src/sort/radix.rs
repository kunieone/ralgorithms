/*
 * [122, 78, 90, 23]
 *   90 122  23  78 个位
 *  122 23 78 90    十位
 *  23 78 90 122 sorted!
 */
pub fn radix<T: Copy + Ord>(v: Vec<T>) -> Vec<T> {
    // arr
    // let max_dig =
    v
}

#[test]
fn radix_() {
    let test_arr = vec![1, 3, 5, 7, 9, 8, 6, 4, 2];
    println!("{:?}", radix(test_arr));
}

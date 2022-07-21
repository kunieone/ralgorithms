pub fn insertion<T: Ord + Copy>(mut arr: Vec<T>) -> Vec<T> {
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut move_ptr: usize = i - 1;
        while arr[move_ptr] > cur && move_ptr > 0 {
            arr[move_ptr + 1] = arr[move_ptr];
            move_ptr -= 1;
        }
        arr[move_ptr + 1] = cur;
    }
    arr
}
#[test]
fn insertion_test() {
    println!("{:?}", insertion(vec![1, 3, 5, 7, 9, 8, 6, 4, 2]))
}

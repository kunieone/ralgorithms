pub fn bucket<T: Ord + Copy>(mut arr: Vec<T>) -> Vec<T> {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                (arr[i], arr[j]) = (arr[j], arr[i]);
            }
        }
    }
    arr
}

#[test]
fn bucket_test() {
    println!("{:?}", bucket(vec![1, 3, 5, 7, 9, 8, 6, 4, 2]))
}

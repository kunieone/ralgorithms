use std::{ops::AddAssign, vec};

fn _counting<T>(arr: Vec<T>, max_val: usize) -> Vec<T>
where
    T: Into<usize> + From<usize> + Into<usize> + From<T> + Into<usize> + AddAssign + Copy,
{
    let mut bucket = vec![0; max_val + 1];
    let mut result: Vec<T> = vec![];
    for i in 0..arr.len() {
        bucket[arr[i].into() as usize] += 1;
    }

    for i in 0..bucket.len() {
        for _ in 0..bucket[i] {
            let tmp = i.into();
            result.push(tmp);
        }
    }
    result
}
pub fn counting<T>(arr: Vec<T>) -> Vec<T>
where
    T: Into<usize> + From<usize> + From<T> + AddAssign + Copy + Ord,
{
    let max = arr.iter().max().ok_or_else(|| 0).unwrap().to_owned().into();
    _counting(arr, max)
}

#[test]
fn counting_test() {
    let test_arr = vec![1, 3, 5, 7, 9, 8, 6, 4, 2];
    println!("{:?}", counting(test_arr));
}

pub fn bubble<T: Ord + Copy>(mut arr: Vec<T>) -> Vec<T> {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                (arr[i], arr[j]) = (arr[j], arr[i]);
            }
        }
    }
    arr
}

pub fn bubble2<T: Ord + Copy>(mut arr: Vec<T>) -> Vec<T> {
    for _ in 0..arr.len() {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                (arr[i], arr[i + 1]) = (arr[i + 1], arr[i]);
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_test() {
        assert_eq!(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            bubble(vec![1, 3, 5, 7, 9, 8, 6, 4, 2]),
        );
        println!("{:?}", bubble2(vec![1, 3, 5, 7, 9, 8, 6, 4, 2]))
    }
}

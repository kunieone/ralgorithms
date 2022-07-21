/* 晃一晃酒杯，就排好序了 */

pub fn cocktail<T: Ord>(mut arr: Vec<T>) -> Vec<T> {
    let l = arr.len();

    if l == 0 {
        return arr;
    }
    loop {
        let mut swaped = false;
        for i in 0..(l - 1).clamp(0, l) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swaped = true;
            }
        }

        if !swaped {
            break;
        }
        swaped = false;

        for i in (0..(l - 1).clamp(0, l)).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swaped = true;
            }
        }
        if !swaped {
            break;
        }
    }
    arr
}

// #[test]
// fn cocktail_test() {
//     println!("{:?}", cocktail(vec![1, 3, 5, 7, 9, 8, 6, 4, 2]))
// }

pub fn fibonacci_rc(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci_rc(n - 1) + fibonacci_rc(n - 2),
    }
}

pub fn fibonacci_df(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    }
    match n {
        0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
        1 | 2 => 1,
        _ => (|| {
            let mut sum = 0;
            let mut last = 0;
            let mut curr = 1;
            for _i in 1..n {
                sum = last + curr;
                last = curr;
                curr = sum;
            }
            sum
        })(),
    }
}
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}
pub fn iterative_fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}
#[test]
fn t() {
    let a = fibonacci_df(50);
    println!("{a}")
}

#[test]
fn t2() {
    let a = fibonacci_df(100100);
    println!("{a}")
}

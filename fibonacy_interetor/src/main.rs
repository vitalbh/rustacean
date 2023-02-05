use std::ops::Add;

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }


}
impl std::ops::Add for Fibonacci {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {curr: other.curr, next: other.next}
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };

    for (i, n) in fib.enumerate().take(6) {
        println!("fib({i}): {n}");
    }

    // just try traits of add and filter
    let fib = Fibonacci { curr: 0, next: 1 };
    let fib2 = Fibonacci { curr: 2, next: 3 };
    let filter_next: u32 = 2;
    for (i, n) in fib.add(fib2).enumerate().take(6).filter(|p| p.1.eq(&filter_next)) {
        println!("fibfilter({i}): {n}");
    }

}
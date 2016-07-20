struct Fib {
    prev: u32,
    cur: u32,
}

impl Iterator for Fib {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {

        let tmp = self.prev + self.cur;
        self.prev = self.cur;
        self.cur = tmp;
        Some(self.prev)

    }
}
impl Fib {
    fn new() -> Fib {
        Fib { cur: 1, prev: 0 }
    }
}
fn main() {
    for i in Fib::new().take(20) {
        println!("{}", i);
    }
}

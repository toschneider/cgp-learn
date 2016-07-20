struct Typ {
    prev: u32,
    cur: u32,
}

impl Iterator for Typ {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        // if  {
        let tmp = self.prev + self.cur;
        self.prev = self.cur;
        self.cur = tmp;
        Some(self.prev)
        // } else {
        //     None
        // }
    }
}
impl Typ {
    fn new() -> Typ {
        Typ { cur: 1, prev: 0 }
    }
}
fn main() {
    for i in Typ::new().take(20) {
        println!("{}", i);
    }
}

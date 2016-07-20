use std::fmt;
struct Swagger<T> {
    a: T,
}

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#swag {} #yolo", self.a)
    }
}


fn main() {
    let a = Swagger { a: 4 };
    println!("{}", a);
}

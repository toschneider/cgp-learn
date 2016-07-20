use std::ops::{Add, Mul};
fn main() {
    println!("{:?}", func(3, 4));
}

fn func<T: Mul + Add + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (a + b, a * b)
}

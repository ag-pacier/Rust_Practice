enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

[#allow(dead_code)]
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    // Box is a struct that puts a pointer on the stack to a piece of data on the heap

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // Normally the compiler is going to look at the largest possible variant of the enum to ensure it can
    // makes space for it. Using Box<T> Rust knows it'll always be the same amount of space in that it will
    // always be a pointer to some data on the heap

    // Box<T> implements the Drop trait which means after it goes out of scope, Rust can clean up the data
    // It also implements the Deref trait so that the data on the heap can be pulled


}

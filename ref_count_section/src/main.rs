enum List {
        Cons(i32, Rc<List>),
        Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

#[allow(unused_variables)]
fn main() {
    // Rc<T> is an abbreviation for reference counting and allows for multiple ownership
    // it keeps track of the number of references to a value which determines if the value is in use
    // Rc<T> puts data on the heap for multiple pointers to point to
    // This is only for single threaded items! It will not work for concurrency!!

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Count after creating c = {}", Rc::strong_count(&a));

    // RefCell<T> type borrowing is enforced at runtime and if broken, cause the program to panic
    // This is in contrast to Box<T> which enforced borrowing at compile time
}
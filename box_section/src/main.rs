use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// In order to implement the Deref trait, we need to use type Target = T; which is covered more later :(
// we create the related function deref so we can implement dereferencing on our new struct
// deref returns a reference to the value we want to access with the * operator

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[allow(unused_variables)]
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

    // Because Box can use Deref, this works:
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // Drop runs automatically once the vars go out of scope. However!
    // If you want to clean up something early, like to open a lock for something else, it's not so straight forward
    // You can't just call drop early using d.drop() ! It won't compile. Doing so would create a "double free" error where
    // Rust drops the value when you call it and again once it officially goes out of scope so you need to use std::mem::drop
    // and call the function on it:
    drop(c);

    println!("CustomSmartPointer c dropped before the end of main!");

}

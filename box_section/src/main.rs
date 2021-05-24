fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    // Box is a struct that puts a pointer on the stack to a piece of data on the heap
}

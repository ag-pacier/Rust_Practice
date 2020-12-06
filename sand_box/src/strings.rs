// primitive str = imutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure -- use when you need to modify or own string data

pub fn run() {
    let hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    println!("{}", hello);
}
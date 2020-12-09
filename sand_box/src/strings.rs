// primitive str = imutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure -- use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
    // push char
    hello.push('W');
    //push str
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    //check if empty
    println!("Is Empty: {}", hello.is_empty());

    //contains
    println!("Contains 'World': {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{}", hello);
}
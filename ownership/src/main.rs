fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let y = x;
    //This little section shows that integers get copied, not moved
    println!("X: {}, Y: {}", x, y);
    let x = x + 1;
    println!("Adding 1 to X: {}", x);
    println!("Y: {}", y);

    //This moves the string to s2, s1 stops existing after line 17
    let s1 = String::from("hello");
    let s2 = s1;

    //Bring s1 back by cloning s2
    let s1 = s2.clone();
    println!("S1 = {} . S2 = {}", s1, s2);

    //This function takes ownership of s2 and s2 stops existing after this function
    takes_ownership(s2);

    //Integer has the copy trait so this function won't remove it after using it
    makes_copy(x);
    println!("Proof X still exists after makes_copy: {}", x);

    //s1 takes ownership of returned value of function
    let s1 = gives_ownership();
    //s2 is a new String on the heap
    let s2 = String::from("yes, hello.");
    //Function takes the ownership away from s2, so it goes away
    //The string is returned again and given to s3
    // Ownership goes S2 -> function -> S3
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s3: {}", s1, s3);

    //Returning stuff as a tuple, passing a string and getting
    //the string back to see the length. They are trying to prove a point
    //and I need practice
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Here you go.");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
#[allow(unused_variables, unused_mut)] //shutting off the warnings since I'm just dorking around
fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    //Or can be done right on the literal:
    let s = "initial contents".to_string();
    //alternative, using the from method of String
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");
    //push_str automatically takes a string slice in case you want to use the var later
    let s2 = "bun";

    s.push_str(s2);
    println!("s is now: {} BUT I can still use s2: {}", s, s2);

    //push only takes a single character
    s.push('z');
    println!("s is now: {}", s);

    //You can concat strings using +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 is moved in this statement so it goes away

    //The + uses an "add" method in string which accepts a string slice hence the &
    //Rust is smart enough to "force" a &str if it gets a String

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // doing pluses would get unwieldy so...
    let s = format!("{}-{}-{}", s1, s2, s3);
    //The above also does not take ownership of any of those variables!

    //Strings are stored UTF-8 and thus can't just be iterated over by default like Python

    let rus_hello = String::from("Здравствуйте");
    let hindi_hello = String::from("नमस्ते");

    println!("Byte indexing for rus_hello (0-3): {}", &rus_hello[0..4]);
    println!("Doing the chars for hindi_hello:");
    for c in hindi_hello.chars() {
        println!("{}", c);
    };
    println!("Doing the bytes for hindi_hello:");
    for b in hindi_hello.bytes() {
        println!("{}", b);
    };
}

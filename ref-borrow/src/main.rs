fn main() {
    //Section 4.2 work since 4.1 was getting full

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    let mut s2 = String::from("Pickle chips");
    change(&mut s2);

    println!("{}", s2);

    //Moving to section 4.3 "slices"
    let s = String::from("Hello world");

    let hello = &s[..5];
    let world = &s[6..];

    println!("This is a manual slice: {} {}", hello, world);
    let s2 = first_word(&s);
    println!("First word of {} is {}", s, s2);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn calculate_length(s: &String) -> usize {
    //Using a reference means s goes out of scope but nothing happens
    //because the thing it references is still kicking within the scope
    //it was declared. This is different than reassigning like the end
    //of 4.1
    s.len()
    //The big deal with this method is that references are immutable by default
}

fn change(some_string: &mut String) {
    //We can use a mutable reference if the variable provided is mutable
    //So we can't have an immutable variable give a mutable reference
    some_string.push_str(" in bed");
    //One big issue: you can only have 1 mutable reference per scope
    //This prevents data races
    //For a similar reason, we can't have references and then a mutable
    //reference in the same scope
    //Yes, I know I'm 5 but my example is clean, I promise
}
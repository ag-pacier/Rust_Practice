fn main() {
    //Section 4.2 work since 4.1 was getting full

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
use std::io;

fn main() {
    println!("Pick what you are converting FROM:");
    println!("1. Celsius");
    println!("2. Fahrenheit");
    //Get input here
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Could not read input");
    println!("Put in temperature to be converted:");
    //Get other input here
}
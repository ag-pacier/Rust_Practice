fn main() {
    let x = 2.0; //defaults to type f64
    let y: f32 = 3.0; //set to f32 since we annoted it

    println!("x is {} and y is {}", x, y);

    // addition
    let sum = x + y;
    // subtraction
    let difference = x - y;
    // multiplication
    let product = x * y;
    // division
    let quotient = x / y;
    // remainder/modulo
    let remainder = y % x;

    println!("Sum: {}, Difference: {}, Product: {}, Division: {}, Remainder: {}", sum, difference, product, quotient, remainder);
}

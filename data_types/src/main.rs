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

    // tuple declaration
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //deconstruct tuple
    let (x, y, z) = tup;

    println!("The new value of y is: {}", y);
    println!("The other values are: {} {}", x, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{:?}", x);
    println!("Split out as {} {} and {}", five_hundred, six_point_four, one);
}

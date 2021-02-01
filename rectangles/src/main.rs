fn original() {
    //section 5.2 original program
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.", area(width1, height1));

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn tu_second() {
    original();
    println!("Now with a tuple for the arguements");

    let rect1 = (30, 50);

    println!("The area of the rectangle is {} square pixels.", tu_area(rect1));
}

fn tu_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    tu_second();
    println!("Now using structs!");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Using this rectangle: {:?}", rect1);
    //Test the # formatting
    println!("Also better formatted: {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", st_area(&rect1));

}

fn st_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
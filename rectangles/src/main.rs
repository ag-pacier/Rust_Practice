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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
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

    println!("Now using a method attached to the Rectangle struct: {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(6);
    println!("rect4 using square: {:?}", rect4);

}

fn st_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
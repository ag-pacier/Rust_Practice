#[derive(Debug)] //added via suggestion from the compiler so I can use {:?}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//structs can be outside the main loop so they can be used by other stuff :P
//This is for section 5.1 in the rust book

fn main() {

    let mut user1 = User {
        email: String::from("someone@mydomain.com"),
        username: String::from("somenerd"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1's email: {}", user1.email);
    user1.email = String::from("hello@mfer.com");
    println!("User stuff: {:?}", user1);

    let user2 = User {
        email: String::from("another@mfer.com"),
        username: String::from("anotherusername"),
        ..user1
        //struct update syntax means the ..and the already existing struct
        //means undescribed items are the same as the mentioned struct
    };

    println!("User2, based on user1: {:?}", user2);

    let user3 = build_user(String::from("candy@man.org"), String::from("candyman"));

    println!("User3, using build_user funct: {:?}", user3);

    //tuple struct part of the show
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black values: {} {} {}", black.0, black.1, black.2);
    println!("Origin values: {} {} {}", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
    //field init shorthand: since the parameters are named the same as the field names,
    //we can just use them.
}

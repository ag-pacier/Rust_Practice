#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Massachusetts,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //section 6.2
    let cents = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Valued at {} cents", cents);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //section 6.3

    //Verbose way using match
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    //More concise way using if let but less thorough
    //I like this line "...you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values"
    if let Some(3) = some_u8_value {
        println!("three");
    }

}

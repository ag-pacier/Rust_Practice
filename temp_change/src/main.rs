use std::io;

fn main() {
    loop {
        println!("Pick what you are converting FROM:");
        println!("1. Celsius");
        println!("2. Fahrenheit");
        println!("Typing anything but 1 or two causes this program to exit.");
        let choice = get_choice(2);
        if choice == 0 {
            break;
        }
        println!("Put in temperature to be converted:");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Could not read input");
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => 0.0,
        };
        let result = convert_temp(choice, temp);
        println!("The temp is: {}", result);
    }
}

fn get_choice(max: u8) -> u8 {
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Could not read input");
    let selection: u8 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => return 0,
        };
    if max < selection {
        return 0;
    } else {
        return selection;
    }
}

fn convert_temp(version: u8, temp: f32) -> f32 {
    if version == 1 {
        return temp * (9.0 / 5.0) + 32.0
    } else {
        return temp - 32.0 * (5.0 / 9.0)
    }
}
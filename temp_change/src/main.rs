use std::io;

fn main() {
    loop {
        println!("Pick what you are converting FROM:");
        println!("1. Celsius");
        println!("2. Fahrenheit");
        println!("Typing anything but 1 or 2 causes this program to exit.");
        let choice = get_choice(2);
        if choice == 0 {
            break;
        }
        let temp = get_temp();
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

fn get_temp() -> f32 {
    loop{
        println!("Put in temperature to be converted:");
        let mut temp = String::new();
            io::stdin()
                .read_line(&mut temp)
                .expect("Could not read input");
            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            return temp;
        }
}

fn convert_temp(version: u8, temp: f32) -> f32 {
    if version == 1 {
        let result = temp * (9.0 / 5.0);
        let result = result + 32.0;
        return result
    } else {
        let result = temp - 32.0;
        let result = result * (5.0 / 9.0);
        return result
    }
}
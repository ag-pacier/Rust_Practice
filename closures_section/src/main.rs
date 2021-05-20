use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num:u32| -> u32 { println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
            );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
            );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[allow(dead_code)]
fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
// Okay, now to explain to my future self so I don't forget:
//The big difference here between closures and functions is that closures can use variables within the scope they are declared
//So if we make equal_to_x a function, suddenly it doesn't work since what's in the function is out of scope of wherever it is placed
//Closures have three patterns, they either take ownership of the variable (FnOnce) or the borrow the variable mutably (FnMut) or borrow immutably (Fn)

//I spent an hour staring at the rust book trying to understand this and I think I got it now
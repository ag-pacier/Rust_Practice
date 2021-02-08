use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

//Can be done in a shorter way explored later
//Returns a result type of either the string or an io::Error which is what open and read can return
//In each case, we are passing along the error we get from the other functions so that when the
//current function fails, it passes along the error we get to the top so we can troubleshoot
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

//We can use the ? to propagate errors to the top, which slims things down:
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
//The above functions do the same thing!
//? does 1 thing differently in that it uses the from function to convert errors
//We can slim this down even more:
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
//Same thing again!


//We can change our uses to shrink things down further:
//use std::fs;
//use std::io;
//fn read_username_from_file() -> Result<String, io::Error> {
//    fs::read_to_string("hello.txt")
//}
//Boom

//The ? only works on functions that have implemented the std::ops::Try
//It does not work on the main function

//Main can have no return () or Result<T, E> using a trait object Box<dyn Error>

fn main() {
    //panic!("crash and burn");

    let v = vec![1, 2, 3, 4];
    println!("{:?}", v);

    //v[99]; this panics so I can see what it looks like

//    let f = match f {
//        Ok(file) => file,
//        Err(error) => match error.kind() {
//            ErrorKind::NotFound => match File::create("hello.txt") {
//                Ok(fc) => fc,
//                Err(e) => panic!("Problem creating the file: {:?}", e),
//            },
//            other_error => {
//                panic!("Problem opening the file: {:?}", other_error)
//            }
//        },
//    };
//The above is kinda primitive way of catching these errors. A better way:
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //This will pull the value out of the file or panic for us
    let _f = File::open("hello.txt").unwrap();

    //This will do the same as above but let us customize the panic message
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

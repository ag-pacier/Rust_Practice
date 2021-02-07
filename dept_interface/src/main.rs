use std::collections::HashMap;
use std::io;

fn main() {
    //Create a text interface to allow a user to add employee names to a department in a company.
    //Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically
    let helptext = "Typical usage is: Add *EMPLOYEE* to *DEPARTMENT* or List *DEPARTMENT* or List all. You can also quit by typing quit.";
    println!("Welcome to Company. Please input command. For a list of available commands, type in help.");
    let mut hash_company = HashMap::new();
    hash_company.insert("Employee", "Department");
    loop {
        let mut command = String::new();
        println!("Command:  ");
        io::stdin().read_line(&mut command).expect("Unable to read command.");
        command.truncate(command.len() -2); //removing /r/n from the end of the command captured
        let new_command = vec!(command.split_whitespace());

        match  {
            "help" => println!("{}", helptext),
            "quit" => break,
            "Add" => continue,
            "List" => continue,
            "test" => continue,
            _ => println!("Command not found, please try again."),
        };
    };
}

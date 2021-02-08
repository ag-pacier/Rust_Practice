use std::collections::HashMap;
use std::io;

fn add_user(hash: &mut HashMap<String, String>, com: &Vec<&str>) {
    if com.len() <= 3 {
        println!("Invalid length. Proper usage: Add *EMPLOYEE* to *DEPARTMENT*.");
        return
    };
    let mut employee = String::new();
    let mut department = String::new();
    let mut to_catch = false;
    for i in com.iter() {
        if i == &"Add" {
            continue
        };
        if to_catch {
            department.push_str(&i);
            department.push(' ');
        } else if i.to_lowercase() == "to" {
            to_catch = true;
        } else {
            employee.push_str(&i);
            employee.push(' ');
        };
    };
    hash.entry(employee).or_insert(department);
}

fn list_hash(hash: &HashMap<String, String>, com: &Vec<&str>) {
    let error = String::from("Invalid length. Proper usage: List *DEPARTMENT* or List all.");
    if com.len() >= 3 {
        println!("{}", error);
        return
    } else if com.len() == 1 {
        println!("{}", error);
        return
    };
    if com.get(1) == Some(&"all") {
        for (emp, dep) in hash {
            println!("Employee: {}, Department: {}", emp, dep);
        };
    } else {
        println!("Employees in {}:", com.get(1).unwrap());
        for (emp, dep) in hash {
            if dep == com.get(1).unwrap() {
                println!("{}", emp);
            }
        }
    };
}

fn main() {
    //Create a text interface to allow a user to add employee names to a department in a company.
    //Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically
    let helptext = "Typical usage is: Add *EMPLOYEE* to *DEPARTMENT* or List *DEPARTMENT* or List all. You can also quit by typing quit.";
    println!("Welcome to Company. Please input command. For a list of available commands, type in help.");
    let mut hash_company: HashMap<String, String> = HashMap::new();
    loop {
        let mut command = String::new();
        println!("Command:");
        io::stdin().read_line(&mut command).expect("Unable to read command.");
        let new_command: Vec<&str> = command.split_ascii_whitespace().collect();
        match new_command.get(0) {
            Some(&"help") => println!("{}", helptext),
            Some(&"quit") => break,
            Some(&"Add") => add_user(&mut hash_company, &new_command),
            Some(&"List") => list_hash(&hash_company, &new_command),
            Some(&"test") => println!("Captured vec: {:#?}", new_command),
            None => continue,
            _ => println!("Command not found, please try again."),
        };
    };
}

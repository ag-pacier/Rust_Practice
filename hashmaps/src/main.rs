use std::collections::HashMap;

fn main() {
    //section 8.3
    //Hashmaps or what python calls dictionaries

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //Hashmaps are like vectors in that all the keys have to be the same type and all the values need to be the same type

    //Another way to create the map is to "collect" a couple of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    //Iterators and methods are covered more in section 13
    //collect method gathers data into a collection
    //zip method creates a vector of tuples
    //We have to put in the type of Hashmap and for the type inside the hashmap, Rust can easily infer in this case so we use _
    //If for some reason it couldn't infer, we'd put HashMap<String, i32>

    scores.insert(String::from("Green"), 35);

    //We can use get and a reference to the key to find a value
    //Since I wanted to display the score, I used a match and dereferenced what was returned
    //If the key doesn't exist, instead of flipping a pancake, the program returns 0
    let team_name = String::from("Green");
    let score = match scores.get(&team_name) {
        None => 0,
        Some(num) => *num,
    };

    println!("For the {} team, the score is {}", team_name, score);

    //We can iterate over HashMaps in arbitrary order

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    };

    //When we run an insert on an existing key, it overwrites its value
    scores.insert(String::from("Green"), 65);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    };

    //We can insert a value only if the key has no value using entry, which returns an enum called Entry
    print!("Checking for 'Blue':  ");
    println!("{:?}", scores.entry(String::from("Blue")));
    print!("Checking for 'Purple':  ");
    println!("{:?}", scores.entry(String::from("Purple")));
    println!("Now adding a value for 'Purple'...");
    scores.entry(String::from("Purple")).or_insert(20);
    println!("Scores:");
    println!("{:#?}", scores);

    //We can also update a value based on the old value
    let text = "hello world it hello the world yes a world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    };

    println!("The breakdown of words in the text is: {:#?}", map);
}

fn pig_latin_word(word: &str) -> String {
    let mut new_word = String::new();
    let first_letter = &word.chars().next().unwrap_or_default().to_ascii_lowercase();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    if vowels.contains(&first_letter) {
        new_word.push_str(&word);
        new_word.push_str("-hay");
    } else {
        new_word.push_str(&word[1..]);
        new_word.push_str("-");
        new_word.push(*first_letter);
        new_word.push_str("ay");
    };
    new_word
}

fn main() {
    //Convert strings to pig latin
    let test1 = String::from("Apple");
    let test2 = String::from("Jerk");
    let test3 = String::from("Muppet");

    println!("Apple: {}, Jerk: {}, Muppet: {}", pig_latin_word(&test1), pig_latin_word(&test2), pig_latin_word(&test3));
}

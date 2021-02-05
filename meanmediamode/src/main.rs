use std::collections::HashMap;

fn mean_ints(ints: &Vec<i32>) -> i32 {
    let mut answer: i32 = 0;
    let mut count = 0;
    for num in ints.into_iter() {
        answer = num + answer;
        count += 1;
    };
    answer / count
}

fn median_ints(ints: &Vec<i32>) -> i32 {
    let mut sorted_vec = ints.clone();
    sorted_vec.sort();
    match sorted_vec.get(sorted_vec.len() / 2) {
        Some(num) => return *num,
        None => return 0,
    }
}

fn mode_ints(ints: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in ints.into_iter() {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    };
    let mut answer: i32 = 0;
    let mut answer_value: i32 = 0;
    for (key, value) in map {
        if value > answer_value {
            answer = *key;
            answer_value = value;
        };
    };
    answer
}

fn main() {
    //first exercise from section 8
    //Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position),
    //and mode (the value that occurs most often; a hash map will be helpful here) of the list.

    let test_vec = vec![10, 200, 10, 4, 10, 6, 7, 8, 9];
    println!("Test vec: {:?}", test_vec);
    println!("Mean: {}", mean_ints(&test_vec));
    println!("Median: {}", median_ints(&test_vec));
    println!("Mode: {}", mode_ints(&test_vec));
}

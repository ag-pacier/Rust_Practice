//Because there is only one T we can only use one type per creation
struct Point<T> {
    x: T,
    y: T,
}

struct NewPoint<T, U> {
    x: T,
    y: U,
}


fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//This doesn't compile as this needs to implement std::cmp::PartialOrd which is a trait
//Traits are in another section :/
//fn largest<T>(list: &[T]) -> &T {
//    let mut largest = &list[0];
//
//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//    largest
//}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is: {}", largest_i32(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    
    println!("Now the largest number is: {}", largest_i32(&number_list));

    let char_list = vec!['y', 'o', 'u', 'z'];

    println!("The biggest char is: {}", largest_char(&char_list));

    let point_1 = Point { x: 5, y: 4};
    let point_2 = Point { x: 4.5, y: 5.5};
    let point_3 = NewPoint { x: 4, y: 4.5};

    println!("Point 1: {}:{}, Point 2: {}:{}, Point 3: {}:{}", point_1.x, point_1.y, point_2.x, point_2.y, point_3.x, point_3.y);
}

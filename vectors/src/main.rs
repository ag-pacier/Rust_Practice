fn main() {
    //Vec<T> or vectors. You can store more than one value in a single data structure that puts all the values next to each other in memory
    //Literally just figured out that <T> means <TYPE> even though I've seen it many times in the book
    //So we make a vector and note the type since we haven't added values yet
    let v: Vec<i32> = Vec::new();

    println!("v vector: {:?}", v);

    //Typically you'd add values immediately so there is a macro that you can use to make the vector and set the type just by your entries
    let v = vec![1, 2, 3];

    println!("v vector: {:?}", v);

    //The two above do almost the same thing, but the second v adds values as it declares a Vec<i32> infering since the values provided are i32

    let mut v = Vec::new();

    for i in 5..9 {
        v.push(i);
    }

    println!("v vector: {:?}", v);

    //To access a vector, we can use index or the get method
    let mut v = Vec::new();

    for i in 1..6 {
        v.push(i);
    }

    let third: &i32 = &v[2];
    println!("The third element of this version of v is {}", third);

    match v.get(2) {
        Some(third) => println!("Match found third element {}", third),
        None => println!("Match did not find the third element"),
    };

    //As in previous chapters, you cannot have mutable and immutable references of the same variable in the same scope
    //For vectors, this is important as sometimes vectors will need to be moved in memory to make space for new values
    //So any references or borrows prior to the mutable borrow have to go poof to prevent referencing deallocated memory

    println!("Printing each value in v:");
    for i in &v {
        println!("{}", i);
    };

    println!("Now adding 50 to each part of v...");
    for i in &mut v {
        *i += 50;
    };
    println!("Done. v looks like this {:?}", v);

    //We can use enums if we want a vector with multiple "types" of info
    //The enum becomes the type the vector accepts and then inside each enum, we have the stuff we really need

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    //It's not subversion if that is how it was designed!

    //More here: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
    //Might have to do another rundown of this to really let it sink in
}

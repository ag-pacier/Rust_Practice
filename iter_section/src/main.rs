fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
        //nom nom nom, consuming the iterator, sortof
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    //Iterators are "lazy" and need to be consumed. That's what collect is doing.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

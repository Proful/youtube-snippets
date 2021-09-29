#![allow(warnings)]

// ch8 - part1
// Introduction to collections
// ^ Vector vec<T>
// - Creation of vector (2 ways)
// - Mutating vector
// - Accessing vector (2 ways)
// - Mixing immutable & mutable references
// - iterating vector
// - mutating vector data while iterating vector
fn main() {
    // type is required
    // store data of same type
    let numbers:Vec<i32> = Vec::new();
    let mut numbers = vec![5, 10, 15];
    numbers.push(20);
    numbers.push(25);

    let second = &numbers[1];//10 => panic
    dbg!(second);
    // dbg!(numbers);
    //20 => return None without panicking
    let third = numbers.get(2);
    match third {
        Some(value) => println!("value = {}", value),
        None => println!("None"),
    }
    // numbers.push(50); ==> work
    // immutable & mutable confusion
    let first = &numbers[0];
    // Big if, if there is no space near the heap to add new elements
    // rust will copy to a new address entire elements
    // numbers.push(50); ==> error
    dbg!(first);
    // numbers.push(50); ==> work

    // numbers => ownership transferred
    // so use immutable reference
    for num in &numbers {
        dbg!(num);
    }
    dbg!(&numbers);

    // *num is important to mutate
    // * => dereference operator (get the value in i)
    for num in &mut numbers {
        *num += 100;
        dbg!(num);
    }
    dbg!(&numbers);

}

//@ Collections
// data stored in the heap, can grow or shrink
// length no need to be known at compile time
// Vector, String, Hashmap
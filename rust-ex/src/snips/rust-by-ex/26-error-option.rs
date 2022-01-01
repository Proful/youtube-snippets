#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    // panic!("Danger, Will Robinson!");

    let x = Some(5);
    // let x: Option<i32> = None;

    match x {
        Some(y) => println!("x = {:?}", y),
        None => println!("x = None"),
    }

    let y = x.unwrap(); // will panic if x is None
    println!("y = {}", y);

    let z = x.expect("x is none");
    println!("z = {}", z);

    println!("add_one(1) = {:?}", add_one(x));
}

fn add_one(x: Option<i32>) -> Option<i32> {
    let mut z = x?; // will return None if x is None
    z += 1;
    Some(z)
}
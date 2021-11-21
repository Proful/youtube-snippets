#![allow(warnings)]

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 100, y: 200};

    match p {
        Point { x, y: 200 } => println!("x = {}", x),
        Point { x: 100, y } => println!("y = {}", y),
        Point { x, y } => println!("x = {}, y = {}", x, y),
    }
}
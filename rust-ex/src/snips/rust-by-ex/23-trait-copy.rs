#![allow(warnings)] // NOT RECOMMENDED

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 0, y: 7 };
    let p2 = p1;
    let p3 = p2.clone();

    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
}

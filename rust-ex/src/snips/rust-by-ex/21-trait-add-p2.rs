#![allow(warnings)] // NOT RECOMMENDED

use std::ops::Add;

#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };

    let p3 = p1 + p2;

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    sdbg!(p2 == p3);
}

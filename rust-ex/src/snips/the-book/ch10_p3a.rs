#![allow(warnings)]

trait Shape {
    fn area(&self) -> i32;
}

#[derive(Debug)]
struct Square {
    height: i32,
}

impl Shape for Square {
    fn area(&self) -> i32 {
        self.height * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

// Ch 10 - part3 (a)
//^ Trait
fn main() {
    let sq = Square { height: 50 };

    display_area(&sq);

    let rect = Rectangle {
        width: 20,
        height: 40,
    };

    display_area(&rect);
}

fn display_area<T: Shape + std::fmt::Debug>(shape: &T) {
    dbg!(shape);
    dbg!(shape.area());
}

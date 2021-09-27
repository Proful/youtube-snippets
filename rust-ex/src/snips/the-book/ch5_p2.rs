#![allow(warnings)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // Associated functions don’t take self as a parameter
    // they don’t have an instance of the struct to work with
    // :: syntax with the struct name to call this associated function
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    // method not fn
    // . when we call a method
    fn aria(&self) -> u32 {
        self.width * self.height
    }
}

// Ch5 - Part 2
fn main() {
    let rect = Rectangle {
        width: 25,
        height: 25,
    };
    // dbg!(aria(rect));
    // dbg!(aria(rect))
    dbg!(rect.aria());
    dbg!(Rectangle::aria(&rect));
    dbg!(&rect.aria()); // automatic referencing behaviour
    dbg!(Rectangle::new(23, 54));
}

fn aria(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

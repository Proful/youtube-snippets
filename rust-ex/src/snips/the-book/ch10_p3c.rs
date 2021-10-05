#![allow(warnings)]

use std::cmp::PartialEq;
use std::fmt::Display;

struct Rectangle {
    width: i32,
    height: i32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rectabgle => width = {}, height = {}",
            self.width, self.height
        )
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

// Ch 10 - part3 (b)
//^ Trait continued...
fn main() {
    let rect = Rectangle {
        width: 20,
        height: 40,
    };

    // dbg!(rect);
    println!("rect = {}", rect);

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    if rect == rect2 {
        println!("Both rect are equals");
    }
}

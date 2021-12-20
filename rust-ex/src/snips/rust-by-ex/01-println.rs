#![allow(warnings)]  // NOT RECOMMENDED

use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(x = {}, y = {})", self.x, self.y)
    }
}

/**
 *   main function
 *   this is some more notes
 */
/// docs for the following item
fn main() {
    println!("hello friends!");
    dbg!("hello friends!");

    let age = 30;
    println!("My age is {} year", age);

    // dbg macro
    dbg!(age);

    println!("Name = {} age = {} {0}", "Proful", age);
    let msg = format!("Name = {name} age = {age} {name}", name = "Proful", age=age);

    dbg!(msg.to_uppercase());

    println!("{:b}", 25);
    println!("{val:>pad$}", val=5, pad=10);
    println!("{val:x>pad$}", val=5, pad=10);

    let p = Point { x: 5, y: 10 };
    println!("{:?}", p); // debug format
    println!("{:#?}", p); // pretty print
    dbg!(&p);

    println!("{}", p); // display format


}
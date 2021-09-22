#![allow(warnings)]

use colour::{blue_ln, cyan_ln};
// no `class` in rust
#[derive(Debug)]
struct User {
    name: String,
    city: String,
    age: i64, // u8 => 1byte
}
fn main() {
    let user = User {
        name: "Proful".to_string(),    // &str => String
        city: String::from("Jeypore"), // &str => String
        age: 28,
    };
    // println!("{}", user);
    cyan_ln!("{:?}", user);
    blue_ln!("{:#?}", user);
}

#![allow(warnings)]

use colour::*;

// chapter 4 -- part 5
fn main() {
    // Dangling references
    let ref_name = dangle();
}

fn dangle() -> &String {
    let name = String::from("Proful");
    &name
}

fn no_dangle() -> String {
    let name = String::from("Proful");
    name
}

// one mutable ref OR multiple immutable ref
// ref must be valid

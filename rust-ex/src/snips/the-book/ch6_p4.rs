#![allow(warnings)]

// Ch6 - Part 4
fn main() {
    let my_age = Some(30);
    dbg!(add_ten(my_age));
    dbg!(add_ten(None));

    for i in [1, 2, 3, 4, 5] {
        match i {
            1 => println!("Woohoo"),
            _ => println!("Yahh!!"),
        }
    }
}

fn add_ten(age: Option<u8>) -> Option<u8> {
    match age {
        Some(value) => Some(value + 10),
        None => None,
    }
}

#![allow(warnings)]

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    role: String,
    // role: &str, won't work need lifetime
}

// Tuple structs
// without named fields
#[derive(Debug)]
struct Color(u8, u8, u8);

// Unit-Like Structs Without Any Fields
// don’t have any fields!
// behave similarly to ()
// same as const Cookie: Cookie = Cookie {}
// rarely useful on its own
struct Cookie;

// Ch5 - Part 1
fn main() {
    let mut emp = Employee {
        id: 1,
        name: String::from("Proful"),
        role: String::from("Developer"),
    };
    emp.name = String::from("Kenny");
    // dbg!(emp) => move ownership
    dbg!(&emp);
    println!("{}", emp.role);
    let mut emp2 = Employee {
        id: 2,
        name: String::from("Steve"),
        ..emp // spread symtax
    };
    // println!("{}", emp.role);
    // emp already moved can't use it here
    dbg!(emp2);

    let white = Color(255, 255, 255);
    dbg!(white);
}

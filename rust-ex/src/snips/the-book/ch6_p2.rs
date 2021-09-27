#![allow(warnings)]

// Ch6 - Part 2
fn main() {
    let some_num = Some(5);
    let some_str = Some("Hi");
    dbg!(some_num, some_str);
    dbg!(some_num.unwrap());
    dbg!(some_str.unwrap());

    let absent: Option<i32> = None;
    dbg!(absent);
}

// enum Option<T> {
//     Some(T),
//     Nonw
// }

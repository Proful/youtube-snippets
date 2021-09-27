#![allow(warnings)]

#[derive(Debug)]
enum Status {
    Open(i32, i32),
    Closed(i32),
}
// enum Status {
//     Open,
//     Closed,
// }

// Ch6 - Part 1
fn main() {
    let open = Status::Open(45, 55);
    let closed = Status::Closed(50);

    dbg!(open);
}

#![allow(warnings)]

enum Move {
    Up,
    Down,
    Left,
    Right,
}
// Ch6 - Part 5
fn main() {
    let move_to = Move::Down;
    // let move_to = Move::Left;

    // shorthand syntax to match & implement all scenarios
    // using if let implement what u want, rest ignores!
    // How it is different from normal if??
    // if => expected `bool`, found `()`
    if let Move::Left = move_to {
        dbg!("Move Left");
    } else {
        dbg!("Move Unknown");
    }
}

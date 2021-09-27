#![allow(warnings)]

enum Move {
    Up,
    Down,
    Left,
    Right(u8),
}
// Ch6 - Part 3
fn main() {
    // let move_to = Move::Left;
    let move_to = Move::Right(10);

    // match expression
    // expression can beany value not needed to be only boolean
    match move_to {
        // Arm => (pattern => code)
        Move::Up => println!("Move Up"),
        Move::Down => println!("Move Down"),
        Move::Left => println!("Move Left"),
        Move::Right(x) => println!("Move Right {} times", x),
    }
}

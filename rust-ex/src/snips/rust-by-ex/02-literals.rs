#![allow(warnings)]  // NOT RECOMMENDED

fn main() {
    //~ Scalar types
    // signed integers (i8, i16, i32, i64, isize)
    // unsigned integers (u8, u16, u32, u64, usize)
    // floating point numbers (f32, f64)
    // char Unicode scalar values e.g. 'a'
    // bool Boolean values e.g. true or false

    //~ Compound types
    // tuples (T, U) e.g. (15, true)
    // arrays [T; n] e.g. [i32; 5] or [1, 2, 3]

    let is_morning = false;

    let salary = 100.00;
    let age = 32;
    let age: i32 = 32;
    let mut age = 32i32;
    age = 45;

    dbg!(10 + 20);
    dbg!(10u32 + 20);
    dbg!(10i32 - 20);

    dbg!(true && false);
    dbg!(true || false);
    dbg!(!false);

    dbg!(0b0101 & 0b0011); // bitwise AND
    dbg!(0b0101 | 0b0011); // bitwise OR
    dbg!(0b0101 ^ 0b0011); // bitwise XOR
    dbg!(2 << 5); // bitwise left shift
    dbg!(2 >> 5); // bitwise right shift

    dbg!(5_000_000);
}
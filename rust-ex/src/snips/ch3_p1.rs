#![allow(warnings)]

use colour::*;

// Chapter 3 -- part 1
// variables, constant, scalar types, tuple, arrays
fn main() {
    let first_name = "Proful";
    let mut last_name = "Sadangi";
    // first_name = "Kenny";
    last_name = "Kumar";
    // type must
    // can't add mut
    // can't assign result of fn
    const CITY: &str = "Jeypore";

    // ~ 4 primary scalar types +> integer, foat, boolen, char
    // ~ integer => i8/u8, i16/u16, i64/u64, i128/u128, isize/usize (arch - 64bit/32bit)
    // ~ u8 => 0 to 2^8 (255)
    // let mut age: u8 = 500;

    // float => f32/f64
    let discount = 5.5;

    // let isOlder = false;
    let is_older = false;

    let x = 'x';
    let y = '😍';
    // let z = '❤️';

    // ! Compound Types
    // ~ Tuple
    // ellments different types
    let tup = ("yt", "sub", 2000);
    let (medium, kind, count) = tup;
    magenta_ln!("[main] medium: {} kind: {} count: {}", medium, kind, count);
    yellow_ln!("[main] medium: {} kind: {} count: {}", tup.0, tup.1, tup.2);

    // ~ Array
    // elements same types & fixed length
    // data allocated in stack not heap
    // Vector => allowed to grow & shrink
    let numbers = [1, 2, 3];
    let fives = [5; 10];
    cyan_ln!("[main] numbers(1st): {}", numbers[0]);
    cyan_ln!("[main] fives(last): {}", fives[11]);
}

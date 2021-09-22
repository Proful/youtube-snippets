#![allow(warnings)]

use colour::*;

// Chapter 3 -- part 2
fn main() {
    // say_hi(500);
    // let salary = get_salary();
    // magenta_ln!("[main] salary: {}", salary);

    let salary = 5000;

    if salary < 10000 {
        magenta_ln!("[main] You are underpaid");
    } else if 10000 < salary && salary < 50000 {
        yellow_ln!("[main] You are doing great")
    } else {
        red_ln!("[main] You are rocking!")
    }

    if salary == 5000 {
        cyan_ln!("[main] salary is equal")
    }
}

// no default params
// no fn overloading
fn say_hi(age: i32) {
    cyan_ln!("[say_hi] Hi!!");
    cyan_ln!("[say_hi] Hi Alien!! You're {} year old.", { age });

    let y;
    let x = y = 5;
    // green_ln!("[say_hi] x: {} y: {}", x, y);
    // ; => removing it expression
    // ; => adding it became statement
    let x = { y + 5 };
    green_ln!("[say_hi] x: {} y: {}", x, y);
}

fn get_salary() -> f32 {
    5000.00
}

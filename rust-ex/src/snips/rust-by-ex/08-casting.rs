#![allow(warnings)]  // NOT RECOMMENDED

fn main() {
    //~ Casting
    let salary = 40.50;
    let sal = salary;
    let sal: u8 = salary as u8;

    let char_a = sal as char;
    // let char_b = salary as char;

    dbg!(salary, sal, char_a);
}
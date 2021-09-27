#![allow(warnings)]

use colour::*;

// chapter 4 -- part 3
// fn & ownership
fn main() {
    let name = String::from("Proful");
    transfer_ownership(name);
    // red_ln!("[main] name = {}", name);

    let age = 28;
    simple_copy(age);
    blue_ln!("[main] age = {}", age);

    let user_name = gives_ownership();
    magenta_ln!("[main] user_name = {}", user_name);
}

fn transfer_ownership(emp_name: String) {
    cyan_ln!("[transfer_ownership] emp_name = {}", emp_name);
}

fn simple_copy(emp_age: i32) {
    yellow_ln!("[simple_copy] emp_age = {}", emp_age);
}

fn gives_ownership() -> String {
    let name = String::from("Proful");
    name
}

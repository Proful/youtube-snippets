#![allow(warnings)]

use colour::*;

// chapter 4 -- part 4
fn main() {
    let name = String::from("Proful");
    // to pass a reference we have to user &name
    pass_reference(&name);
    cyan_ln!("[main] name = {}", name);
    // change_name(name);
    let mut name = String::from("Proful");
    change_name(&mut name);
    cyan_ln!("[main] name = {}", name);
    let r1 = &mut name;
    // let r2 = &mut name;
    r1.push_str(" it's interesting!");
    // cyan_ln!("[main] name = {}", name);
    cyan_ln!("[main] r1 = {}", r1);

    let mut name = String::from("Proful");
    let r1 = &name;
    let r2 = &name;
    // let r3 = &mut name;
    // r1.push_str(" very good");
    cyan_ln!("[main] r1 = {},  r2 = {}", r1, r2);
    let r3 = &mut name;
    cyan_ln!("[main] r3 = {}", r3);
}

// to pass reference we have to use &String in the method parameters
// employ_name contains the pointer to the name
fn pass_reference(employ_name: &String) {
    cyan_ln!("[pass_reference] employ_name = {}", employ_name);
}

// fn change_name(employ_name: &String) {
//     employ_name = "it taste";
// }
fn change_name(employ_name: &mut String) {
    employ_name.push_str("I'm happy");
}

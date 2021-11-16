#![allow(warnings)]

use std::cell::RefCell;

//^ Smart Pointers
// RefCell & Interior Mutability Pattern
// rs_ch15_p6_smart_ptr
fn main() {
    let name = String::from("Proful");
    // let full_name = name;
    // full_name.push_str(" Kumar");

    let full_name = RefCell::new(name);
    full_name.borrow_mut().push_str(" Kumar");

    // dbg!(full_name);

    let mut emp1 = full_name.borrow_mut();
    let mut emp2 = full_name.borrow_mut();

    emp1.push_str(" K");
    emp2.push_str(" S");
}

// Enforcing borrowing rules at runtime
// used for single threaded scenario
//

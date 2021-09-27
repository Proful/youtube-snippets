#![allow(warnings)]

use colour::*;

// chapter 4 -- part 2
fn main() {
    let x = 10;
    let y = x; // copy value

    magenta_ln!("[main] x = {}, y = {}", x, y);

    // all scalar types are Copy trait
    // integers, bool, floating points, char, tuples with scalar types only

    let tup_a = ('a', 500);
    let tup_b = tup_a;

    yellow_ln!("[main->tuple] tup_a = ({}, {})", tup_a.0, tup_a.1);
    green_ln!("[main->tuple] tup_b = ({}, {})", tup_b.0, tup_b.1);

    let tup_c = ("xyz", 500);
    // let tup_c = (String::from("abc"), 500);
    let tup_d = tup_c;

    yellow_ln!("[main->tuple] tup_c = ({}, {})", tup_c.0, tup_c.1);
    green_ln!("[main->tuple] tup_d = ({}, {})", tup_d.0, tup_d.1);

    //in stack, name => ptr | len(6) | capacccity(6)
    // in heap, data => "Proful"
    let name = String::from("Proful");

    // in stack, emp_name => ptr | len(6) | capacccity(6)
    // copied from name
    // But the actual data not copied

    //^ it's a "move" not shallow copy
    // invalidates name after move
    // no deep copy in rust
    let emp_name = name;

    //@ double free error
    // when name & emp_name both goes out of scope, both tries to free up memory
    // cyan_ln!("[main] name: {}", name);

    // deep clone
    let user_name = emp_name.clone();
    cyan_ln!("[main] emp_name: {} user_name: {}", emp_name, user_name);
}

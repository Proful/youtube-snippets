#![allow(warnings)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}
// enum List {
//   Cons(i32, List),
//   Nil,
// }

use crate::List::{Cons, Nil};

//^ Smart Pointers
// rs_ch15_p1_smart_ptr
fn main() {
    let x = 25;
    let age = Box::new(x);
    dbg!(age);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); //dereference

    let x = 5;
    let y = Box::new(x); // copied value of x

    assert_eq!(5, x);
    assert_eq!(5, *y);

        // Recursive types
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

#![allow(warnings)]

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

//^ Smart Pointers
// Rc<T>
// rs_ch15_p5_smart_ptr
fn main() {
    // single value may have multiple owners
    // Rc<T> -- Reference counting
    // only used in single threaded scenario
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

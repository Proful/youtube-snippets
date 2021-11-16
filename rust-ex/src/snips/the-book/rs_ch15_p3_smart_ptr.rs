#![allow(warnings)]

use std::{fmt::Display, ops::Deref};

#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox {}", self.0);
    }
}

//^ Smart Pointers
// rs_ch15_p3_smart_ptr
fn main() {
    // Implicit deref coercion
    // applied to arguments to fn & methods
    // deref coercion works only on types that implement deref traits
    // convert one type to a ref to another type
    // &String -> &str
    let name = MyBox::new(String::from("Proful"));

    //@ DEREF
    println!("{}", name.0);
    println!("{}", *name);

    // &MyBox<String> -> &String -> &str
    hello(&name);

    // *name => MyBox<String> -> String
    // &(String)[..] -> &String[..] -> &str
    hello(&(*name)[..]);

    //@ DROP
    let name1 = MyBox::new(String::from("Kenny"));
    let name2 = MyBox::new(String::from("Steve"));

    // name.drop();
    drop(name);

    // it all done at compile time, no runtime cost

    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
    // mut ref to immutable ref possible but not vice versa
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

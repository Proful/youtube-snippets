#![allow(warnings)]

// https://www.youtube.com/watch?v=dHkzSZnYXmk&t=1390s
//^ Chapter 13 - Part 5 (closure)
// rs_ch13_p5_closure
fn main() {
    // the below is correct
    let f1 = |x| 0;
    f1(5); // if u call in same scope type is not needed
    f1(20);
    // f1(34.55);

    let name = String::from("Proful");
    let f2 = |x: i32| {
        // &self => Fn
        dbg!(&name);
        0
    };
    dbg!(&name);

    let name = String::from("Proful");
    let f2 = move |x: i32| {
        // self => FnOnce
        dbg!(&name);
        0
    };
    // dbg!(&name); // Error

    let mut name = String::new();
    let mut f3 = |x: i32| {
        // mutable reference
        // &mut self => FnMut
        name.push_str("hi");
        0
    };
    f3(34);
    dbg!(&name);

    let name = String::new();
    let f4 = |x: i32| {
        // ownership transfered
        // self => FnOnce
        drop(name);
        0
    };
    f4(33);
    // dbg!(&name);
    // call_me(f3);
}

#![allow(warnings)]

// https://www.youtube.com/watch?v=dHkzSZnYXmk&t=1390s
//^ Chapter 13 - Part 4 (closure)
// rs_ch13_p4_closure
fn main() {
    // x is a Fn Item
    // 0 sized value -> doesn't hold a pointer at all
    // Fn Item is a identifier that compiler identifies this unique type of function
    let x = hello;
    dbg!(std::mem::size_of_val(&x));

    // associated with type i32
    let y = hi::<i32>;

    // Fn Item -> (co-ercible) to a Fn pointer
    call_me(get_age);
}

fn hello() {}
fn hi<T>() {}
fn get_age(id: i32) -> i32 {
    28
}

// f is a fn pointer
// it is not zero sized, it is a pointer valued size e.g., 8
// std::mem::size_of_val(&f)
fn call_me(f: fn(i32) -> i32) {
    dbg!(std::mem::size_of_val(&f));
}
// fn item & fn pointer
// does not have any state, no memory referenced, no lifetime associated
// fn pointer implements all 3 traits

#![allow(warnings)]

//^ Chapter 13 - Part 1 (closure)
// rs_ch13_p1_closure
fn main() {
    // Non capturing closure
    // let adder = |num1: i32, num2: i32| {
    let adder = |num1, num2| {
        dbg!(num1, num2);
        num1 + num2
    };
    dbg!(adder(5, 6));
    // dbg!(adder(5.0, 6.0));

    let multiplier = 5;
    // Environment capturing closure
    let mul = |num: i32| num * multiplier;

    dbg!(mul(10));
    // call_me(mul);

    pass_mul(mul);
}

fn pass_mul(multiply: impl Fn(i32) -> i32) {
    dbg!(multiply(20));
}

// fn call_me(f: fn(i32) -> i32) {
//     dbg!(std::mem::size_of_val(&f));
// }
// - closure is an implementation details

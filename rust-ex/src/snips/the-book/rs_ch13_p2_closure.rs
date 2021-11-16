#![allow(warnings)]

//^ Chapter 13 - Part 2 (closure)
// rs_ch13_p2_closure
fn main() {
    let x = vec![1, 2, 3];
    let equal_to = |z| z == x; // use `move`

    dbg!(&x);

    let y = vec![1, 2, 3];
    dbg!(equal_to(y));
}

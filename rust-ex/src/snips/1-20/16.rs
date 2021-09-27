#![allow(warnings)]

use colour::*;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
fn main() {
    // variable binding
    let age; // declare
    age = 28; // assign

    let salary = 500;

    let sub: i32 = 2000;

    // throwaway
    let _ = 55;

    // variable shadowing
    let age = 30;
    let age = age + 5;

    // tuple => collection of values of different types
    let yt = ("sub", 2000);
    cyan_ln!("[main] 1st: {} 2nd: {}", yt.0, yt.1);

    let (kind, count) = ("sub", 2000);
    blue_ln!("[main] kind: {} count: {}", kind, count);

    {
        let age = 5;
        yellow_ln!("[main] block > age: {}", age);
    }

    let name = "Proful";
    let name = { "Proful" };

    let older = if age > 30 { "Older" } else { "Younger" };

    magenta_ln!("[main] if result {}", older);

    let younger = match age > 50 {
        true => "Older",
        false => "Younger",
    };
    green_ln!("[main] match result {}", younger);
    cyan_ln!("[main] younger length {}", younger.len());
    // str is primitive type, in scope by default
    blue_ln!("[main] younger length {}", str::len(younger));

    let p1 = Point { x: 1.0, y: 2.0 };
    // rest of the fields from another syntax
    // struct update syntax
    let p2 = Point { x: 11.0, ..p1 };
    yellow_ln!("[main] p1: {:#?}", p1);
    yellow_ln!("[main] p2: {:#?}", p2);

    let Point { x, y } = p2;
    magenta_ln!("[main] x: {} y: {}", x, y);
}

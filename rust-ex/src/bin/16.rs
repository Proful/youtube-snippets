#![allow(warnings)]

use colour::*;

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
}

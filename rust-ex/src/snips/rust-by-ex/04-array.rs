#![allow(warnings)]  // NOT RECOMMENDED

use std::mem;

fn main() {
    //~ Arrays
    //- stored in contiguous memory
    //- [] or [T; length]
    //- length is known at compile time
    //~ Slices
    //- length is not known at compile time
    //- two words: pointer and length

    let a = [1, 2, 3, 4, 5];
    dbg!(a);

    let b = [55; 20];
    // dbg!(b);
    // dbg!(b.len(), b[0], b[4]);
    dbg!(mem::size_of_val(&b));
    check_slice(&b);
}

fn check_slice(arr: &[i32]) {
    // dbg!(arr);
    dbg!(arr.len(), arr[0], arr[4]);
}
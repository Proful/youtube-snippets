#![allow(warnings)]

// Ch 9 - part 1
fn main() {
    //@ Unrecoverable errors
    // print failure msg, unwind & clean up stack & quit!
    // panic!("I'm dead!");
    let numbers = vec![5, 10, 15];

    // security vulnearabilities
    dbg!(numbers[55]);
}

//^ Error Handling
// Recoverable -> webservices call failed, Result<T, E.
// Unrecoverable -> symptoms of bug, array out of bounds error, panic!
// No exception in Rust

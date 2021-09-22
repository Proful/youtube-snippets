#![allow(warnings)]

use colour::*;

// chapter 4 -- part 1
fn main() {
    // string literals => immutable
    // we know contents at compile time
    // fast & efficient
    let name = "Proful";

    // ~ String
    // heap, store unknown size at compile time
    // mutable, growin piece of text
    // memory automatically returned or freed one's the variable goes out of scope
    let name = String::from("Proful"); // request the memory it needs
    let mut name = String::from("Proful");
    name.push_str(" Kumar");
    cyan_ln!("[main] name: {}", name);
}

// ~ stack
// stores the values in the order it gets them (pushing onto the stack)
// removes the values in the opposite order (popping off the stack)
// LIFO => last in first out
// all data stored in stack must have fixed size

// ~ heap
// data of unknown size must stored un heap
// less organized,
// memory allocator finds an empty space in heap that is big enough
// marks it as being in use & returns a pointer (address)
// pointer it self can be stored in stack

// ~ Ownership Rule
// 1. each value has a variable that's called it's owner
// 2. one owner at a time
// 3. when owner goes oot of scope, the value will be dropped

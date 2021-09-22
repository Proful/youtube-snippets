#![allow(warnings)]

use colour::{cyan_ln, magenta_ln, yellow_ln};

// only fields here, no method
#[derive(Debug)]
struct User {
    name: String,
    age: i64,
}

// u can write methods that connect to User struts
impl User {
    // kind of constructor
    // new is a convention not rule
    fn new(name: String, age: i64) -> User {
        User { name, age }
    }

    // getter method
    // no implicit this in rust
    // i64 no heap allocation, lives on stack, these are simple types
    // store in a h/w cpu register while processing
    // move to other register if needed => copying
    fn age(&self) -> i64 {
        self.age
    }

    // data stored in heap
    // if we allow naively copy ref, then two pointers refer to same copy
    //  two owners points to same memory
    // we need a different way to create a copy of data not ref
    fn name(&self) -> String {
        // self.name
        // two owner with two identical data
        self.name.clone() // deep copy
    }
}
fn main() {
    // no need to type name: and age: (auto inferred)
    // stack allocated
    // new doesn't imply memory allocation
    let user = User::new(String::from("Proful"), 30);

    cyan_ln!("[main] {:#?}", user);
    magenta_ln!("[main] name is {}", user.name);
    magenta_ln!("[main] age is via getter {}", user.age());
    yellow_ln!("[main] name is via getter {}", user.name());
}

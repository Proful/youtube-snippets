#![allow(warnings)]

use colour::{cyan_ln, magenta_ln, yellow_ln};
#[derive(Debug)]
struct User {
    name: String,
    age: i64,
}
impl User {
    // the below new fn get ownership of name & age
    // it is responsible for destroying or passing
    fn new(name: String, age: i64) -> User {
        yellow_ln!("[new] name: {}", name);
        // at this point ownership transfered to User
        let user = User { name, age };
        // println!("[new] name: {}", name); // ! err
        // new method doesn't own name anymore

        yellow_ln!("[new] age: {}", age);
        return user;
    }

    // fn new(name: String, age: i64) -> User {
    //     // memory allocated for name will be cleaned up
    // }

    // &self is not ownership, that means after end of this fn not deleting memory
    // &self => borrowing, temporary ownership of the object
    // at the end of fn execution release ownership rather than deleting
    // can't change value inside it
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let name = String::from("Proful");
    let age = 30;
    let user = User::new(name, age);

    // println!("[main] name: {}", name); // ! err
    magenta_ln!("[main] age: {}", age);

    cyan_ln!("[main] user > name: {}", user.name);
    cyan_ln!("[main] user > name: {}", user.name());
}

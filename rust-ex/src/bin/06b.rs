#![allow(warnings)]

use colour::{blue_ln, cyan_ln, yellow_ln};
#[derive(Debug)]
struct User {
    name: String,
    age: i64,
}
impl User {
    fn new(name: String, age: i64) -> User {
        User { name, age }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

fn transfer_ownership(user: User) {
    yellow_ln!("[transfer_ownership] name: {}", user.name())
}

fn borrow_ownership(user: &User) {
    blue_ln!("[borrow_ownership] name: {}", user.name())
}

fn main() {
    let name = String::from("Proful");
    let age = 30;
    let user = User::new(name, age);

    // responsibility transfered to the fn
    // transfer_ownership(user);

    // responsibility lies with main fn
    borrow_ownership(&user);

    cyan_ln!("[main] user > name: {}", user.name);
    cyan_ln!("[main] user > name: {}", user.name());
}

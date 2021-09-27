#![allow(warnings)]

use colour::cyan_ln;
#[derive(Debug)]
struct User {
    name: String,
    age: i64,
}
impl User {
    fn new(name: String, age: i64) -> User {
        User { name, age }
    }

    // fn name(&self) -> String {
    //     self.name.clone()
    // }

    // &str => owned by someoe else
    // only one copy at the heap
    // slightly more efficient
    fn name(&self) -> &str {
        // no need to clone any more
        &(self.name)
        // &self.name // same as above
    }
}

fn main() {
    let user = User::new(String::from("Proful"), 30);

    cyan_ln!("[main] Name {}", user.name());
}

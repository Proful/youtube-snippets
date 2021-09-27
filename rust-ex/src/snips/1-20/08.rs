#![allow(warnings)]

use colour::{cyan_ln, yellow_ln};
#[derive(Debug)]
struct User {
    name: String,
    age: i64,
}
impl User {
    fn new(name: String, age: i64) -> User {
        User { name, age }
    }
    fn name(&self) -> &str {
        &self.name
    }
}

// extension or adaptation through trait
trait AgeTrait {
    fn get_age(&self) -> i64;
}

impl AgeTrait for User {
    fn get_age(&self) -> i64 {
        self.age
    }
}
// fn use_age_trait(x: impl AgeTrait) {
//     println!("[use_age_trait] x age {}", x.get_age())
// }

// dyn AgeTrait => something that impl AgeTrait
fn use_age_trait(x: Box<dyn AgeTrait>) {
    yellow_ln!("[use_age_trait] x age {}", x.get_age());
    // println!("[use_age_trait] x name {}", x.name());
}

// During compi;e time
// fn use_age_trait(x: Employee) {
//     println!("[use_age_trait] x age {}", x.get_age())
// }

fn main() {
    let user = User::new(String::from("Proful"), 30);

    cyan_ln!("[main] Name {}", user.name);

    // use_age_trait(user);
    // use_age_trait(Box<user>);
    use_age_trait(Box::new(user));

    // let name1 = user.name();
    // let name2 = User::name(&user);
}

#![allow(warnings)]
#[derive(Debug)]
struct User {
    name: String,
    age: i64,
}
impl User {
    fn new(name: String, age: i64) -> User {
        User { name, age }
    }
}

trait AgeTrait {
    fn get_age(&self) -> i64;
}

impl AgeTrait for User {
    fn get_age(&self) -> i64 {
        self.age
    }
}
impl AgeTrait for i64 {
    fn get_age(&self) -> i64 {
        *self + 5
    }
}

fn use_age_trait(x: Box<dyn AgeTrait>) {
    println!("[use_age_trait] x age {}", x.get_age());
}

fn main() {
    let user = User::new(String::from("Proful"), 30);

    println!("[main] Name {}", user.name);

    use_age_trait(Box::new(user));
    use_age_trait(Box::new(50));

    let age = 60;
    print!("[main] age is {}", age.get_age());
}

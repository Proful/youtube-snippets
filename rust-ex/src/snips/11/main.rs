#![allow(warnings)]
// connect main.rs => user.rs
mod user;
use user::User;

mod agetrait;
// if traits not in scope user.get_age() won't work
// use agetrait::{use_age_trait};
use agetrait::{use_age_trait, AgeTrait};

fn main() {
    // user::User -> can be used instead of use
    let user = User::new(String::from("Proful"), 30);

    // println!("[main] Name {}", user.name);
    println!("[main] age {}", user.get_age());

    use_age_trait(Box::new(user));
}

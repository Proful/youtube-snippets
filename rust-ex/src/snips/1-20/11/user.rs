#![allow(warnings)]
// no need of mod
// slightly different syntax, need to add crate::
use crate::agetrait::AgeTrait;

pub struct User {
    name: String,
    pub age: i64,
}
impl User {
    pub fn new(name: String, age: i64) -> User {
        User { name, age }
    }
}

impl AgeTrait for User {
    fn get_age(&self) -> i64 {
        self.age
    }
}

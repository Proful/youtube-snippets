#![allow(warnings)]
pub trait AgeTrait {
    fn get_age(&self) -> i64;
}

pub fn use_age_trait(x: Box<dyn AgeTrait>) {
    println!("[use_age_trait] x age {}", x.get_age());
}

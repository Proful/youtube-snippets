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

fn use_age_trait(x: Box<dyn AgeTrait>) {
    println!("[use_age_trait] x age {}", x.get_age());
}

struct Student {
    name: String,
    stu_age: i64,
}
impl AgeTrait for Student {
    fn get_age(&self) -> i64 {
        self.stu_age
    }
}
fn main() {
    let user = User::new(String::from("Proful"), 30);
    // student stored in stack
    let student = Student {
        name: "Kenny".to_string(),
        stu_age: 22,
    };

    println!("[main] Name {}", user.name);

    // Different way to think about polymorphism
    // There is no relationship between user & student
    // Only similarity between them is that both impl AgeStrait
    use_age_trait(Box::new(user));
    // Box allows to copy from stack to heap
    use_age_trait(Box::new(student));
}

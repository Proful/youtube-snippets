#![allow(warnings)]

// https://www.youtube.com/watch?v=XQ0N8-3i0bA&t=488s
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

#[derive(Debug)]
struct Employee<'a> {
    first_name: &'a str, // string slice, a reference
}

impl<'a> Employee<'a> {
    // fn given_name(&self, given_name: &str) -> &str {
    //     if given_name.is_empty() {
    //         return self.first_name;
    //     }
    //     "random_name"
    // }
    fn given_name<'b>(&'b self, given_name: &'b str) -> &'b str {
        if given_name.is_empty() {
            return self.first_name;
        }
        given_name
    }
}
//^ lifetime - Chapter 10 (part-4c)
fn main() {
    let first_name = String::from("Proful");
    let emp = Employee {
        first_name: &first_name,
    };
    let given_name = String::from("Mr Proful");
    dbg!(&emp.given_name("&given_name"));
}

//^ static
//- entire duration of program

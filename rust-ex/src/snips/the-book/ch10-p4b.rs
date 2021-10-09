#![allow(warnings)]

// https://www.youtube.com/watch?v=XQ0N8-3i0bA&t=488s
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

#[derive(Debug)]
struct Employee<'a> {
    first_name: &'a str, // string slice, a reference
}

//^ lifetime - Chapter 10 (part-4b)
fn main() {
    let full_name = String::from("Proful Sadangi");
    let first_name = &full_name[..6];
    dbg!(&first_name);

    let emp = Employee {
        first_name: first_name,
    };
    dbg!(&emp);

    // let emp2;
    // {
    //     let full_name = String::from("Proful Sadangi");
    //     let first_name = &full_name[..6];
    //     emp2 = Employee {
    //         first_name: first_name,
    //     };
    // }
    // dbg!(&emp2);

    let emp3;
    {
        let first_name = "Proful";
        emp3 = Employee {
            first_name: first_name,
        };
    }
    dbg!(&emp3);
}

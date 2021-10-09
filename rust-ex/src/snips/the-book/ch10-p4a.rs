#![allow(warnings)]

// https://www.youtube.com/watch?v=XQ0N8-3i0bA&t=488s
// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

//^ lifetime - Chapter 10 (part-4a)
fn main() {
    //lifetime-start 'a
    let r;

    {
        // 'b -- start
        let x = 5;
        r = &x;
        println!("r: {}", r);
        // let mut x = 5;
        // r = &mut x;
        // *r = *r + 15; // How to add 5 to the value
        // println!("r: {}", r);
        // 'b -- end
    }
    // println!("r: {}", r);

    // It works!
    // let str1 = String::from("Hello there!");
    // let str2 = String::from("Proful`");

    // let result = largest(str1, str2);
    // dbg!(result);
    // dbg!(str1);

    // let str1 = "abcde";
    // let str2 = "xyz";

    // let result = largest(str1, str2);
    // dbg!(result);

    let str1 = "abc";
    let result;
    {
        let str2 = "xyzxyz";

        result = largest(str1, str2);
    }
    dbg!(result);

    // let str1 = String::from("abc");
    // let result;
    // {
    //     let str2 = String::from("xyzxyz");
    //     let str3 = &str2[..];

    //     result = largest(str1.as_str(), str3);
    //     result = largest(str1.as_str(), str2.as_str());
    //     result = largest(&str1, &str2);
    // }
    // dbg!(result);

    //lifetime-ends 'a
}

// String checker
// fn largest(x: String, y: String) -> String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// &str string slices or references
// borrow checker should reject any values that don't adhere to these lifetime constraint
fn largest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn largest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
// fn largest<'a>(x: &'a str, y: &str) -> &'a str {
//     let z = String::from("abc");
//     // dangling reference
//     z.as_str()
// }
//- every reference has a lifetime
//- lifetime is the scope for which that reference is valid
//- most of time lifetime => implicit & inferred

//^ Borrow Checker
// Rust Compiler has borrow checker which compares scopes to determine
// that all borrows are valid

//^ Lifetime Annotation
// Doesn't affect how long a reference lived

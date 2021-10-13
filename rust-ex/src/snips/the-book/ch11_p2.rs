#![allow(warnings)]

pub fn say_hi(name: &str) -> String {
    // dbg!(name);
    format!("Hi {}", name)
}

#[test]
//~ cargo test -- --ignored
#[ignore = "it's failing a lot"]
fn say_hi_contains_name() {
    let name = "Proful";
    assert!(say_hi(name).contains(name));
    assert!(
        say_hi("Kenny").contains(name),
        "Message doesn't contained desired value i.e., '{}'",
        name
    );
}

fn divide_by_num(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

#[test]
// #[should_panic]
#[should_panic(expected = "attempt to divide by zero")]
fn test_divide_by_num() {
    assert_eq!(divide_by_num(6, 3), 2);
    assert_eq!(divide_by_num(6, 0), 2);
}

//~ cargo test -- --test-threads=1

// not run when u do cargo build or compiling code
#[cfg(test)]
mod tests {}

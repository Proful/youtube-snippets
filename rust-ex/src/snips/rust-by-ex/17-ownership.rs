#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let x = 45;
    let y = x;

    println!("x = {}", x);
    println!("y = {}", y);

    let a = Box::new(67);
    let b = a;

    let mut z = *b;
    z = 55;
    println!("z = {}", z);

    // println!("a = {}", a);
    println!("b = {}", b);

    do_magic(b);
    // println!("b = {}", b);

    let name = String::from("John");
    let mut emp_name = name;
    emp_name.push_str(" Doe");

    println!("emp_name = {}", emp_name);
    // println!("name = {}", name);

    struct Employee {
        name: String,
        role: String,
    }

    let emp = Employee {
        name: String::from("John"),
        role: String::from("Manager"),
    };

    let Employee { name, ref role } = emp;

    println!("name = {}", name);
    println!("role = {}", role);
    // println!("emp name = {}", emp.name);
    println!("emp role = {}", emp.role);
}

fn do_magic(c: Box<i32>) {
    println!("{}", c);
}
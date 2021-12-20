#![allow(warnings)] // NOT RECOMMENDED

#[derive(Debug)]
struct Employee {
    name: String,
    age: u8,
    salary: u32,
}

#[derive(Debug)]
struct Pair(i32, bool);

#[derive(Debug)]
struct Empty;

fn main() {
    let emp = Employee {
        name: String::from("John"),
        age: 30,
        salary: 5000,
    };
    dbg!(&emp);
    dbg!(&emp.name);

    let pair = Pair(10, true);
    dbg!(&pair);
    dbg!(&pair.0);

    let empty = Empty;
    dbg!(empty);

    let Employee {
        name,
        age,
        salary,
    } = emp;
    dbg!(&name);

    let Pair(uid, is_valid) = pair;
    dbg!(&uid);
}
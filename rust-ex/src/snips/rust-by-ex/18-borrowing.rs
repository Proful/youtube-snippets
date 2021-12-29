#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let name = String::from("Proful");

    hello(&name);

    println!("name: {}", name);

    let mut name = String::from("Proful");

    append_last_name(&mut name);

    println!("name: {}", name);


    let name1 = &name;
    let name2 = &name;
    let name3 = &mut name;

    // println!("name1: {}", name1);
}

fn hello(name: &String) {
    println!("Hello, {}!", name);
}

fn append_last_name(name: &mut String) {
    name.push_str(" Sadangi");
}
#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let x = 5;

    {
        let y = &x;
        println!("{}", y);
    }

    {
        let z = &x;
        println!("{}", z);
    }

    println!("{}", x);

    let name = String::from("Proful");
    let role = String::from("Developer");

    print_name(&name);
    print_name_role(&name, &role);
}

fn print_name(name: &String) {
    println!("{}", name);
}

fn print_name_role<'a>(name: &'a String, role: &'a String) -> &'a String {
    println!("{} is a {}", name, role);
    name
}
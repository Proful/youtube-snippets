#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    //~ String
    //- stored as vector of bytes Vec<u8>
    //- UTF-8 encoding
    //- heap allocated, growable
    //~ &str
    //- slice &[u8]

    let name = "Proful";

    let mut s = String::new();
    s.push_str("Hello, ");
    s.push_str(name);

    println!("{}", s);

    let s1 = s.replace("Proful", "John");
    println!("{}", s1);
}

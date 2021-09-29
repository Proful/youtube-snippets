#![allow(warnings)]

use std::collections::HashMap;

// ch8 - part3
// ^ Hashmap
// - Creation of hash map type variables
// - Insert data into hash map
// - Ownership of key/value
// - Retrieve & process value based on key
// - Iterate over key/value pair
// - Updating data
fn main() {
    let mut name_age = HashMap::new();
    let first = String::from("Proful");
    let second = String::from("Kenny");
    name_age.insert(first, 30);
    name_age.insert(second, 22);

    dbg!(&name_age);
    // dbg!(first);

    let first_key = String::from("Proful1");
    let first_val = name_age.get(&first_key);

    match first_val {
        Some(val) => println!("val = {}", val),
        None => println!("Key not found"),
    }

    for (key, val) in &name_age {
        println!("({}, {})", key, val);
    }

    // updating by overwriting
    let first = String::from("Proful");
    name_age.insert(first, 500);
    dbg!(&name_age);

    let first = String::from("Proful");
    name_age.entry(first).or_insert(300);

    let third = String::from("Steve");
    name_age.entry(third).or_insert(60);
    dbg!(&name_age);


}

// HashMap<K, V>
// hashing fn => decides how it places keys & values into memory
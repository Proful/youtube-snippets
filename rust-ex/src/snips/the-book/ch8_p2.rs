#![allow(warnings)]

// ch8 - part2
// ^ String
// - 3 different ways to create String
// - Append String
// - String concatenation using +
// - format!
// - handling utf-8 (unicode) char
// - iterating over String, inspect chars & bytes
fn main() {
    let mut name = String::new();
    name.push('P');
    name.push('r');
    name.push('o');
    name.push('f');
    name.push('u');
    name.push('l');
    let city = "Jeypore".to_string();
    let role = String::from("Developer");
    dbg!(&name, &city, &role);

    name.push_str(" Sadangi");
    dbg!(&name);

    // + => fn add(self, &str) -> String {}
    // but &role is &String then how it is compatible with &str
    // copiler can coerce &String to &str
    // deref coercion: &role to &role[..]
    let msg = city + &role;
    dbg!(&msg);
    // dbg!(city); // city no longer available

    // doesn't take ownership
    let profile = format!("{} is a {}", name, role);
    dbg!(&profile);
    dbg!(&name, &role);

    // Rust string doesn't support indexing
    // dbg!(&name[0]);

    // String => wrapper of Vec<u8>
    dbg!(&name.len());
    let odia_name = "ପ୍ରଫୁଲ୍ଲ".to_string();
    dbg!(&odia_name);
    dbg!(&odia_name.len());
    dbg!(&odia_name[0..3]);

    for c in odia_name.chars() {
        dbg!(c);
    }
    let mut i = 0;
    for b in "ପ୍ରଫୁଲ୍ଲ".bytes() {
        i += 1;
        dbg!(b);
    }
    dbg!(i);
}

// collection of bytes
// Rust => one string type in core lang i.e., &str (string slices)
// &str => string literals stored in program's binary, UTF-8 encoded
// String => part of Rust standard library, not part core lang
// growable, mutable, owned, UTF-8 encoded string type
// OsString, OsStr, CString, CStr
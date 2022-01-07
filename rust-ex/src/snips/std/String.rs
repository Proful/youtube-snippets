#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let name = "Proful";
    let name = "Proful".to_string();
    let name = name.as_str();

    let name = String::from("Proful");
    let name: String = "Proful".into();
    let full_name = name + " Kumar";
    let full_name = format!("{} {}", "Proful", "Kumar");

    let heart = vec![240, 159, 146, 150];
    let heart = String::from_utf8(heart).unwrap();
    dbg!(&heart);
    dbg!(&heart.repeat(5));
    // dbg!(&heart.into_bytes()); // consumes the String
    // dbg!(&heart.as_bytes()); // borrows the String
    // dbg!(&heart);

    let mut name = String::from("Proful");
    name.push(' ');
    name.push_str("Kumar");
    dbg!(&name);
    // dbg!(&name[0]); // ERROR: due to UTF-8 constraints
    dbg!(&name[0..1]);

    // pointer => internal buffer String uses to store data
    let ptr = name.as_ptr();

    println!("len = {} capacity = {}", name.len(), name.capacity());

    let mut msg = String::new();
    let mut msg = String::with_capacity(25);

    dbg!(&msg.capacity());

    for _ in 0..5 {
        msg.push_str("hello");
        // dbg!(&msg.len());
        // dbg!(&msg.capacity());
    }

    let hello = msg.as_mut_str();
    hello.make_ascii_uppercase();
    dbg!(&hello);

    let mut msg = String::from("hello");
    dbg!(&msg.to_lowercase());
    dbg!(&msg.to_uppercase());

    let mut msg = String::from("Rust is awesome!");
    &msg.truncate(4);
    dbg!(&msg);

    dbg!(&msg.pop());
    dbg!(&msg.remove(1)); // requires copying every element in the buffer
    dbg!(&msg);

    let mut msg = String::from("Rust is a perfect programming language!");
    msg.retain(|c| c != ' ');
    dbg!(&msg);

    msg.insert(4, '_');
    msg.insert_str(7, " not ");
    dbg!(&msg);

    let name = String::from("Proful");
    dbg!(&name.len());
    dbg!(&name.chars().count());

    let name = String::from("ପ୍ରଫୁଲ୍ଲ");
    dbg!(&name.len());
    dbg!(&name.chars().count());



    let mut name = String::from("ProfulKumar");
    let last_name = name.split_off(6);

    dbg!(&name);
    dbg!(&last_name);
    let mut name = String::from("ProfulKumar");
    let last_name = name.split_at(6);

    dbg!(&name);
    dbg!(&last_name);

    name.clear();

    dbg!(&name.is_empty());

    let mut name = String::from("ProfulKumar");
    name.replace_range(6..name.len(), " Sadangi");
    dbg!(&name);
    dbg!(&name.replace("Proful", "John"));

    dbg!("abcxyabcxhabc".replacen("abc", "123", 10));

    dbg!(&name.get(..6));

    let name = name.into_boxed_str(); // drop any excess capacity

    let mut name = String::from("ProfulKumar");
    let mut chars = name.chars();

    dbg!(&chars.next());

    let mut char_indices = name.char_indices();

    dbg!(&char_indices.next());
    dbg!(&char_indices.next());

    let mut name = String::from("Proful Kumar");
    let mut words = name.split_whitespace();
    dbg!(&words.next());

    let text = String::from("Rust is a \nperfect \nprogramming language!");
    let mut lines = text.lines();

    dbg!(&lines.next());

    let text = String::from("London,Paris,Tokyo");
    let mut cities = text.split(",").collect::<Vec<_>>();
    dbg!(&cities);

    // split_terminator, rsplit_terminator, splitn, rsplitn, split_terminator_at, rsplit_terminator_at

    let mut name_numbers = String::from("a1b3c6h7");
    let mut numbers = name_numbers.matches(char::is_numeric).collect::<Vec<_>>();
    // dbg!(&numbers);
    let mut numbers = name_numbers.match_indices(char::is_numeric).collect::<Vec<_>>();
    // dbg!(&numbers);

    let text = String::from(" aa ");
    dbg!(&text.trim());
    dbg!(&text.trim_start());
    dbg!(&text.trim_end());

    let key_val = String::from("host=www.google.com");
    dbg!(&key_val.strip_prefix("host="));

}


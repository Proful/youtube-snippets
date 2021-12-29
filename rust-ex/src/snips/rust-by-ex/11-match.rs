#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 | 3 | 4 => println!("two, three or four"),
        5..=20 => println!("five to twenty"),
        _ => println!("something else"),
    }

    // let is_morning = false;

    // let msg = if is_morning {
    //     "Good morning"
    // } else {
    //     "Good night"
    // };

    // dbg!(msg);

    let coordinate = (1, 8, 5);

    match coordinate {
        (0, 0, 0) => println!("origin"),
        (0, 0, z) => println!("z = {}", z),
        (1, y, z) => println!("y = {} z = {}", y, z),
        (x, y, z) => println!("({}, {}, {})", x, y, z),
    }


    enum Color {
        Red,
        Green,
        Blue,
        RGB(u8, u8, u8),
    }

    let color = Color::RGB(255, 0, 0);

    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RGB(0, 0, 0) => println!("black"),
        Color::RGB(r, g, b) => println!("r = {}, g = {}, b = {}", r, g, b),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 10, y: 0 };

    match p {
        Point { x: 0, y: 0 } => println!("origin"),
        Point { x, y: y1 } => println!("({}, {})", x, y1),
    }

    let pair = (10, -10);

    match pair {
        (x, y) if x == y => println!("x == y: {}", x),
        (x, y) if x + y == 0 => println!("x + y == 0: {}", x),
        _ => println!("no match"),
    }

    match age() {
        0 => println!("infant"),
        n @ 1..=5 => println!("child {}", n),
        _ => println!("adult"),
    }
}

fn age() -> u8 {
    4
}
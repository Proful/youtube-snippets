#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let x = Some(5);

    match x {
        Some(y) => println!("Matched, y = {}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    if let Some(y) = x {
        println!("Matched, y = {}", y);
    } else {
        println!("Default case, x = {:?}", x);
    }

    enum Color {
        Red(u8),
        Green,
        Blue,
    }

    let color = Color::Red(255);

    if let Color::Red(val) = color {
        println!("Color is red {}", val);
    }

    let mut count = Some(0);

    while let Some(i) = count {
        if i > 10 {
            count = None;
        } else {
            println!("{}", i);
            count = Some(i + 1);
        }
    }

}
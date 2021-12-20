#![allow(warnings)] // NOT RECOMMENDED

// #[derive(Debug)]
// enum Color {
//     Red,
//     Green,
//     Blue,
// }
#[derive(Debug)]
enum Color {
    Red=0xff0000,
    Green=0x00ff00,
    Blue=0x0000ff,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

type Msg = Message;

fn main() {
    let color = Color::Red;
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
    }
    println!("{:06x}", color as u32);
    // let msg = Message::ChangeColor(Color::Blue);
    // let msg = Msg::Write(String::from("Hello, world!"));

    use Message::*;
    let msg = Write(String::from("Hello, world!"));

    match msg {
        Message::Quit => println!("The Quit message was sent!"),
        Message::Move { x, y } => {
            println!("x = {} and y = {}", x, y);
        }
        Message::Write(text) => println!("text: {}", text),
        Message::ChangeColor(color) => println!("color to: {:?}", color),
    }
}
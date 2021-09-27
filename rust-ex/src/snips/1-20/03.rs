#![allow(warnings)]

use colour::cyan_ln;
// enum Staus {
//     Open,
//     Close,
// }
// fn main() {
//     let status = Staus::Open;

//     match status {
//         Staus::Open => println!("Hey Open"),
//         Staus::Close => println!("Close"),
//     }
// }
enum Staus {
    Open(i64),
    Close,
}
fn main() {
    // payload can be passed in enum
    let status = Staus::Open(150);

    match status {
        // Staus::Open => println!("Open"),
        // _ ignore
        // Staus::Open(_) => println!("Open"),
        Staus::Open(x) => println!("Open {}", x),
        Staus::Close => println!("Close"),
    }
}

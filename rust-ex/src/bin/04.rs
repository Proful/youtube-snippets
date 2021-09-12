#![allow(warnings)]
use colour::*;
use std::error::Error;

// Box<dyn Error> => very generic error
// Box => on the heap
// dyn Error => something that implement Error Trait
// No exception in Rust, so this Result pattern used everywhere
fn fetch_data() -> Result<String, Box<dyn Error>> {
    Ok("Gold".to_string())
    // error
}

// fn main() {
//     let data = fetch_data();
//     // println!("Data is {}", data); // error

//     match data {
//         Ok(d) => {
//             cyan_ln!("[main] Data received {}", d);
//         }
//         Err(err) => {
//             red_ln!("Error");
//         }
//     }
// }
fn main() -> Result<(), Box<dyn Error>> {
    let data = fetch_data();

    // ? if it is success return the value
    // ? if it is error bubble up. Termiinate the execution
    cyan_ln!("Data is {}", data?);
    yellow_ln!("Data is {}", fetch_data()?);

    // () => empty type/value
    Ok(()) // return something
}

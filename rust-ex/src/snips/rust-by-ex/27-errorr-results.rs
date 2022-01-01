#![allow(warnings)] // NOT RECOMMENDED

// fn main() {
//     let x = "5".parse::<i32>();

//     match x {
//         Ok(n) => println!("x = {}", n),
//         Err(e) => println!("{}", e),
//     }

//     let y = "hello".parse::<i32>().unwrap();
//     println!("y = {}", y);
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), std::num::ParseIntError> {
    let x = "5aa".parse::<i32>();

    match x {
        Ok(n) => println!("x = {}", n),
        Err(e) => return Err(e),
    }

    // let y = "hello".parse::<i32>().unwrap();
    let y = "hello".parse::<i32>()?;
    println!("y = {}", y);

    Ok(())
}
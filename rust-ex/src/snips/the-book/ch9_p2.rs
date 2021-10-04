#![allow(warnings)]

use std::{error::Error, fs::{self, File}, io::{self, ErrorKind, Read}};

// Ch 9 - part 2
// fn main() {
    //@ Recoverable errors
    // let file = File::open("hello.txt");

    // match file {
    //     Ok(_) => println!("File found"),
    //     Err(e) => println!("Err = {}", e),
    // }
    // let file = match file {
    //     Ok(f) => f,
    //     Err(e) => panic!("Err = {}", e),
    // };
    // let file = match file {
    //     Ok(f) => f,
    //     Err(e) => {
    //         let err_kind = e.kind();
    //         match err_kind {
    //             ErrorKind::NotFound => match File::create("hello.txt") {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("Err = {}", e)
    //             },
    //             anything => panic!("Unknown Err! = {}", e)
    //         }
    //     },
    // };

    // let mut file = File::open("hello.txt").unwrap_or_else(|err| {
    //     if err.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|err| {
    //             panic!("Creation failed. Err = {}", err);
    //         })
    //     } else {
    //         panic!("File open failed. Err = {}", err);
    //     }
    // });
    // // dbg!(&file.metadata());
    // let mut contents = String::new();
    // file.read_to_string(&mut contents);
    // dbg!(&contents);

    // let file = File::open("hello.txt1").unwrap();
    // let file = File::open("hello.txt").expect("File open failed.");
    // dbg!(&file.metadata());

//     match read_from_file() {
//         Ok(data) => println!("data = {}", data),
//         Err(err) => println!("Err = {}", err),
//     }
// }

fn main() -> Result<(),  Box<dyn Error>>{
    dbg!(read_from_file()?);

    Ok(())
}
// Propagating errors
fn read_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
// fn read_from_file() -> Result<String, io::Error> {
//     let mut contents = String::new();
//     // - chaining
//     File::open("hello.txt")?.read_to_string(&mut contents)?; // ~ ? is imp

//     Ok(contents) //~ No ;
// }
// fn read_from_file() -> Result<String, io::Error> {
//     // value reurned
//     // if err, retrn error
//     let mut file = File::open("hello.txt")?; // ~ ? is imp

//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?; // ~ ? is imp

//     Ok(contents) //~ No ;
// }
// fn read_from_file() -> Result<String, io::Error> {
//     let file = File::open("hello.txt");

//     let mut file = match file {
//         Ok(f)=> f,
//         Err(err) => return Err(err),// here return is required
//     };

//     let mut contents = String::new();

//     match file.read_to_string(&mut contents) {
//         Ok(_) => Ok(contents),
//         Err(err) => Err(err), // skip return
//     }
// }


/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/
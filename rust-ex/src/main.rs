#![allow(warnings)]

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let data = read_from_file()?;
    println!("data = {}", data);

    Ok(())
}

fn read_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

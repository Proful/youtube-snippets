#![allow(warnings)] // NOT RECOMMENDED

use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let path = Path::new(".");

    dbg!(&path);
    dbg!(path.display());

    let new_path = path.join("src").join("lib.rs");
    dbg!(&new_path);
    dbg!(new_path.to_str());

    let hello_path = Path::new("hello.txt");

    let mut file = File::open(hello_path).expect("file not found");

    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("something went wrong reading the file");

    // dbg!(&contents);

    // let hi_path = Path::new("hi.txt");
    // let mut file = File::create(hi_path).expect("file not found");
    // file.write_all(&contents.as_bytes())
    //     .expect("something went wrong writing to the file");

    // BufReader::new(file).lines().for_each(|line| {
    //     println!("{}", line.unwrap());
    // });

    let lines = BufReader::new(file).lines();
    for line in lines {
        println!("{}", line.unwrap());
    }
}

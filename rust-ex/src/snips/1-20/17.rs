#![allow(warnings)]

use colour::*;
use rand::Rng;
use std::io;

fn main() {
    // String => type, new => Associate fn
    let mut name = String::new();

    // let result = io::stdin() // returns instance of std::io::Stdin
    //     .read_line(&mut name);

    // match result {
    //     Ok(b) => {
    //         green_ln!("[main] bytes reead {}", b);
    //         green_ln!("[main] name is {}", name);
    //     }
    //     Err(err) => {
    //         red_ln!("[main] err: {}", err);
    //     }
    // }

    loop {
        cyan_ln!("[main] Enter your name below 👇",);
        // & => reference
        // give access to one part of data to multiple pieces of code without copying
        io::stdin() // returns instance of std::io::Stdin
            .read_line(&mut name)
            .expect("Failed to read line");

        cyan_ln!("[main] Hi {}", name);

        green_ln!("[main] What's your age 👇",);
        let mut age_entered = String::new();
        io::stdin() // returns instance of std::io::Stdin
            .read_line(&mut age_entered)
            .expect("Failed to read line");

        // let age_entered = age_entered.trim().parse().expect("You must enter a number");
        let age_entered = match age_entered.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        let age = rand::thread_rng().gen_range(1..100);
        magenta_ln!("[main] I think your age is {}", age);

        match age.cmp(&age_entered) {
            std::cmp::Ordering::Less => yellow_ln!("[main] U'r younger"),
            std::cmp::Ordering::Greater => {
                yellow_ln!("[main] U'r much older");
                break;
            }
            std::cmp::Ordering::Equal => yellow_ln!("[main] U'r correct"),
        }
    }
}

#![allow(warnings)]
fn main() {
    // it can go wrong, bu in rust there is no exception
    // std::fs::write("hello.txt", "Hi from Proful");
    // () => empty tuple, unit kind of void
    // () => signifies it works! with no additional data
    let result = std::fs::write("hello.txt", "Hi from Proful");

    // std::fs::write("hello.txt", "Hi from Proful").unwrap();

    // pattern matching
    // kind of switch but better
    match result {
        Ok(()) => {
            println!("[main-match] File writing successful");
        }
        Err(e) => {
            // chmod 400 hello.txt
            println!("[main-match] File writing failed {}", e);
        }
    }

    let data = std::fs::read_to_string("hello.txt");

    match data {
        Ok(x) => {
            println!("[main-match] File reading successful");
            println!("[main-match] data {}", x);
        }
        Err(e) => {
            println!("[main-match] File reading failed {}", e);
        }
    }
}

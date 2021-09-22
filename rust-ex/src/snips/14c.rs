#![allow(warnings)]
fn read_data() -> Result<String, std::io::Error> {
    // ? Error bubble up to the caller
    Ok(std::fs::read_to_string("hello.txt")?)
}

fn main() {
    let data = read_data();
    match data {
        Ok(x) => {
            println!("[main] data: {}", x);
        }
        Err(e) => {
            println!("[main] err: {}", e);
        }
    }
}
// fn main() -> Result<(), std::io::Error> {
//     let data = read_data();
//     println!("[main] data: {}", data?);
//     Ok(())
// }

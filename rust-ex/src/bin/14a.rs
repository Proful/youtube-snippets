#![allow(warnings)]
fn read_data() {
    let file_result = std::fs::read_to_string("hello.txt");

    match file_result {
        Ok(x) => {
            println!("[read_data] File reading successful");
            println!("[read_data] data {}", x);
        }
        Err(e) => {
            println!("[read_data] File reading failed {}", e);
        }
    };
}

fn main() {
    read_data();
}

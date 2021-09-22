#![allow(warnings)]
fn read_data() -> Result<String, std::io::Error> {
    let file_result = std::fs::read_to_string("hello.txt");

    // match => expression not a statement
    let data = match file_result {
        Ok(x) => {
            println!("[read_data] File reading successful");
            println!("[read_data] data {}", x);
            x
        }
        Err(e) => {
            println!("[read_data] File reading failed {}", e);
            return Err(e);
        }
    };
    Ok(data)
}

fn main() {
    let data = read_data();
    match data {
        Ok(x) => {
            println!("[main] data {}", x);
        }
        Err(e) => {
            println!("[main] err {}", e);
        }
    }
}

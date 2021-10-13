use std::{env, error, fs, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    // let args = env::args();
    // dbg!(&args.inner);
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // ownership got transfered in for loop
    // for arg in args {
    //     dbg!(arg);
    // }
    //ownership of args not transfered in if condition
    if args.len() < 3 {
        panic!("You need to pass search string & file name");
    }
    let search_str = args.get(1);
    let search_str = match search_str {
        Some(x) => x,
        None => panic!("search_str shouldn't be empty"),
    };

    //search_str is already &String
    dbg!(search_str);
    // dbg!(search_str);

    let file_name = args.get(2);
    let file_name = match file_name {
        Some(x) => x,
        None => panic!("file_name shouldn't be empty"),
    };

    dbg!(file_name);

    let file_contents = read_from_file(file_name)?;
    dbg!(file_contents);

    Ok(())
}

fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

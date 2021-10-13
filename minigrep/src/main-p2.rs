use std::{env, error, fs, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let (search_str, file_name) = parse_args(&args);

    let file_contents = read_from_file(file_name)?;
    dbg!(search_str, file_name, file_contents);

    Ok(())
}

fn parse_args(args: &[String]) -> (&str, &str){
    if args.len() < 3 {
        panic!("You need to pass search string & file name");
    }
    let search_str = args.get(1);
    let search_str = match search_str {
        Some(x) => x,
        None => panic!("search_str shouldn't be empty"),
    };

    let file_name = args.get(2);
    let file_name = match file_name {
        Some(x) => x,
        None => panic!("file_name shouldn't be empty"),
    };
    (search_str, file_name)
}


fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

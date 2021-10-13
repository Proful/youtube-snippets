use std::{env, fs, io, process};

#[derive(Debug)]
struct Config {
    search_str: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("You need to pass search string & file name");
            return Err("You need to pass search string & file name");
        }
        let search_str = args.get(1);
        let search_str = match search_str {
            Some(x) => x,
            None => return Err("search_str shouldn't be empty"),
        };

        let file_name = args.get(2);
        let file_name = match file_name {
            Some(x) => x,
            None => return Err("file_name shouldn't be empty"),
        };
        // cloning bit easy but inefficent
        Ok(Config {
            search_str: search_str.clone(),
            file_name: file_name.clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args)?;
    let config = Config::new(&args).unwrap_or_else(|err| {
        dbg!(err);
        process::exit(1);
    });

    // let file_contents = read_from_file(&config.file_name)?;
    // dbg!(config, file_contents);

    // run(config).unwrap_or_else(|err| {
    //     dbg!(err);
    //     process::exit(1);
    // });
    if let Err(err) = run(config) {
        dbg!(err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), io::Error> {
    let file_contents = read_from_file(&config.file_name)?;
    dbg!(config, file_contents);
    Ok(())
}
fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

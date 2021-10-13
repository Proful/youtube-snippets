use std::{fs, io};

#[derive(Debug)]
pub struct Config {
    search_str: String,
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
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

pub fn run(config: Config) -> Result<(), io::Error> {
    let file_contents = read_from_file(&config.file_name)?;
    dbg!(&config, &file_contents);

    let result = search(&config.search_str, &file_contents);

    dbg!(&result);

    Ok(())
}

fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

fn search<'a>(search_str: &'a str, file_contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in file_contents.lines() {
        dbg!(line, search_str);
        if dbg!(line.contains(search_str)) {
            results.push(line);
        }
    }

    results
}

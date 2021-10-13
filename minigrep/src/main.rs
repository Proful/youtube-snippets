use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        dbg!(err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        dbg!(err);
        process::exit(1);
    }
}

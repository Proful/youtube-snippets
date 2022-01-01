#![allow(warnings)] // NOT RECOMMENDED

macro_rules! hello {
    () => {
        println!("Hello, friends!");
        println!("How are you?");
    }
}

macro_rules! create_fn {
    ($name:ident) => {
        fn $name() {
            println!("Creating function {}", stringify!($name));
        }
    }
}

macro_rules! print_expr {
    ($e:expr) => {
        println!("{} = {}", stringify!($e), $e);
    }
}

// block, expr, ident, item, stmt, tt, token, lit, meta, path, pat, path_segment

macro_rules! find_max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($y:expr),+) => {
        std::cmp::max($x, find_max!($($y),+))
    }
}

fn main() {
    hello!();

    create_fn!(hi);
    hi();

    print_expr!(10 + 20 );

    println!("{}", find_max!(1, 2, 3, 4, 5));
}
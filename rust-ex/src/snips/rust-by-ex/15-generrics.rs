#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    echo_back(1);
    echo_back(1.0);

    print(1);
    print(5.05);
    print("hello");

    dbg!(add(1, 2));
    dbg!(add(10.0, 20.0));

    debug_print("hi there");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };

    dbg!(p1);
    dbg!(p2);
}

fn echo_back<T>(x: T) -> T {
    x
}

fn print<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

// fn debug_print<T: std::fmt::Display + std::fmt::Debug>(x: T) {
//     println!("{:?}", x);
// }

fn debug_print<T>(x: T)
    where T: std::fmt::Display + std::fmt::Debug {
    println!("{:?}", x);
}
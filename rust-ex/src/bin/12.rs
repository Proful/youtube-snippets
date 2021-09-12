#![allow(warnings)]
fn main() {
    // std -> module
    // env -> environment modules inside std
    // args -> fn
    // Args -> iterator, allows to get elements one by one
    // let args = std::env::args();

    // we can skip elements using skip fn
    // let arguments = std::env::args().skip(1);
    let mut arguments = std::env::args().skip(1);

    // to access next element we need to add mut keyword
    // Option => way of marking string might be there or not there
    // let key = arguments.next();
    // if no string is there crash the program
    // There is no such thing as `null` in rust, there is `None`
    let key = arguments.next().unwrap();
    // let value = arguments.next().unwrap();
    let value = arguments.next().expect("value is required");

    println!("[main] key is {}", key);
    println!("[main] value is {}", value);
}

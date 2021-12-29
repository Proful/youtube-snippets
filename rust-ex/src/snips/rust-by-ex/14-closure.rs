#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let add = |x: i32, y: i32| -> i32 { x + y };
    let sub = |x, y| x-y;

    dbg!(add(1, 2));
    dbg!(sub(1, 2));

    let name = String::from("Proful");

    let hi = || println!("Hi, {}", name);
    //let hi = move || println!("Hi, {}", name);

    hi();
    dbg!(&name);

    let mut name = String::from("Proful");

    let mut hello = || {
        name.push_str(" Sadangi");
        println!("Hello, {}", name);
    };

    hello();
    dbg!(&name);

    //- Fn: the closure captures by reference (&T)
    //- FnMut: the closure captures by mutable reference (&mut T)
    //- FnOnce: the closure captures by value (T)

    let hi = || println!("Hi");
    fn hi_there() {
        println!("Hi there");
    }
    fn call_me(f: impl Fn()) {
        f();
    }
    call_me(hi);
    call_me(hi_there);

    dbg!(void());

    diverges();
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

fn void() -> (){
    println!("This function returns nothing");
    ()
}
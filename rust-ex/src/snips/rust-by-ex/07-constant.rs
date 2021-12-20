#![allow(warnings)] // NOT RECOMMENDED

static mut LANGUAGE: &str = "English";
const MAX_COUNTS: i32 = 100;

fn main() {
    // MAX_COUNTS = 200;
    unsafe { // NOT RECOMMENDED
        LANGUAGE = "French";
        dbg!(LANGUAGE);
    }
    dbg!(MAX_COUNTS);

    let age = 10;
    let mut age = 20;
    age = 30;
    age += 1;
    dbg!(age);

    let _unused_variable = 3;

    let x = 5;
    {
        let y = 10;
        dbg!(x + y);

        let x = 67;
        dbg!(x);
    }
    // dbg!(y);
    dbg!(x);

    let z;
    // dbg!(z);
    z = 5;
    dbg!(z);
}
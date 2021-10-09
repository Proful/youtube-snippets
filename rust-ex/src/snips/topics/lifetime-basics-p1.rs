#![allow(warnings)]
// https://www.youtube.com/watch?v=Kde5Y424azM

//^ lifetime - part 1
fn main() {
    dbg!(capitalize("Hi"));

    let c;
    let a = "abc";
    {
        let b = "xyz".to_string();
        c = concat1("fdf", &b);
        // c = concat1(&b, "xxxx");
    }
    dbg!(a, c);

    let c;
    let a = "abc";
    {
        let b = "xyz";
        c = concat("", b);
    }
    // dbg!(a, b, c);
    dbg!(a, c);

    // let c;
    // let a = "abc";
    // {
    //     let b = String::from("xyz");
    //     c = concat("", &b);
    // }
    // dbg!(a, c);

    let c;
    let a = "abc";
    let b = String::from("xyz");
    {
        c = concat("", &b);
    }
    dbg!(a, c);
}

// No lifetime involved bcoz only one input ref & only one output ref
// fn capitalize(msg: &str) -> &str {
//     msg
// }
fn capitalize<'a>(msg: &'a str) -> &'a str {
    msg
}

// Bcoz not reference, no lifetime needed
fn join(msg: String, tmp: String) -> String {
    msg
}

// Two input param - lifetime needed
// fn concat(msg: &str, tmp: &str) -> &str {
fn concat1<'a>(msg: &'a str, tmp: &str) -> &'a str {
    msg
}
// fn concat<'a, 'b>(msg: &'a str, tmp: &'b str) -> &'b str {
//     tmp
// }
// fn concat<'a, 'b>(msg: &'a str, tmp: &'b str) -> &'b str {
//     if msg.is_empty() {
//         tmp
//     } else {
//         msg
//     }
// }
fn concat<'a>(msg: &'a str, tmp: &'a str) -> &'a str {
    if msg.is_empty() {
        tmp
    } else {
        msg
    }
}

//- What is borrow-checker?
//- borrow-checker uses lifetime to validate the code
//- No need to worry about lifetime if you don't have references

//- in a fn, each parameter that is reference gets its own lifetime parameter
//- if only one input parameter, then input lifetime assigned to all output parameter
//- except if one of them is &self or &mut self, the lifetime of self is assigned to all output parameter

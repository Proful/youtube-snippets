// https://doc.rust-lang.org/nomicon/lifetime-elision.html

//^ Lifetime Elision rules are as follows:

//~ (Rule 1) Each elided lifetime in input position becomes a distinct lifetime parameter.

// fn print(s: &str); // elided
// fn print<'a>(s: &'a str); // expanded

// fn debug(lvl: usize, s: &str); // elided
// fn debug<'a>(lvl: usize, s: &'a str); // expanded

//@ (Rule 2) If there is exactly one input lifetime position (elided or not),
//@          that lifetime is assigned to all elided output lifetimes.

// until is not reference
// fn substr(s: &str, until: usize) -> &str; // elided
// fn substr<'a>(s: &'a str, until: usize) -> &'a str; // expanded

// fn new(buf: &mut [u8]) -> BufWriter; // elided
// fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded

//% (Rule 3) If there are multiple input lifetime positions,
//%          but one of them is &self or &mut self, the lifetime of self
//%          is assigned to all elided output lifetimes.
// fn get_mut(&mut self) -> &mut T; // elided
// fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

// fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
// fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

//! (Error) Otherwise, it is an error to elide an output lifetime.
// fn get_str() -> &str; // ILLEGAL
// fn frob(s: &str, t: &str) -> &str; // ILLEGAL

fn main() {}

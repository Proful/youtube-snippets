#![allow(warnings)]

//^ Chapter 13 - Part 3 (closure)
// rs_ch13_p3_closure
// https://www.youtube.com/watch?v=9PIn4suU3jM
fn main() {
    let mut hello = || println!("Hello Friends");

    hello();
    // hello.call(());
    // hello.call_mut(());
    // hello.call_once(());

    let x = vec![1, 2, 3];
    let get_x = || x;

    // dbg!(x); error - above line behaved like move

    //@ hello.call(());
    //@ hello.call_mut(());
    //# hello.call_once(());
}

//^ Fn Traits
// different from fn pointer

// Takes a owned reference to self
// only call single times
// you moved the value to the fn, so no longer call agan
trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

// FnOnce -- super trait
// fnm^t
// Takes a exclusive reference to self
// calls once at a time
trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

// Takes a shared reference to self
// call multiple times without mutating state
trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

//- closures & functions automatically implemented them

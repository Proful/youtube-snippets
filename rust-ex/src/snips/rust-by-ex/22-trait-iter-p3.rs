#![allow(warnings)] // NOT RECOMMENDED

use std::iter::Iterator;

#[derive(Debug)]
struct Counter {
    current: u32,
    next: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.next;
        self.next += 1;
        Some(current)
    }
}

fn main() {
    let alphabets = ['a', 'b', 'c'];
    let mut iter = alphabets.iter();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());


    let mut counter = Counter { current: 0, next: 1 };

    for _ in 0..5 {
        println!("{:?}", counter.next());
    }
    dbg!(counter);
}

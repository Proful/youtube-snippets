#![allow(warnings)]

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // associated types
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// rs_ch13_p3_iter
fn main() {
    let mut counter = Counter::new();
    dbg!(&counter.next());
    dbg!(&counter.next());
    dbg!(&counter.next());
}

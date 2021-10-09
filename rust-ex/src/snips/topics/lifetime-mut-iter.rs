#![allow(warnings)]

#[derive(Debug)]
struct MyMutIter<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T> Iterator for MyMutIter<'a, T> {
    // one & only one exclusive mutable ref
    type Item = &'a mut T;

    // iterator are
    fn next(&mut self) -> Option<Self::Item> {
        // re-borrow
        // reference to reference
        let slice = &mut self.slice;

        // self.slice will be empty
        let slice = std::mem::replace(slice, &mut []);

        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        todo!()
    }
    // next fn borrowed self from it's calling fn
    // self ref only valid till next fn i.e., next lifetime
    // fn next<'next>(&mut self) -> Option<Self::Item> {
    //     // self.slice also has 'next lifetimes
    //     let first = self.slice.get_mut(0);

    //     // first has `a lifetime, this is not going to be deed soon
    //     first
    // }
}

//^ lifetime
fn main() {
    let mut numbers = vec![5, 10, 15, 20];

    let my_iter = MyMutIter {
        slice: &mut numbers,
    };

    dbg!(my_iter);
}

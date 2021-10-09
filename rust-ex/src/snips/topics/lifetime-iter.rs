#![allow(warnings)]

// https://www.youtube.com/watch?v=MSi3E5Z8oRw&t=634s
// MyIter will be valid till slice is valid
#[derive(Debug)]
struct MyIter<'a, T> {
    // uused & reference, that means it is a borrow
    // if you are using borrow, then need to mention
    // how long that is valid
    slice: &'a [T],
}

impl<'a, T> Iterator for MyIter<'a, T> {
    type Item = &'a T;

    // iterator are
    fn next(&mut self) -> Option<Self::Item> {
        // pointers are not expensive computation
        let (first, rest) = self.slice.split_first()?;
        self.slice = rest;
        Some(first)
        // if self.slice.is_empty() {
        //     return None;
        // }
        // let first = self.slice.get(0);
        // self.slice = &self.slice[1..];
        // first
    }
}
// impl<'a, T> Iterator for MyIter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.slice.is_empty() {
//             return None;
//         }

//         // don't know why &self?
//         let first = &self.slice[0];
//         self.slice = &self.slice[1..];
//         Some(&first)
//         // todo!()
//     }
// }

//^ lifetime
fn main() {
    let numbers = vec![5, 10, 15, 20];

    // if numbers moved here, how can I print after for loop
    // for num in numbers.iter_mut() {
    //     *num = *num * 5
    // }

    // dbg!(&numbers[..]);

    // why?? [..]
    // why mut keyword not used here
    let my_iter = MyIter {
        slice: &numbers,
        // slice: &numbers[..], // used in tutorial, but both worked
    };

    // my_iter is mutable
    for num in my_iter {
        dbg!(num);
    }
    dbg!(&numbers);
    // dbg!(&my_iter);
}

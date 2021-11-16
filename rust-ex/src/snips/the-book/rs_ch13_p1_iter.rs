#![allow(warnings)]

// rs_ch13_p1_iter
fn main() {
    let numbers = vec![5, 10, 15];

    // lazy - no effect till you call next
    // let mut numbers_itr = numbers.iter();
    let mut numbers_itr = numbers.iter();

    dbg!(&numbers_itr);

    let first = numbers_itr.next();
    dbg!(first);

    dbg!(&numbers_itr);
    dbg!(&numbers);

    for num in &numbers {
        dbg!(num);
    }

    for num in numbers_itr {
        dbg!(num);
    }
}

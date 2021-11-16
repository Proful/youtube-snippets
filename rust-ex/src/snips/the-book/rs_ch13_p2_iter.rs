#![allow(warnings)]

// rs_ch13_p2_iter
fn main() {
    // TODO - into_iter, iter_mut, iter
    let numbers = vec![5, 10, 15];
    let mut numbers_iter = numbers.into_iter();
    dbg!(numbers_iter.next());
    // dbg!(numbers);

    let mut numbers = vec![5, 10, 15];
    let mut numbers_iter = numbers.iter_mut();

    for num in numbers_iter {
        *num += 50;
    }
    // dbg!(numbers_iter.next());
    dbg!(numbers);

    let numbers = vec![5, 10, 15];
    let numbers_iter = numbers.iter();
    let total: i32 = numbers_iter.sum();

    dbg!(total);
    // dbg!(numbers_iter);

    //^ iterator adapters
    // convert one type of iterators to other
    // chain multiple
    // all iterator adapters are lazy, so u need to call consuming adapter to get results
    let numbers = vec![5, 10, 15];
    let numbers_iter: Vec<i32> = numbers.iter().map(|x| x + 5).collect();
    dbg!(numbers_iter);
    dbg!(numbers);
}

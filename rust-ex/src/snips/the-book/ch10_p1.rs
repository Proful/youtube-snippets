#![allow(warnings)]

// Ch 10 - part1
// Generic Types, Traits & Lifetimes
fn main() {
    //^ Find the largest number inside a vector
    let numbers = vec![5, 50, 20, 35, 10];

    // let mut largest = numbers[0];

    // for num in numbers {
    //     if num > largest {
    //         largest = num;
    //     }
    // }

    // dbg!(largest);
    dbg!(largest(&numbers));

    let list_chars = vec!['d', 'a', 'b'];
    dbg!(largest(&list_chars));

    let list_str = vec!["abc", "xyz", "xef", "xzf"];
    dbg!(largest(&list_str));
}

// fn largest<T>(list: &[T]) -> &T {
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for num in list {
        if num > largest {
            largest = num;
        }
    }

    &largest
}
// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for num in list {
//         if num > largest {
//             largest = num;
//         }
//     }

//     &largest
// }

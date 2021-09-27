#![allow(warnings)]

use colour::*;

// Chapter 3 -- part 3 (end)
fn main() {
    let mut count = 0;

    loop {
        count += 1;

        cyan_ln!("[main] count: {}", count);
        if count == 10 {
            break;
        }
    }
    cyan_ln!("[main] count(outside): {}", count);
    let result = loop {
        count += 1;

        // cyan_ln!("[main] count: {}", count);
        if count == 10 {
            break count + 5;
        }
    };
    cyan_ln!("[main] result(outside): {}", result);

    while count < 10 {
        count += 1;
        magenta_ln!("[main->while] count: {}", count);
    }

    let numbers = [5, 10, 15, 20, 25];

    for num in numbers.iter() {
        yellow_ln!("[main->for] num: {}", num);
    }
    // 1..=5 (two dot =)
    for num in (1..5).rev() {
        yellow_ln!("[main->for] num: {}", num);
    }
}

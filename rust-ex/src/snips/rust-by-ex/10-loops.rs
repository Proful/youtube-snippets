#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let age: u8 = "34".parse().unwrap();
    let age = "24".parse::<i32>().unwrap();

    let x = 5;

    let y = {
        let z = 3;
        x + z // try after adding ;
    };

    dbg!(y);

    let age = 30;

    if age >= 21 {
        println!("You can buy a drink!");
    } else {
        println!("You can't buy a drink!");
    }

    let is_of_age = {
        if age >= 21 {
            true
        } else {
            false
        }
    };
    dbg!(is_of_age);

    let mut count = 0;

    loop {
        count += 1;

        if count == 5 {
            continue;
        }

        //dbg!(count);

        if count == 10 {
            break;
        }
    }
    dbg!(count);

    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 5;
        }
    };
    dbg!(result);

    'outer: loop {
        println!("outer loop");

        'inner: loop {
            println!("inner loop");

            break 'outer;
        }

        println!("never reached");
    }

    let mut count = 0;

    while count < 10 {
        count += 1;

        if count == 5 {
            continue;
        }

        if count == 10 {
            break;
        }
    }
    dbg!(count);

    for i in 1..11 {
        dbg!(i);
    }

    for (i, j) in (5..11).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    let arr = [1, 2, 3, 4, 5];

    for i in arr.iter() {
        dbg!(i);
    }

    let names = vec!["John", "Jane", "Joe"];

    for name in names.iter() {
        dbg!(name);
    }
}
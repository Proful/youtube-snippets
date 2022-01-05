#![allow(warnings)] // NOT RECOMMENDED

fn main() {
    let mut numbers = vec![10, 2, 30];

    numbers.sort();

    dbg!(numbers);

    let mut numbers = vec![10.5, 2.1, 30.5];

    // numbers.sort();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    dbg!(numbers);

    #[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
    struct Person {
        name: String,
        age: u8,
    }

    let mut people = vec![
        Person {
            name: "John".to_string(),
            age: 45,
        },
        Person {
            name: "Anthony".to_string(),
            age: 30,
        },
        Person {
            name: "Mary".to_string(),
            age: 25,
        },
    ];

    people.sort();

    dbg!(&people);

    people.sort_by(|a, b| a.age.cmp(&b.age));

    dbg!(&people);

}
#![allow(warnings)] // NOT RECOMMENDED

use std::collections::HashMap;


#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Green"), 25);

    let team_score = scores.get(&String::from("Blue"));
    dbg!(team_score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    dbg!(scores.remove(&String::from("Yellow")));

    let mut point_maps = HashMap::new();
    point_maps.insert(Point { x: 10, y: 20 }, 10);
    point_maps.insert(Point { x: 20, y: 30 }, 20);

    for (key, value) in &point_maps {
        println!("{:?}: {}", key, value);
    }
}

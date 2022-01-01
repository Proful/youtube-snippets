#![allow(warnings)] // NOT RECOMMENDED

use std::collections::HashSet;

fn main() {
    let mut cities = HashSet::new();
    cities.insert("London");
    cities.insert("Paris");
    cities.insert("Tokyo");
    cities.insert("Jeypore");

    dbg!(&cities);

    dbg!(cities.contains("London"));


    let mut fav_cities = HashSet::new();
    fav_cities.insert("Jeypore");
    fav_cities.insert("Sydney");

    dbg!(fav_cities.union(&cities));
    dbg!(fav_cities.intersection(&cities));
    dbg!(fav_cities.difference(&cities));
    dbg!(fav_cities.symmetric_difference(&cities));
}

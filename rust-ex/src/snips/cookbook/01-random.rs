#![allow(warnings)] // NOT RECOMMENDED

use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform, Standard};
use rand::distributions::Alphanumeric;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        Point {
            x: rng.gen_range(0..10),
            y: rng.gen_range(0..10),
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let rnd_num: u32 = rng.gen();
    println!("{}", rnd_num);
    println!("{}", rng.gen::<f64>());

    println!("1 to 100 = {}", rng.gen_range(1..=100));

    println!("{}", Uniform::new(1, 10).sample(&mut rng));

    let p1: Point = rng.gen();
    let p2: Point = rng.gen();

    dbg!(&p1);
    dbg!(&p2);

    let pwd: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(|c| char::from(c))
        .collect();
    dbg!(&pwd);
}
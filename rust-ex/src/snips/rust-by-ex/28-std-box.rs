#![allow(warnings)] // NOT RECOMMENDED

use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("{:?}", p);
    println!("mem of p = {} bytes (64 + 64 = 128 = 16 * 16)", mem::size_of_val(&p));

    let box_p = Box::new(p);
    println!("{:?}", box_p);
    println!("mem of box_p = {} bytes", mem::size_of_val(&box_p));

    // let p2 = *box_p;
    // println!("{:?}", p2);

    let box_box_p = Box::new(box_p);
    println!("{:?}", box_box_p);
    println!("mem of box_box_p = {} bytes", mem::size_of_val(&box_box_p));


}

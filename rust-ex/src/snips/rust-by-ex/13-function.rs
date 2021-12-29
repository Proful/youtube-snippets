#![allow(warnings)] // NOT RECOMMENDED

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
    fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
    // self: &mut Self
    fn add(&mut self, pair: (i32, i32)) {
        self.x += pair.0;
        self.y += pair.1;
    }
}

fn main() {
    let sum = add(1, 2);
    dbg!(sum);

    let origin = Point::origin();
    let p = Point::new(1, 2);

    dbg!(&origin);
    dbg!(&p);

    p.print();
    Point::print(&p);

    let mut p = Point::new(1, 2);
    p.add((10, 40));
    p.print();
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
#![allow(warnings)] // NOT RECOMMENDED

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Traits are not types, if there is a declaration trait Bar { ... }, you cannot write x: Bar. Traits are bounds on types.
trait Shape {
    // fn new(x: i32, y: i32) -> Self;
    fn print(&self);
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl Shape for Point {
    fn print(&self) {
        println!("x = {} y = {}", self.x, self.y);
    }
}

// A box is just a reference to some memory in the heap.
// unsized type or dynamically sized type
// size not known at compile time
// dynamic dispatch
fn fav_shape(x: i32, y: i32) -> Box<dyn Shape> {
    let p = Point::new(x, y);
    Box::new(p)
}

// impl Trait is a completely static, compile-time thing.
// sized value, avoid heap allocation
// static dispatch
fn random_shape() -> impl Shape {
    Point::new(1, 2)
}

fn main() {
    let p1 = Point::new(1, 2);
    p1.print();

    let p2 = Point::new(1, 2);
    p2.print();

    let p3 = fav_shape(10, 20);
    p3.print();

    let p4 = random_shape();
    p4.print();
}

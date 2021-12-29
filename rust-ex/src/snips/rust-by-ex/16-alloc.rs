#![allow(warnings)] // NOT RECOMMENDED

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping point at ({}, {})", self.x, self.y);
    }
}

// RAII
// - Resource Acquisition Is Initialization
fn main() {
    let box1 = Box::new(5);

    {
        let box2 = Box::new(4);
        let sum = *box1 + *box2;
        println!("Sum is {}", sum);
    }

    boxy();

    let p = Point { x: 3.3, y: 4.4 };
    dbg!(&p);
}

fn boxy() {
    let box1 = Box::new(5);
}
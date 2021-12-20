#![allow(warnings)] // NOT RECOMMENDED

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let (x, y) = (100, 200);
    dbg!(x, y);

    let nested = ((1, 2), (3, 4));
    dbg!(nested);

    let p = Point { x: 1.0, y: 2.0 };
    dbg!(&p);


    let (x, y) = point_to_tuple(&p);
    dbg!(x, y);

    let Point { x, y } = p;
    dbg!(x, y);

}

fn point_to_tuple(p: &Point) -> (f64, f64) {
    (p.x, p.y)
}
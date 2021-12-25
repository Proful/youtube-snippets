#![allow(warnings)] // NOT RECOMMENDED

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from(point: (i32, i32)) -> Self {
        Point {
            x: point.0,
            y: point.1,
        }
    }
}

fn main() {
    let name_str = "Proful";
    let name_string = String::from(name_str);

    let p = Point { x: 0, y: 7 };
    dbg!(p);

    let p = Point::from((3, 4));
    dbg!(p);

    let pair = (1, 2);
    let p: Point = pair.into();
    dbg!(p);
}
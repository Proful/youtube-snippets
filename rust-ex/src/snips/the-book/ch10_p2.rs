#![allow(warnings)]

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        // why returning a reference here?
        &self.x
    }
}

// Ch 10 - part2
fn main() {
    let point1 = Point { x: 50, y: 20 };
    let point2 = Point { x: 20.00, y: 10.50 };

    dbg!(point2.x());
    //^ performance using Generics
    // no runtime penalty
    let a = Some(5);

    //* monomorphization
    enum Option_i32 {
        Some(i32),
        None,
    }
    let a = Option_i32::Some(5);
}

//^ Enum definations
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

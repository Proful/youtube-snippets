#![allow(warnings)]
// https://www.youtube.com/watch?v=Kde5Y424azM

#[derive(Debug)]
struct Employee<'a> {
    name: &'a str,
}

//^ lifetime - part2
fn main() {
    let emp3;
    {
        let name = "Michel"; // where actual data "Michel" stored? in heap or stack
                             //let name = String::from("Steve");// actual data "Steve" stored in heap
        emp3 = Employee { name: name };
    }
    // dbg!(emp3, name);
    dbg!(emp3);

    let emp4;
    {
        let name = String::from("Rahul");
        emp4 = Employee {
            name: name.as_str(),
        };
    }
    dbg!(emp4);
}

// let emp1 = Employee { name: "Steve" };
// dbg!(emp1);

// let name = String::from("Kenny");
// let emp2 = Employee {
//     name: name.as_str(),
// };
// dbg!(emp2);

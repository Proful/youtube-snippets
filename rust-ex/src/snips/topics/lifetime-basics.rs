#![allow(warnings)]
// https://www.youtube.com/watch?v=r8uTSz32RZs

// Avoid cloning using RC
// RC -> Reference counting
// use std::rc::Rc;

// #[derive(Debug)]
// struct Employee {
//     id: i32,
//     name: &'static str, //if default static used, no need to specify at struct declaraation time
// }

//@ 'static
//- valid till entire lifespan of the program
//- outlived all other lifetime

// Inner data doesn't get cloned
// pub struct Employee(Rc<EmployeeInner>);
#[derive(Debug)]
struct Employee<'a> {
    id: i32,
    name: &'a str, //name will be around till Employee type will be around
}

impl<'a> Employee<'a> {
    fn name(&self) -> &str {
        &self.name // How &String can be returned as &str
    }
}
// struct Employee {
//     id: i32,      // stored in stack
//     name: String, // type that is owned
// }

// impl Employee {
//     fn name(&self) -> &str {
//         &self.name // How &String can be returned as &str
//     }
// }

//^ lifetime
fn main() {
    let name = "Proful"; //name will be around till Employee type will be around
    let emp = Employee { id: 1, name };
    let name = "Proful".to_string(); //name will be around till Employee type will be around
    let emp = Employee { id: 1, name: &name };
    // dbg!(name);
    // dbg!(name);
    // dbg!(emp);
    // dbg!(emp);

    let mut emp2 = Employee {
        id: 2,
        name: "Kenny",
    };
    {
        let name2 = "Steve";
        emp2.name = name2;
    }
    dbg!(&emp2);
    {
        let name3 = "Steve".to_string();
        emp2.name = &name3;
    }
    dbg!(&emp2);
}

//- lifetime doesn't come into play if you deals with somethin temporary
//- There is no way to create lifetime, you just name it! & specify the relationship between references

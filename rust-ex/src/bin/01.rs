fn greeting(name: String) -> String {
    // create a String after variable substitution
    // allocate the memory in heap, and passing the ownership to caller fn
    // return is optional (last thing in a function), also need to omit ;
    format!("Hi {}", name)
}

// warning: function `sayHi` should have a snake case name
// i64 => signed 64-bit integer
fn say_hi(age: i64) {
    println!("You are {}year old", age)
}

// function too long to type, so fn :)
fn main() {
    // ! => macro
    // helps to save write lots of code
    // generate more code under the hood!
    // tightly integrated with cargo & tooling
    // println is a macro
    // can take more than one arguments (variable)
    // println!("Hello, world!! {}", "hi");
    // say_hi(12); // fn => take defined number of aguments

    // leaving in the heap somewhere
    // let name = "Proful".to_string();
    // println!("Hello {}", name);

    let name = "Kenny".to_string();
    let gr = greeting(name);
    println!("Hello {}", name);
    println!("greeting {}", gr);
}

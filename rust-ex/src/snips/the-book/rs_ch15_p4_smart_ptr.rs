#![allow(warnings)]

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        dbg!(&self.data);
    }
}

//^ Smart Pointers
//- NOT NEEDED
// rs_ch15_p4_smart_ptr
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // c.drop();//error
    drop(c);
    println!("CustomSmartPointers after manual ddrop.");
}

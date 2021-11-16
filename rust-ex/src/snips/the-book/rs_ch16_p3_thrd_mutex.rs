#![allow(warnings)]

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
//^ Threads
//- Mutex
// rs_ch16_p3_thrd_mutex
fn main() {
    let m = Mutex::new(5);

    // dbg!(&m);
    {
        // This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
        // The call to lock would fail if another thread holding the lock panicked.
        // num => as a mutable reference to the data inside
        let mut num = m.lock().unwrap();
        *num = 10;
    }

    // dbg!(&m);

    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

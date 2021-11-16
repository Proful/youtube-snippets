#![allow(warnings)]

use std::thread;
use std::time::Duration;

use colour::{green_ln, red_ln};

//^ Threads
//- spawn & join handle
//- usage of move closure
// rs_ch16_p1_thrd
fn main() {
    // the new thread will be stopped when the main thread ends, whether or not it has finished running
    let join_handle = thread::spawn(|| {
        for i in 1..10 {
            red_ln!("Number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // join_handle.join().unwrap();

    // order of thread execution depends on how your operating system schedules the threads
    for i in 1..5 {
        green_ln!("Number {} from the main thread!", i);
        // force a thread to stop its execution for a short duration, allowing a different thread to run.
        thread::sleep(Duration::from_millis(1));
    }

    join_handle.join().unwrap();

    let name = String::from("Proful");

    // without move -> compile error
    let join_handle = thread::spawn(move || {
        // only needs a reference to name
        green_ln!("Hi from {}", name);
    });

    // Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid
    // Then, when the spawned thread starts to execute, v is no longer valid, so a reference to it is also invalid.
    // drop(name)

    join_handle.join().unwrap();
}

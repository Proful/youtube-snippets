#![allow(warnings)]

use std::thread;
use std::{sync::mpsc, time::Duration};

use colour::{green_ln, red_ln};

//^ Threads
//- Transfer data between threads
//- channel, transmitter, receiver
//- message passing
// rs_ch16_p2_thrd_channel
fn main() {
    let (tx, rx) = mpsc::channel();

    // The spawned thread needs to own the transmitting end of the channel to be able to send messages through the channel.
    thread::spawn(move || {
        let name = String::from("Proful");
        tx.send(name).unwrap();
        // println!("val is {}", val); //Error
    });

    // will block the main thread’s execution and wait until a value is sent down the channel
    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time.
    let received = rx.recv().unwrap();
    green_ln!("Hi {}", received);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        red_ln!("Got: {}", received);
    }

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        green_ln!("Got: {}", received);
    }
}

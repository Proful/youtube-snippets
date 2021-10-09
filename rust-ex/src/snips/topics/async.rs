#![allow(warnings)]

use std::future::Future;

// Async & Await
fn main() {
    // hello().await;
}

async fn hello() {
    dbg!("hello..");
}

// - Future trait signifies a value not exist yet
// future does nothing until it is awaited
fn hello_ex() -> impl Future<Output = ()> {
    async {}
}

// async fn hello()->usize {
//     0
// }

// fn hello_ex() -> impl Future<Output = usize> {
//     async { 0 }
// }

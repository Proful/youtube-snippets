#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);

    let name = String::from("Proful");
    tokio::spawn(async move{
        println!("Hello, {}!", name);
    }).await.unwrap();
}
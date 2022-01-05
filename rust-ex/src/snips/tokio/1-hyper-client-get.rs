#![allow(warnings)] // NOT RECOMMENDED

use hyper::{Client, http};
use hyper::body::HttpBody as _;


#[tokio::main]
async fn main() {
    let posts_url: http::Uri = "http://jsonplaceholder.typicode.com/posts".parse().unwrap();

    // dbg!(&posts_url.scheme());
    // dbg!(&posts_url.host());
    // dbg!(&posts_url.path());
    // dbg!(&posts_url.query());

    let client = Client::new();

    let mut resp = client.get(posts_url).await.unwrap();

    // dbg!(&resp);
    // dbg!(&resp.status());
    // dbg!(&resp.headers());
    // dbg!(&resp.version());

    // dbg!(&response.body());
    let body = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
    // dbg!(&body);

    let posts: serde_json::Value = serde_json::from_slice(&body).unwrap();
    // dbg!(&posts);
    dbg!(&posts[9]);
    dbg!(&posts[9]["title"]);

    // while let Some(chunk) = resp.body_mut().data().await {
    //     println!("----------------------------------------");
    //     println!("{}", std::str::from_utf8(&chunk.unwrap()).unwrap());
    // }
}


#![allow(warnings)] // NOT RECOMMENDED

use hyper::{Body, Method, Request, Uri};
use hyper::body::HttpBody as _;
use serde::{Deserialize, Serialize};
use hyper_tls::HttpsConnector;

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {
   let posts_url = "https://jsonplaceholder.typicode.com/posts";

   let post_request = Post {
       userId: 1010,
       id: 101,
       title: "hello there".to_string(),
       body: "this is great".to_string(),
   };

   let request = Request::builder()
      .method(Method::POST)
      .uri(posts_url)
      .header("content-type", "application/json")
      //.body(Body::from("{\"title\": \"foo\", \"body\": \"bar\", \"userId\": 1}"))
      //   .body(Body::from(r#"{"title": "hello post", "body": "something interesting", "userId": 1}"#))
      .body(Body::from(serde_json::to_string(&post_request).unwrap()))
      .unwrap();

    // let client = hyper::Client::new();

    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);


    let mut resp = client.request(request).await.unwrap();

    println!("Status: {}", resp.status());

    let body = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
    // dbg!(&body);

    // let post: serde_json::Value = serde_json::from_slice(&body).unwrap();
    // dbg!(&post);

    let post: Post = serde_json::from_slice(&body).unwrap();
    dbg!(&post);
}


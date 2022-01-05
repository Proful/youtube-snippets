#![allow(warnings)] // NOT RECOMMENDED

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}
#[derive(Debug, Deserialize)]
struct Post {
    userId: Option<i32>,
    id: Option<i32>,
    title: Option<String>,
    body: Option<String>,
}

/* */
#[tokio::main]
async fn main() {

    let posts_url = "https://jsonplaceholder.typicode.com/posts";
    let mut resp = reqwest::blocking::get(posts_url).unwrap();

    resp.copy_to(&mut std::io::stdout()).unwrap();

    let post_res = res.text().unwrap();
    dbg!(&post_res);

    let posts_url = "https://jsonplaceholder.typicode.com/posts";
    let resp = reqwest::get(posts_url).await.unwrap();

    dbg!(&resp);
    dbg!(&resp.url().host());
    dbg!(&resp.status());
    dbg!(&resp.headers());
    dbg!(&resp.headers()["content-type"]);
    dbg!(&resp.headers().get("content-type"));
    dbg!(&resp.remote_addr());
    dbg!(&resp.text().await.unwrap());

    // Using serde_json > loosely typed way of representing any valid JSON value
    let posts: serde_json::Value = resp.json().await.unwrap();
    dbg!(&posts[10]);
    dbg!(&posts[10]["body"]);
    dbg!(&posts[10]["title"].as_str().unwrap());
    dbg!(&posts[10]["id"].as_i64().unwrap());

    let posts: Vec<Post> = resp.json().await.unwrap();

    dbg!(posts);

    for post in posts {
        println!("{:?} {:?}", post.id, post.title);
    }

    let mut post_map = HashMap::new();
    post_map.insert("userId", "1");
    post_map.insert("id", "101");
    post_map.insert("title", "hello world");
    post_map.insert("body", "lorem ipsum");

    let post_map = serde_json::json!({
        "userId": 1,
        "title": "hello world",
        "body": "lorem ipsum"
    });

    let client = reqwest::Client::new();

    let resp = client.post(posts_url)
        .json(&post_map)
        .send()
        .await.unwrap();

    let resp = client.post(posts_url)
        .form(&[("userId", "11"),
            ("title", "hello world"),
            ("body", "lorem ipsum")])
        .header(reqwest::header::ACCEPT, "application/json")
        .header(reqwest::header::AUTHORIZATION, "Bearer teryzfdfef")
        .send()
        .await.unwrap();

    // let mut post_map: HashMap<String, String> = HashMap::new();
    dbg!(&resp.status());
    dbg!(&resp.text().await.unwrap());

    let post_json: serde_json::Value = resp.json().await.unwrap();
    dbg!(&post_json);
    dbg!(&post_json["userId"]);

}
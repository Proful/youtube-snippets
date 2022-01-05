#![allow(warnings)] // NOT RECOMMENDED

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Todo {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

 fn main() {
    let todos_url = "https://jsonplaceholder.typicode.com/posts";
    let mut resp = reqwest::blocking::get(todos_url).unwrap();

    resp.copy_to(&mut std::io::stdout()).unwrap();

    let todo_res = res.text().unwrap();
    dbg!(&todo_res);

    let todos: Vec<Todo> = resp.json().unwrap();
    dbg!(resp);
    dbg!(todos);
    for todo in todos {
        println!("{} {}", todo.id, todo.title);
    }

    let todo = Todo {
        userId: Some(1),
        id: Some(1),
        title: Some("foo".to_string()),
        body: Some("bar".to_string()),
    };

    let client = reqwest::blocking::Client::new();
    let res = client.post("https://jsonplaceholder.typicode.com/posts")
                    .body("the exact body that is sent")
                    .send().unwrap();

    let todo_res = res.text().unwrap();
    let todo_res = res.json::<Todo>().unwrap();
    dbg!(res);
    dbg!(todo_res);
}

use std::{path::Path, sync::Arc};

use axum::{
    routing::{delete, get},
    Json, Router,
};
use chrono::{DateTime, Local, NaiveDate, TimeZone};
use serde::{self, Deserialize, Serialize};

struct Error {}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Store {
    posts: Vec<Post>,
}

impl Store {
    fn init() -> Self {
        let file = Arc::new(include_str!("../data.json"));
        match serde_json::from_str(&file) {
            Ok(data) => Store {
                posts: data,
            },
            Err(e) => {
                println!("{e}");
                Store { posts: vec![] }
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct PostID(String);

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Post {
    id: PostID,
    image: Option<String>,
    text: String,
    author: String,
    date: DateTime<Local>,
    likes: usize,
    bookmarks: usize,
    comments: Vec<Comment>,
}


impl Post {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: PostID,
        image: &str,
        text: &str,
        author: &str,
        likes: usize,
        bookmarks: usize,
        comments: Vec<Comment>,
    ) -> Self {
        Post {
            id,
            image: Some(image.to_string()),
            text: text.to_string(),
            author: author.to_string(),
            date: Local::now(),
            likes,
            bookmarks,
            comments,
        }
    }

}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Comment {}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/blogs", get(blogs).post(post_blog))
        .route(
            "/blogs/{id}",
            get(single_blog)
                .post(post_blog)
                .put(put_blog)
                .delete(delete_blog),
        );

    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4445").await.unwrap();
    println!("{time} start the server on http://localhost:4445/");
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "welcome!"
}

async fn blogs() -> Json<Vec<Post>> {
    // Json(Store::init().posts)
    Json(vec![Post::new(PostID("1".to_string()), "image", "text", "author", 1, 1, vec![])])
}

async fn single_blog() {}

async fn post_blog() {}

async fn put_blog() {}

async fn delete_blog() {}

use axum::{routing::{delete, get, post}, Router};
use chrono::{DateTime, Local, NaiveDate, TimeZone};



struct Error {}

struct Store {
    posts: Post,
}

struct Post {
    image: Option<String>,
    text: String,
    author: String,
    date: DateTime<Local>,
    likes: usize,
    bookmarks: usize,
    comments: Option<Vec<Comment>>,
}

struct Comment {}




#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/blogs", get(blogs).post(post_blog))
        .route("/blogs/{id}", get(single_blog).post(post_blog).put(put_blog).delete(delete_blog));

    let time  = Local::now().format("%Y-%m-%d %H:%M:%S");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4445").await.unwrap();
    println!("{time} start the server on http://localhost:4445/");
    axum::serve(listener, app).await.unwrap();
}


async fn home() -> &'static str {
    "welcome!"
}

async fn blogs() {
    
}

async fn single_blog() {
    
}

async fn post_blog() {
    
}

async fn put_blog() {
    
}

async fn delete_blog() {
    
}
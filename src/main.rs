mod error;
mod routes;
mod store;
mod types;

use axum::{routing::{delete, get}, Router};
use chrono::Local;
use routes::{
    blogs::{
        blog_comments, blog_text, blogs, delete_blog, delete_blog_comment, post_blog,
        post_blog_comments, post_blog_text, put_blog, put_blog_text, single_blog,
    },
    home::home,
};
use store::Store;

#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:4431@localhost:5432/blog_api").await;
    let app = Router::new()
        .route("/", get(home))
        .route("/blogs", get(blogs).post(post_blog))
        .route(
            "/blogs/{id}",
            get(single_blog).put(put_blog).delete(delete_blog),
        )
        .route(
            "/blogs/{id}/text",
            get(blog_text).put(put_blog_text).post(post_blog_text),
        )
        .route(
            "/blogs/{id}/comments",
            get(blog_comments)
                .post(post_blog_comments),
        )
        .route("/blogs/{id}/comments/{id}", delete(delete_blog_comment))
        .with_state(store);

    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4445").await.unwrap();
    println!("{time} start the server on http://localhost:4445/");
    axum::serve(listener, app).await.unwrap();
}


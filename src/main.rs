mod error;
mod routes;
mod store;
mod types;

use axum::{routing::get, Router};
use chrono::Local;
use routes::{
    blogs::{blogs, delete_blog, post_blog, put_blog, single_blog},
    home::home,
};

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

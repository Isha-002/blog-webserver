mod error;
mod routes;
mod store;
mod types;

use std::sync::Arc;

use axum::{routing::get, Router};
use chrono::Local;
use routes::{
    blogs::{blogs, delete_blog, post_blog, put_blog, single_blog},
    home::home,
};
use store::Store;

#[tokio::main]
async fn main() {
    let store = Arc::new(Store::init());
    let app = Router::new()
        .route("/", get(home))
        .route("/blogs", get(blogs).post(post_blog))
        .route(
            "/blogs/{id}",
            get(single_blog)
                .put(put_blog)
                .delete(delete_blog),
        )
        .with_state(store);

    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4445").await.unwrap();
    println!("{time} start the server on http://localhost:4445/");
    axum::serve(listener, app).await.unwrap();
}


// problems:
// - no idea how to implement error handling
// - need a better solution to handle ids
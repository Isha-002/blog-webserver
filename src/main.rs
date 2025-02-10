mod error;
mod routes;
mod store;
mod types;
use std::fs::{create_dir_all, OpenOptions};

use axum::{
    http::{self, Method},
    routing::{delete, get},
    Router,
};
use chrono::Local;
use routes::{
    blogs::{
        blog_comments, blog_text, blogs, delete_blog, delete_blog_comment, post_blog,
        post_blog_comments, post_blog_text, put_blog, put_blog_text, single_blog,
    },
    home::home,
};
use store::Store;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing_subscriber::{
    fmt::format::{self, FmtSpan},
    prelude::*,
};
use types::custome_time::CustomTimer;

#[tokio::main]
async fn main() {
    let timer = CustomTimer;
    create_dir_all("log").expect("failed to create log directory");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log/info.log")
        .expect("couldn't open or create the log file");
    let (none_blocking, _worker_guard) = tracing_appender::non_blocking(file);

    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "blog_api=info,tower_http=info,axum::rejection=trace".to_owned());
    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_writer(none_blocking)
        .with_ansi(false)
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact()
        .fmt_fields(
            format::debug_fn(|writer, field, value| write!(writer, "[{}: {:?}]", field, value))
                .delimited(" - "),
        )
        .init();

    let store = Store::new("postgres://postgres:4431@localhost:5432/blog_api").await;

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:4446".parse().unwrap()))
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ]))
        .allow_headers(AllowHeaders::list([http::header::CONTENT_TYPE]));

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
            get(blog_comments).post(post_blog_comments),
        )
        .route("/blogs/{id}/comments/{id}", delete(delete_blog_comment))
        .with_state(store)
        .layer(cors);

    let time = Local::now().format("%Y-%m-%d %H:%M:%S");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4445").await.unwrap();
    println!("{time} start the server on http://localhost:4445/");
    tracing::info!("Server started on http://localhost:4445/");
    axum::serve(listener, app).await.unwrap();
}

mod error;
mod routes;
mod store;
mod types;
mod utils;
use std::fs::{create_dir_all, OpenOptions};

use axum::{
    http::{self, HeaderValue, Method},
    routing::{delete, get},
    Router,
};
use chrono::Local;
use owo_colors::OwoColorize;
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
use types::custom_time::CustomTimer;
use utils::{
    arguments::arguments,
    setting::{config_builder, LogLevel, ServerConfig},
};

#[tokio::main]
async fn main() {
    println!("{}", "Starting the application...".magenta());

    let arguments = arguments();

    let db_url = arguments
        .get_one::<String>("database url")
        .cloned()
        .unwrap_or_else(|| "postgres://postgres:4431@localhost:5432/blog_api".to_string());

        let server_port = arguments
        .get_one::<u16>("server port")
        .cloned()
        .unwrap_or(4445)
        .to_string();
    
    let origin = arguments
        .get_one::<u16>("set origin")
        .cloned()
        .unwrap_or(4446)
        .to_string();

    let log_level = arguments
        .get_one::<String>("log level")
        .cloned()
        .unwrap_or_else(|| "info".to_string());

    // let this be here for when we get in trouble
    let _construct_config = arguments.get_one::<bool>("config").cloned().unwrap_or(true);

    let config_type = ServerConfig {
        db_url: db_url.clone(),
        server_port,
        origin_port: origin,
        log_level: log_level.parse::<LogLevel>().unwrap_or(LogLevel::info),
    };

    let config = match config_builder(config_type) {
        Ok(c) => c,
        Err(e) => {
            panic!("Couldn't construct {} File: {e}\n\nRestart the App or turn off this feature using {}", "config".bright_red(), "--save=false")
        }
    };

    println!(
        "{} {}",
        "Database URL:".cyan(),
        config.db_url.bright_black()
    );
    println!(
        "{} {}",
        "Server Port:".cyan(),
        config.server_port.bright_black()
    );
    println!(
        "{} {}",
        "Allow Origin on Port:".cyan(),
        config.origin_port.bright_black()
    );
    println!(
        "{} {}\n",
        "Log Level:".cyan(),
        config.log_level.bright_black()
    );

    let timer = CustomTimer;
    create_dir_all("log").expect("failed to create log directory");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log/info.log")
        .expect("couldn't open or create the log file");
    let (none_blocking, _worker_guard) = tracing_appender::non_blocking(file);

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "blog_api={},tower_http={},axum::rejection=trace",
            config.log_level, config.log_level
        )
    });
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

    let store = Store::new(&config.db_url).await;

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(
            format!("0.0.0.0:{}", config.origin_port)
                .parse()
                .unwrap_or_else(|_|{
                    println!(
                        "Due to an unexpected error Allow origin value change to 0.0.0.0:9999"
                    );
                    HeaderValue::from_static("0.0.0.0:9999")
                })
        ))
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
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port)).await.unwrap();
    let print_server_start = format!("{time} start the server on http://localhost:{}/", config.server_port);
    println!("{}", print_server_start);
    tracing::info!(print_server_start);
    axum::serve(listener, app).await.unwrap();
}

// todos
// reading and writing configs to a toml file [done]
// automatically read from a migration file if not exist create one with the defaults provided

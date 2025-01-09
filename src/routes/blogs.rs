use axum::Json;

use crate::types::blog::Blog;

pub async fn blogs() -> Json<Vec<Blog>> {
    // Json(Store::init().posts)
    Json(vec![Blog::new(
        crate::types::blog::BlogID("1".to_string()),
        "image",
        "text",
        "author",
        1,
        1,
        vec![],
    )])
}

pub async fn single_blog() {}

pub async fn post_blog() {}

pub async fn put_blog() {}

pub async fn delete_blog() {}

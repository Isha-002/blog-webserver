use std::sync::Arc;

use axum::{
    extract::{self, Path, State}, http::StatusCode, response::{IntoResponse, Response}, Json
};

use crate::{
    store::Store,
    types::blog::Blog,
};

pub async fn blogs(State(store): State<Arc<Store>>) -> Json<Vec<Blog>> {
    Json((*store.posts).read().await.to_vec())
}

pub async fn single_blog(
    State(store): State<Arc<Store>>,
    Path(blog_id): Path<usize>,
) -> Json<Blog> {
    Json(store.posts.read().await[blog_id].clone())
}

pub async fn post_blog(
    State(store): State<Arc<Store>>,
    extract::Json(payload): extract::Json<Blog>,
) -> Response {
    store.posts.write().await.push(payload);
    (StatusCode::OK, "blog added").into_response()
}

pub async fn put_blog() {}

pub async fn delete_blog() {}

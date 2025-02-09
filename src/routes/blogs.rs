use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::Error,
    store::Store,
    types::blog::{Blog, NewBlog, Pagination},
};

pub async fn blogs(
    State(store): State<Store>,
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<Blog>>, Error> {
    match store.blogs(params).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn single_blog(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
) -> Result<Json<Blog>, Error> {
    match store.get_single_blog(blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn post_blog(
    State(store): State<Store>,
    Json(payload): Json<NewBlog>,
) -> Result<Json<Blog>, Error> {
    match store.post_blog(payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn put_blog(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
    Json(payload): Json<Blog>,
) -> Result<Json<Blog>, Error> {
    match store.put_blog(payload, blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_blog(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
) -> Result<StatusCode, Error> {
    match store.delete_blog(blog_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

pub async fn blog_text() {}

pub async fn put_blog_text() {}

pub async fn post_blog_text() {}

pub async fn blog_comments() {}

pub async fn post_blog_comments() {}

pub async fn delete_blog_comments() {}

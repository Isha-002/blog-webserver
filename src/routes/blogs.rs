use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::Error,
    store::Store,
    types::{blog::{Blog, NewBlog, Pagination, Text}, comment::{Comment, NewComment}},
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

pub async fn blog_text(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
) -> Result<Json<Text>, Error> {
    match store.blog_text(blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn put_blog_text(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
    Json(payload): Json<Text>,
) -> Result<Json<Text>, Error> {
    match store.put_blog_text(payload ,blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn post_blog_text(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
    Json(payload): Json<Text>,
) -> Result<Json<Text>, Error> {
    match store.post_blog_text(payload, blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}


pub async fn blog_comments(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
) -> Result<Json<Vec<Comment>>, Error> {
    match store.get_blog_comments(blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn post_blog_comments(
    State(store): State<Store>,
    Path(blog_id): Path<i64>,
    Json(payload): Json<NewComment>,
) -> Result<Json<Comment>, Error> {
    match store.post_blog_comments(payload, blog_id).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e),
    }
}

pub async fn delete_blog_comment(
    State(store): State<Store>,
    Path((blog_id, comment_id)): Path<(i64, i64)>,
) -> Result<StatusCode, Error> {
    match store.delete_blog_comment(blog_id, comment_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e),
    }
}

use std::{ptr::null, sync::Arc};

use axum::{
    extract::{self, rejection::JsonRejection, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Error, Json,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BlogParams {
    start: Option<usize>,
    end: Option<usize>,
}

use crate::{store::Store, types::blog::Blog};

pub async fn blogs(
    State(store): State<Arc<Store>>,
    Query(params): Query<BlogParams>,
) -> Result<Json<Vec<Blog>>, (StatusCode, String)> {
    let res = (*store.posts).read().await.to_vec();
    if let (Some(start), Some(end)) = (params.start, params.end) {
        if end > res.len() {
            return Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                "end parameter is out of bounds".to_string(),
            ));
        }
        let sliced_res = res[start..end].to_vec();
        Ok(Json(sliced_res))
    } else if params.start.is_none() && params.end.is_none() {
        Ok(Json(res))
    } else if params.end.is_none() && params.start.is_some_and(|s| s < res.len()) {
        let Some(start) = params.start else { panic!() };
        Ok(Json(res[start..].to_vec()))
    } else {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "missing params or out of bounds".to_string(),
        ));
    }
}

pub async fn single_blog(
    State(store): State<Arc<Store>>,
    Path(blog_id): Path<usize>,
) -> Result<Json<Blog>, (StatusCode, String)> {
    let res = store.posts.read().await;
    if blog_id == 0 || blog_id > res.len() {
        return Err((
            StatusCode::NOT_FOUND,
            "couldnt find block".to_string(),
        ));
    }
    Ok(Json(res[blog_id - 1].clone()))
}

pub async fn post_blog(State(store): State<Arc<Store>>, Json(payload): Json<Blog>) -> StatusCode {
    store.posts.write().await.push(payload);
    StatusCode::CREATED
}

pub async fn put_blog(
    State(store): State<Arc<Store>>,
    Path(blog_id): Path<usize>,
    Json(payload): Json<Blog>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut res = store.posts.write().await;
    if blog_id == 0 || blog_id > res.len() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "path is not acceptable".to_string(),
        ));
    }
    res[blog_id - 1] = payload;
    Ok(StatusCode::OK)
}

pub async fn delete_blog(
    State(store): State<Arc<Store>>,
    Path(blog_id): Path<usize>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut res = store.posts.write().await;

    if blog_id == 0 || blog_id > res.len() {
        return Err((
            StatusCode::BAD_REQUEST,
            "blog not found".to_string(),
        ));
    }
    res.remove(blog_id - 1);
    Ok(StatusCode::NO_CONTENT)
}

pub async fn blog_text() {}

pub async fn put_blog_text() {}

pub async fn post_blog_text() {}

pub async fn blog_comments() {}

pub async fn post_blog_comments() {}

pub async fn delete_blog_comments() {}
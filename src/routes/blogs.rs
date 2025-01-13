use std::{ptr::null, sync::Arc};

use axum::{
    extract::{self, rejection::JsonRejection, Path, Query, State}, http::StatusCode, response::{IntoResponse, Response}, Error, Json
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
    println!("blog params: {params:?}");
    // let res = (*store.posts).read().await.to_vec();
    let res = (*store.posts).to_vec();
    if let (Some(start), Some(end)) = (params.start, params.end) {
        let sliced_res = res[start..end].to_vec();
        Ok(Json(sliced_res))
    } else {
        if params.start.is_none() || params.end.is_none() {
            return Err((StatusCode::UNPROCESSABLE_ENTITY, "missing params".to_string()));
        }
        Ok(Json(res))
    }
    }


pub async fn single_blog(
    State(store): State<Arc<Store>>,
    Path(blog_id): Path<usize>,
) -> Json<Blog> {
    // Json(store.posts.read().await[blog_id].clone())
    Json(store.posts[blog_id].clone())
}

// pub async fn post_blog(State(store): State<Arc<Store>>, Json(payload): Json<Blog>) -> StatusCode {
//     store.posts.write().await.push(payload);
//     StatusCode::CREATED
// }
pub async fn post_blog() {}

pub async fn put_blog(State(store): State<Arc<Store>>) {}

pub async fn delete_blog() {}

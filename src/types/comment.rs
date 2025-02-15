use serde::{Deserialize, Serialize};
use sqlx::Type;
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "comments")]
#[derive(sqlx::FromRow)]
pub struct Comment {
    pub id: i64,
    pub blog_id: i64,
    pub author: String,
    pub text: String,
    pub likes: i32,
    pub date: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(type_name = "comments")]
#[derive(sqlx::FromRow)]
pub struct NewComment {
    pub blog_id: i64,
    pub author: String,
    pub text: String,
    pub likes: i32,
    pub date: String,
}

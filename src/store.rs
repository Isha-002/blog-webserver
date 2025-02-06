use std::time::Duration;

use sqlx::{postgres::{PgPoolOptions, PgRow}, PgPool};
use sqlx::Row;
use crate::{
    error::Error,
    types::blog::{Blog, BlogID, Pagination},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(3))
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("coundln't establish a database connection: {e:?}"),
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn blogs(&self, page: Pagination) -> Result<Vec<Blog>, Error> {
        let total_items = match sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM blogs;")
            .fetch_one(&self.connection)
            .await
        {
            Ok(t) => t,
            Err(e) => return Err(Error::db_query_error(e)),
        };
        let pagination = match page.calculate_items(total_items) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        match sqlx::query("SELECT * from blog LIMIT $1 OFFSET $2")
        .bind(pagination.1)
        .bind(pagination.0)
        .map(|row: PgRow | Blog {
            id: BlogID(row.get("id")),
            image: row.get("image"),
            author: row.get("author"),
            date: row.get("date"),
            likes: row.get("likes"),
            bookmarks: row.get("bookmarks"),
            comments: row.get("comments")
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(blogs) => Ok(blogs),
            Err(e) => {
                Err(Error::db_query_error(e))
            }
        }
    }
}

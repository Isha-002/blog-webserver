use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlogID(pub i64);

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Blog {
    pub id: BlogID,
    pub image: Option<String>,
    pub author: String,
    pub date: NaiveDateTime,
    pub likes: i64,
    pub bookmarks: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewBlog {
    pub image: Option<String>,
    pub author: String,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Text {
    pub blog_id: i64,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
}
impl Pagination {
    pub fn calculate_items(&self, total_items: i64) -> Result<(i64, Option<i64>), Error> {
        if self.page.is_none() {
            return Ok((0, None));
        } else if self.page < Some(1) {
            return Err(Error::invalid_offset);
        }
        let page = self.page.unwrap();
        let offset = (page - 1) * 10;
        let mut limit = offset + 9;
        if offset >= total_items {
            return Err(Error::out_of_range_offset);
        } else if limit > total_items {
            limit = total_items;
            return Ok((offset, Some(limit)));
        }
        Ok((offset, Some(limit)))
    }
}

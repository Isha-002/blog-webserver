use chrono::Local;
use serde::{Deserialize, Serialize};

use super::comment::Comment;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlogID(pub usize);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Blog {
    pub id: BlogID,
    pub image: Option<String>,
    pub text: String,
    pub author: String,
    pub date: String,
    pub likes: usize,
    pub bookmarks: usize,
    pub comments: Vec<Comment>,
}

impl Blog {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: BlogID,
        image: &str,
        text: &str,
        author: &str,
        likes: usize,
        bookmarks: usize,
        comments: Vec<Comment>,
    ) -> Self {
        Blog {
            id,
            image: Some(image.to_string()),
            text: text.to_string(),
            author: author.to_string(),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            likes,
            bookmarks,
            comments,
        }
    }
}

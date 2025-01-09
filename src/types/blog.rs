use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::comment::Comment;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlogID(pub String);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Blog {
    pub id: BlogID,
    pub image: Option<String>,
    pub text: String,
    pub author: String,
    pub date: DateTime<Local>,
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
            date: Local::now(),
            likes,
            bookmarks,
            comments,
        }
    }
}

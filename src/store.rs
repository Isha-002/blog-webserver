use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::types::blog::Blog;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Store {
    pub posts: Vec<Blog>,
}

impl Store {
    pub fn init() -> Self {
        let file = Arc::new(include_str!("../data.json"));
        match serde_json::from_str(&file) {
            Ok(data) => Store { posts: data },
            Err(e) => {
                println!("{e}");
                Store { posts: vec![] }
            }
        }
    }
}

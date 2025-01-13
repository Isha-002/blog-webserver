use std::sync::Arc;
use tokio::sync::RwLock;

use crate::types::blog::Blog;

#[derive(Debug, Clone)]
pub struct Store {
    // pub posts: Arc<RwLock<Vec<Blog>>>,
    pub posts: Vec<Blog>,
}

impl Store {
    pub fn init() -> Self {
        let file = Arc::new(include_str!("../data.json"));
        match serde_json::from_str(&file) {
            Ok(data) => Store {
                // posts: Arc::new(RwLock::new(data)),
                posts: data,
            },
            Err(e) => {
                println!("there waas an error when reading the data.json: {e}");
                Store {
                    // posts: Arc::new(RwLock::new(vec![])),
                    posts: vec![],
                }
            }
        }
    }
}

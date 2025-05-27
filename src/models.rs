use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub content: String,
}

impl Prompt {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.trim().to_string(),
            content: content.trim().to_string(),
            tags: tags.into_iter()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        }
    }
}

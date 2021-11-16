use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub content: String
}
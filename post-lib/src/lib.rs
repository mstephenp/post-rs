//! Shared structs for client and server

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub post_id: u64,
    pub content: String,
}

/// a request to update a post, given an id and updated content
#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub post_id: u64,
    pub updated_content: String,
}

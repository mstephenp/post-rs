//! Requests Module
//!
//! These are request definitions

use serde::Deserialize;

/// a request to create a new post
#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
}

/// a request to update a post, given an id and updated content
#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub post_id: u64,
    pub updated_content: String,
}

//! Requests Module
//!
//! These are request definitions

use serde::Deserialize;

/// a request to create a new user
#[derive(Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
}

/// a request to update a user, given an id and updated fields
#[derive(Deserialize)]
pub struct UpdatePostRequest {
    pub post_id: u64,
    pub updated_content: String,
}

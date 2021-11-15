mod post_db;
mod requests;

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub use post_db::{Post, PostDb, PostDbResponse, PostDbStatus};
use requests::{CreatePostRequest, UpdatePostRequest};
use serde::Serialize;

/// Get All Posts
pub async fn get_all_posts_handler<'a>(
    Extension(user_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let users = user_db.lock().unwrap().get_posts();
    (StatusCode::OK, Json(users))
}

/// Get Post By ID
pub async fn get_post_handler(
    Path(id): Path<u64>,
    Extension(user_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let response = user_db.lock().unwrap().get_post(id);
    response_handler(response)
}

/// Create New Post
pub async fn new_post_handler(
    Json(payload): Json<CreatePostRequest>,
    Extension(user_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let user_db_lock = user_db.lock();
    match user_db_lock {
        Ok(mut user_db) => {
            let response = user_db.create_post(payload.content);
            response_handler(response)
        }
        Err(e) => {
            eprintln!("error getting db lock: {}", e);
            response_handler(PostDbResponse {
                status: PostDbStatus::Err,
                value: 0,
            })
        }
    }
}

/// Update Post By ID (update content)
pub async fn update_post_handler(
    Json(payload): Json<UpdatePostRequest>,
    Extension(user_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let response = user_db
        .lock()
        .unwrap()
        .update_post(payload.post_id, payload.updated_content);
    response_handler(response)
}

/// Delete Post By ID
pub async fn delete_post_handler(
    Path(id): Path<u64>,
    Extension(user_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let response = user_db.lock().unwrap().delete_post(id);
    response_handler(response)
}

/// Handle the response
fn response_handler<T: Serialize>(response: PostDbResponse<T>) -> impl IntoResponse {
    match response.status {
        PostDbStatus::Ok => (StatusCode::OK, Json(response.value)),
        PostDbStatus::Err => (StatusCode::EXPECTATION_FAILED, Json(response.value)),
    }
}

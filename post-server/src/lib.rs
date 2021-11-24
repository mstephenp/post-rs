mod post_db;

use std::sync::{Arc, Mutex};

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub use post_db::{PostDb, PostDbResponse, PostDbStatus};
use post_lib::{CreatePostRequest, UpdatePostRequest};
use serde::Serialize;

/// Get All Posts
pub async fn get_all_posts_handler<'a>(
    Extension(post_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let posts = post_db.lock().unwrap().get_posts();
    (StatusCode::OK, Json(posts))
}

/// Get Post By ID
pub async fn get_post_handler(
    Path(id): Path<u64>,
    Extension(post_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let response = post_db.lock().unwrap().get_post(id);
    response_handler(response)
}

/// Create New Post
pub async fn new_post_handler(
    Json(payload): Json<CreatePostRequest>,
    Extension(post_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let post_db_lock = post_db.lock();
    match post_db_lock {
        Ok(mut post_db) => {
            let response = post_db.create_post(payload.content);
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
    Extension(post_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let response = post_db
        .lock()
        .unwrap()
        .update_post(payload.post_id, payload.updated_content);
    response_handler(response)
}

/// Delete Post By ID
pub async fn delete_post_handler(
    Path(id): Path<u64>,
    Extension(post_db): Extension<Arc<Mutex<PostDb>>>,
) -> impl IntoResponse {
    let response = post_db.lock().unwrap().delete_post(id);
    response_handler(response)
}

/// Handle the response
fn response_handler<T: Serialize>(response: PostDbResponse<T>) -> impl IntoResponse {
    match response.status {
        PostDbStatus::Ok => (StatusCode::OK, Json(response.value)),
        PostDbStatus::Err => (StatusCode::EXPECTATION_FAILED, Json(response.value)),
    }
}

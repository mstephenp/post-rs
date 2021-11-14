use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};

use post_server::{
    delete_post_handler, get_all_posts_handler, get_post_handler, new_post_handler,
    update_post_handler, PostDb,
};

#[tokio::main]
async fn main() {
    let db = create_post_db();
    let app = app(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_post_db() -> Arc<Mutex<PostDb>> {
    Arc::new(Mutex::new(Default::default()))
}

fn app(db: Arc<Mutex<PostDb>>) -> Router {
    Router::new()
        .route("/posts", get(get_all_posts_handler))
        .route("/post/:id", get(get_post_handler))
        .route("/addPost", post(new_post_handler))
        .route("/updatePost", post(update_post_handler))
        .route("/deletePost/:id", post(delete_post_handler))
        .layer(AddExtensionLayer::new(db))
}

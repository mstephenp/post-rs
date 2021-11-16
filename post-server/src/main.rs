use std::sync::{Arc, Mutex};

use axum::{
    routing::{get, post},
    AddExtensionLayer, Router,
};

use hyper::{Method, header::CONTENT_TYPE};
use tower_http::cors::{CorsLayer, Origin};

use post_server::{
    delete_post_handler, get_all_posts_handler, get_post_handler, new_post_handler,
    update_post_handler, PostDb,
};

/// The main application entry point
#[tokio::main]
async fn main() {
    let db = create_post_db();
    let app = app(db);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_post_db() -> Arc<Mutex<PostDb>> {
    Arc::new(Mutex::new(Default::default()))
}

fn app(db: Arc<Mutex<PostDb>>) -> Router {

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Origin::exact("http://localhost:8080".parse().unwrap()))
        .allow_credentials(false)
        .allow_headers(vec![CONTENT_TYPE]);

    Router::new()
        .route("/posts", get(get_all_posts_handler))
        .route("/post/:id", get(get_post_handler))
        .route("/addPost", post(new_post_handler))
        .route("/updatePost", post(update_post_handler))
        .route("/deletePost/:id", post(delete_post_handler))
        .layer(cors)
        .layer(AddExtensionLayer::new(db))
}

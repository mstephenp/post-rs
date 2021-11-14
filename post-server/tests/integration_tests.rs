/// Integration tests
use std::{
    net::{SocketAddr, TcpListener},
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    routing::{get, post},
    AddExtensionLayer, Router,
};

use serde_json::{json, Value};

use tower::ServiceExt;

use post_server::{
    delete_post_handler, get_all_posts_handler, new_post_handler, update_post_handler, PostDb,
};

fn create_post_db() -> Arc<Mutex<PostDb>> {
    Arc::new(Mutex::new(Default::default()))
}

fn app(db: Arc<Mutex<PostDb>>) -> Router {
    Router::new()
        .route("/posts", get(get_all_posts_handler))
        .route("/post/:id", get(get_all_posts_handler))
        .route("/addPost", post(new_post_handler))
        .route("/updatePost", post(update_post_handler))
        .route("/deletePost/:id", post(delete_post_handler))
        .layer(AddExtensionLayer::new(db))
}

#[tokio::test]
async fn new_db_empty() {
    let db = create_post_db();
    let app = app(db);
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/posts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn create_post() {
    let db = create_post_db();
    let app = app(db);
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/addPost")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    "{\"content\": \"this is some content\"}".to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!(1));
}

#[tokio::test]
async fn update_post() {
    let db = create_post_db();
    let app = app(db);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/addPost")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    "{\"content\": \"this is some content\"}".to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!(1));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/updatePost")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    "{\"post_id\": 1, \"updated_content\": \"updated content\"}".to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!(1));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/post/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert!(body != json!([{"post_id": 1, "content": "this is some content"}]));
    assert_eq!(body, json!([{"post_id": 1, "content": "updated content"}]));
}

#[tokio::test]
async fn delete_post() {
    let listener = TcpListener::bind("127.0.0.1:4321".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();
    let db = create_post_db();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app(db).into_make_service())
            .await
            .unwrap()
    });

    let client = hyper::Client::new();

    let response = client
        .request(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("http://{}/addPost", addr))
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    "{\"content\": \"this is some content\"}".to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"1");

    let response = client
        .request(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("http://{}/deletePost/1", addr))
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    "{\"content\": \"this is some content\"}".to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"1");

    let response = client
        .request(
            Request::builder()
                .method(http::Method::GET)
                .uri(format!("http://{}/posts", addr))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"[]");
}

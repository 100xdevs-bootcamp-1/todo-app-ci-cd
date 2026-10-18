use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn send(app: &Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(body.map_or(Body::empty(), |b| Body::from(b.to_string())))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, value)
}

#[tokio::test]
async fn crud_flow() {
    let app = rust_todo::app();

    let (status, todo) = send(&app, "POST", "/todos", Some(json!({ "title": "write ci" }))).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(todo, json!({ "id": 1, "title": "write ci", "done": false }));

    let (_, list) = send(&app, "GET", "/todos", None).await;
    assert_eq!(list.as_array().unwrap().len(), 1);

    let (_, updated) = send(&app, "PATCH", "/todos/1", Some(json!({ "done": true }))).await;
    assert_eq!(updated["done"], true);

    let (status, _) = send(&app, "DELETE", "/todos/1", None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = send(&app, "DELETE", "/todos/1", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn rejects_empty_title() {
    let app = rust_todo::app();
    let (status, _) = send(&app, "POST", "/todos", Some(json!({ "title": "  " }))).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

mod helpers;

use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

async fn body_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

async fn create(app: &axum::Router, name: &str, display_order: i32) -> Value {
    let res = app.clone()
        .oneshot(
            Request::post("/component-groups")
                .header("content-type", "application/json")
                .body(Body::from(json!({"name": name, "display_order": display_order}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    body_json(res.into_body()).await
}

#[tokio::test]
async fn list_when_empty_returns_empty_array() {
    let app = helpers::setup().await;

    let res = app
        .oneshot(Request::get("/component-groups").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res.into_body()).await, json!([]));
}

#[tokio::test]
async fn list_returns_groups_sorted_by_display_order() {
    let app = helpers::setup().await;
    create(&app, "Backend", 2).await;
    create(&app, "Frontend", 1).await;
    create(&app, "Database", 3).await;

    let res = app
        .oneshot(Request::get("/component-groups").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = body_json(res.into_body()).await;
    let names: Vec<&str> = body.as_array().unwrap().iter()
        .map(|g| g["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["Frontend", "Backend", "Database"]);
}

#[tokio::test]
async fn create_returns_created_group() {
    let app = helpers::setup().await;
    let created = create(&app, "API", 1).await;
    let id = created["id"].as_str().unwrap();

    let res = app
        .oneshot(Request::get(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res.into_body()).await["name"], "API");
}

#[tokio::test]
async fn find_missing_returns_not_found() {
    let app = helpers::setup().await;
    let id = uuid::Uuid::new_v4();

    let res = app
        .oneshot(Request::get(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_returns_updated_group_with_partial_changes() {
    let app = helpers::setup().await;
    let created = create(&app, "Old Name", 1).await;
    let id = created["id"].as_str().unwrap();

    let res = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/component-groups/{id}"))
                .header("content-type", "application/json")
                .body(Body::from(json!({"name": "New Name"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = body_json(res.into_body()).await;
    assert_eq!(body["name"], "New Name");
    assert_eq!(body["display_order"], 1);
}

#[tokio::test]
async fn delete_returns_no_content_and_removes_group() {
    let app = helpers::setup().await;
    let created = create(&app, "To Delete", 1).await;
    let id = created["id"].as_str().unwrap();

    let res = app.clone()
        .oneshot(Request::delete(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let res = app
        .oneshot(Request::get(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_missing_returns_not_found() {
    let app = helpers::setup().await;
    let id = uuid::Uuid::new_v4();

    let res = app
        .oneshot(Request::delete(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

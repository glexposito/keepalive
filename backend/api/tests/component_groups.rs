mod helpers;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

const COMPONENT_GROUPS_PATH: &str = "/api/v1/component-groups";

async fn body_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn list_when_empty_returns_empty_array() {
    let test = helpers::setup().await;

    let res = test
        .router
        .oneshot(
            Request::get(COMPONENT_GROUPS_PATH)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res.into_body()).await, json!([]));
}

#[tokio::test]
async fn list_returns_groups_sorted_by_display_order() {
    let test = helpers::setup().await;
    test.insert_component_group("Backend", 2).await;
    test.insert_component_group("Frontend", 1).await;
    test.insert_component_group("Database", 3).await;

    let res = test
        .router
        .oneshot(
            Request::get(COMPONENT_GROUPS_PATH)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = body_json(res.into_body()).await;
    let names: Vec<&str> = body
        .as_array()
        .unwrap()
        .iter()
        .map(|g| g["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["Frontend", "Backend", "Database"]);
}

#[tokio::test]
async fn create_returns_created_group() {
    let test = helpers::setup().await;
    let res = test
        .router
        .clone()
        .oneshot(
            Request::post(COMPONENT_GROUPS_PATH)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name": "API", "display_order": 1}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    let body = body_json(res.into_body()).await;
    assert_eq!(body["name"], "API");
    assert_eq!(body["display_order"], 1);
    let id = uuid::Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();

    let persisted = test.find_component_group(id).await.unwrap();
    assert_eq!(persisted.name, "API");
    assert_eq!(persisted.display_order, 1);
}

#[tokio::test]
async fn find_missing_returns_not_found() {
    let test = helpers::setup().await;
    let id = uuid::Uuid::new_v4();

    let res = test
        .router
        .oneshot(
            Request::get(format!("{COMPONENT_GROUPS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_returns_updated_group_with_partial_changes() {
    let test = helpers::setup().await;
    let created = test.insert_component_group("Old Name", 1).await;
    let id = created.id;

    let res = test
        .router
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("{COMPONENT_GROUPS_PATH}/{id}"))
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

    let persisted = test.find_component_group(id).await.unwrap();
    assert_eq!(persisted.name, "New Name");
    assert_eq!(persisted.display_order, 1);
}

#[tokio::test]
async fn delete_returns_no_content_and_removes_group() {
    let test = helpers::setup().await;
    let created = test.insert_component_group("To Delete", 1).await;
    let id = created.id;

    let res = test
        .router
        .clone()
        .oneshot(
            Request::delete(format!("{COMPONENT_GROUPS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    assert!(test.find_component_group(id).await.is_none());
}

#[tokio::test]
async fn delete_missing_returns_not_found() {
    let test = helpers::setup().await;
    let id = uuid::Uuid::new_v4();

    let res = test
        .router
        .oneshot(
            Request::delete(format!("{COMPONENT_GROUPS_PATH}/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

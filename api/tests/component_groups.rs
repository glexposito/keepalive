use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use keepalive_api::{app, infra::{db, stores::component_group::Store}};
use once_cell::sync::OnceCell;
use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use serde_json::{json, Value};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use tower::ServiceExt;

static DB: OnceCell<DatabaseConnection> = OnceCell::new();

async fn get_db() -> &'static DatabaseConnection {
    if let Some(db) = DB.get() {
        return db;
    }
    let container = Box::leak(Box::new(Postgres::default().start().await.unwrap()));
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);
    let conn = db::connect_with(&url).await.unwrap();
    db::run_migrations(&conn).await.unwrap();
    DB.get_or_init(|| conn)
}

async fn clean() {
    get_db().await
        .execute(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "TRUNCATE TABLE component_group RESTART IDENTITY CASCADE",
        ))
        .await
        .unwrap();
}

async fn make_app() -> axum::Router {
    app(Store::new(get_db().await.clone()))
}

async fn body_json(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

async fn create(name: &str, display_order: i32) -> Value {
    let res = make_app().await
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
    clean().await;

    let res = make_app().await
        .oneshot(Request::get("/component-groups").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res.into_body()).await, json!([]));
}

#[tokio::test]
async fn list_returns_groups_sorted_by_display_order() {
    clean().await;
    create("Backend", 2).await;
    create("Frontend", 1).await;
    create("Database", 3).await;

    let res = make_app().await
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
    clean().await;
    let created = create("API", 1).await;
    let id = created["id"].as_str().unwrap();

    let res = make_app().await
        .oneshot(Request::get(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(body_json(res.into_body()).await["name"], "API");
}

#[tokio::test]
async fn find_missing_returns_not_found() {
    clean().await;
    let id = uuid::Uuid::new_v4();

    let res = make_app().await
        .oneshot(Request::get(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_returns_updated_group_with_partial_changes() {
    clean().await;
    let created = create("Old Name", 1).await;
    let id = created["id"].as_str().unwrap();

    let res = make_app().await
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
    clean().await;
    let created = create("To Delete", 1).await;
    let id = created["id"].as_str().unwrap();

    let res = make_app().await
        .clone()
        .oneshot(Request::delete(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let res = make_app().await
        .oneshot(Request::get(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_missing_returns_not_found() {
    clean().await;
    let id = uuid::Uuid::new_v4();

    let res = make_app().await
        .oneshot(Request::delete(format!("/component-groups/{id}")).body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

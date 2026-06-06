#![allow(dead_code)]

use keepalive_api::{app, infra::{db, stores::component_group::Store}};

pub async fn setup() -> axum::Router {
    let conn = db::connect_with("sqlite::memory:").await.unwrap();
    db::run_migrations(&conn).await.unwrap();

    app(Store::new(conn))
}

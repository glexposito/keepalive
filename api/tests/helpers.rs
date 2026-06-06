#![allow(dead_code)]

use keepalive_api::{app, infra::{db, stores::component_group::Store}};
use sea_orm::{ConnectionTrait, Statement};
use testcontainers_modules::{postgres::Postgres, testcontainers::{ContainerAsync, runners::AsyncRunner}};
use tokio::sync::OnceCell;

static CONTAINER: OnceCell<ContainerAsync<Postgres>> = OnceCell::const_new();

async fn pg_port() -> u16 {
    let container = CONTAINER.get_or_init(|| async {
        Postgres::default().start().await.unwrap()
    }).await;
    container.get_host_port_ipv4(5432).await.unwrap()
}

pub async fn setup() -> axum::Router {
    let port = pg_port().await;
    let db_name = format!("test_{}", uuid::Uuid::new_v4().simple());

    let admin_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);
    let admin = db::connect_with(&admin_url).await.unwrap();
    admin.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        format!("CREATE DATABASE {db_name}"),
    )).await.unwrap();

    let url = format!("postgres://postgres:postgres@127.0.0.1:{}/{}", port, db_name);
    let conn = db::connect_with(&url).await.unwrap();
    db::run_migrations(&conn).await.unwrap();

    app(Store::new(conn))
}

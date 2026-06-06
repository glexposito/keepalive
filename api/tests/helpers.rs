#![allow(dead_code)]

use keepalive_api::{app, infra::{db, stores::component_group::Store}};
use testcontainers_modules::{postgres::Postgres, testcontainers::{ContainerAsync, runners::AsyncRunner}};

pub struct DbGuard {
    _container: ContainerAsync<Postgres>,
}

pub async fn setup() -> (axum::Router, DbGuard) {
    let container = Postgres::default().start().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

    let conn = db::connect_with(&url).await.unwrap();
    db::run_migrations(&conn).await.unwrap();

    (app(Store::new(conn)), DbGuard { _container: container })
}

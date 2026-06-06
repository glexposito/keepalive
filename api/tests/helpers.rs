#![allow(dead_code)]

use keepalive_api::{app, infra::{db, stores::component_group::Store}};
use sea_orm::{ConnectionTrait, Statement};
use std::sync::Mutex;
use testcontainers_modules::{postgres::Postgres, testcontainers::{ContainerAsync, runners::AsyncRunner}};
use tokio::sync::OnceCell;

static CONTAINER: Mutex<Option<ContainerAsync<Postgres>>> = Mutex::new(None);
static PG_PORT: OnceCell<u16> = OnceCell::const_new();

extern "C" fn stop_container() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Ok(mut guard) = CONTAINER.lock() {
                if let Some(c) = guard.take() {
                    c.rm().await.ok();
                }
            }
        });
}

async fn pg_port() -> u16 {
    *PG_PORT.get_or_init(|| async {
        let c = Postgres::default().start().await.unwrap();
        let port = c.get_host_port_ipv4(5432).await.unwrap();
        *CONTAINER.lock().unwrap() = Some(c);
        unsafe { libc::atexit(stop_container); }
        port
    }).await
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

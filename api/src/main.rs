mod core;
mod infra;
mod presentation;

use axum::Router;
use infra::stores::component_group::Store as ComponentGroupStore;

#[tokio::main]
async fn main() {
    let db = infra::db::connect().await.expect("failed to connect to database");
    infra::db::run_migrations(&db).await.expect("failed to run migrations");

    let app = Router::new()
        .nest("/component-groups", presentation::routers::component_groups::router())
        .with_state(ComponentGroupStore::new(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

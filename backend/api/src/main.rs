use keepalive_api::{app, infra::{db, stores::component_group::Store}};

#[tokio::main]
async fn main() {
    let db = db::connect().await.expect("failed to connect to database");
    db::run_migrations(&db).await.expect("failed to run migrations");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app(Store::new(db))).await.unwrap();
}

#![allow(dead_code)]

use keepalive_api::{
    app,
    core::component_group::ComponentGroup,
    infra::{db, stores::component_group::Store},
};
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, Statement, Value};
use uuid::Uuid;

pub struct TestApp {
    pub router: axum::Router,
    db: DatabaseConnection,
}

impl TestApp {
    pub async fn insert_component_group(&self, name: &str, display_order: i32) -> ComponentGroup {
        let id = Uuid::new_v4();

        self.db
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "INSERT INTO component_group (id, name, display_order) VALUES (?, ?, ?)",
                [
                    Value::String(Some(Box::new(id.to_string()))),
                    Value::String(Some(Box::new(name.to_owned()))),
                    Value::Int(Some(display_order)),
                ],
            ))
            .await
            .unwrap();

        ComponentGroup {
            id,
            name: name.to_owned(),
            display_order,
        }
    }

    pub async fn find_component_group(&self, id: Uuid) -> Option<ComponentGroup> {
        let row = self.db
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Sqlite,
                "SELECT id, name, display_order FROM component_group WHERE id = ?",
                [Value::String(Some(Box::new(id.to_string())))],
            ))
            .await
            .unwrap()?;

        let id: String = row.try_get("", "id").unwrap();
        let name: String = row.try_get("", "name").unwrap();
        let display_order: i32 = row.try_get("", "display_order").unwrap();

        Some(ComponentGroup {
            id: Uuid::parse_str(&id).unwrap(),
            name,
            display_order,
        })
    }
}

pub async fn setup() -> TestApp {
    let conn = db::connect_with("sqlite::memory:").await.unwrap();
    db::run_migrations(&conn).await.unwrap();

    TestApp {
        router: app(Store::new(conn.clone())),
        db: conn,
    }
}

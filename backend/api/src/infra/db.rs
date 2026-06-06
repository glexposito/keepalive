use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    connect_with(&url).await
}

pub async fn connect_with(url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(url).await
}

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    Migrator::up(db, None).await
}

use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryOrder, Set};
use uuid::Uuid;

use crate::core::component_group::ComponentGroup;

mod entity {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "component_group")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub name: String,
        pub display_order: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

impl From<entity::Model> for ComponentGroup {
    fn from(m: entity::Model) -> Self {
        Self {
            id: Uuid::parse_str(&m.id).expect("component group id must be a valid UUID"),
            name: m.name,
            display_order: m.display_order,
        }
    }
}

pub struct UpdateInput {
    pub name: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Clone)]
pub struct Store {
    db: DatabaseConnection,
}

impl Store {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all(&self) -> Result<Vec<ComponentGroup>, DbErr> {
        entity::Entity::find()
            .order_by_asc(entity::Column::DisplayOrder)
            .all(&self.db)
            .await
            .map(|rows| rows.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ComponentGroup>, DbErr> {
        entity::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
            .map(|opt| opt.map(Into::into))
    }

    pub async fn insert(&self, name: String, display_order: i32) -> Result<ComponentGroup, DbErr> {
        entity::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name),
            display_order: Set(display_order),
        }
        .insert(&self.db)
        .await
        .map(Into::into)
    }

    pub async fn update(&self, id: Uuid, input: UpdateInput) -> Result<Option<ComponentGroup>, DbErr> {
        let Some(row) = entity::Entity::find_by_id(id.to_string()).one(&self.db).await? else {
            return Ok(None);
        };
        let mut active: entity::ActiveModel = row.into();
        if let Some(name) = input.name {
            active.name = Set(name);
        }
        if let Some(order) = input.display_order {
            active.display_order = Set(order);
        }
        active.update(&self.db).await.map(|m| Some(m.into()))
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, DbErr> {
        entity::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await
            .map(|r| r.rows_affected > 0)
    }
}

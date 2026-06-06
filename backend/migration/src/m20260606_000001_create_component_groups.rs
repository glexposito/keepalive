use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ComponentGroup::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ComponentGroup::Id)
                            .string_len(36)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ComponentGroup::Name).string().not_null())
                    .col(
                        ColumnDef::new(ComponentGroup::DisplayOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ComponentGroup::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ComponentGroup {
    Table,
    Id,
    Name,
    DisplayOrder,
}

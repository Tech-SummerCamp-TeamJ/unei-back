use sea_orm_migration::prelude::*;

use crate::m20240914_140343_create_group::Group;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Group::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("theme"))
                            .string()
                            .not_null()
                            .default("#ffffff"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Group::Table)
                    .drop_column(Alias::new("theme"))
                    .to_owned(),
            )
            .await
    }
}

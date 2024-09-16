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
                    .drop_column(Group::MemberId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Group::Table)
                    .add_column(ColumnDef::new(Group::MemberId).uuid().not_null())
                    .to_owned(),
            )
            .await
    }
}

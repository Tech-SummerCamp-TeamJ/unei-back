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
                    .table(Member::Table)
                    .add_column(ColumnDef::new(Member::GroupId).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_group_id")
                            .from_tbl(Member::Table)
                            .from_col(Member::GroupId)
                            .to_tbl(Group::Table)
                            .to_col(Group::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Member::Table)
                    .drop_column(Member::GroupId)
                    .drop_foreign_key(Alias::new("fk_group_id"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Member {
    Table,
    Id,
    GroupId,
    UserId,
    IsAdmin,
}

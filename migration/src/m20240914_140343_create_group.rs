use sea_orm_migration::prelude::*;

use crate::m20240914_135509_create_member::Member;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .col(ColumnDef::new(Group::MemberId).uuid().not_null())
                    .col(ColumnDef::new(Group::IconPath).string())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_member_id")
                            .from(Group::Table, Group::MemberId)
                            .to(Member::Table, Member::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Group {
    Table,
    Id,
    Name,
    MemberId,
    IconPath,
}

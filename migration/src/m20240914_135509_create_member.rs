use sea_orm_migration::prelude::*;

use crate::{m20240914_134946_create_user::User, m20240914_140343_create_group::Group};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Member::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Member::UserId).uuid().not_null())
                    .col(ColumnDef::new(Member::IsAdmin).boolean().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_group_id")
                            .from(Member::Table, Member::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(Member::Table, Member::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
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

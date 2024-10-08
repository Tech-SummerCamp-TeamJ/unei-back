use sea_orm_migration::prelude::*;

use crate::m20240914_134946_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reaction::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Reaction::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Reaction::AuthorId).uuid().not_null())
                    .col(ColumnDef::new(Reaction::IconPath).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reaction_author_id")
                            .from(Reaction::Table, Reaction::AuthorId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reaction::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Reaction {
    Table,
    Id,
    AuthorId,
    IconPath,
}

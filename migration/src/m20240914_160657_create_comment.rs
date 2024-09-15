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
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Comment::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Comment::Content).string().not_null())
                    .col(ColumnDef::new(Comment::AuthorId).uuid().not_null())
                    .col(ColumnDef::new(Comment::ReactionId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_author_id")
                            .from(Comment::Table, Comment::AuthorId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum Comment {
    Table,
    Id,
    Content,
    AuthorId,
    ReactionId,
}

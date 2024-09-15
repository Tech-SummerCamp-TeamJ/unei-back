use sea_orm_migration::prelude::*;

use crate::{m20240914_134946_create_user::User, m20240914_153249_create_reaction::Reaction, m20240914_160657_create_comment::Comment};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CommentReactedUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CommentReactedUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CommentReactedUser::CommentId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CommentReactedUser::ReactionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CommentReactedUser::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comment_id")
                            .from(CommentReactedUser::Table, CommentReactedUser::CommentId)
                            .to(Comment::Table, Comment::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_reaction_id")
                            .from(CommentReactedUser::Table, CommentReactedUser::ReactionId)
                            .to(Reaction::Table, Reaction::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_id")
                            .from(CommentReactedUser::Table, CommentReactedUser::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CommentReactedUser::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CommentReactedUser {
    Table,
    Id,
    CommentId,
    ReactionId,
    UserId,
}

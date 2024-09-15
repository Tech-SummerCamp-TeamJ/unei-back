use sea_orm_migration::prelude::*;

use crate::{
    m20240914_134946_create_user::User, m20240914_145822_create_tag::Tag,
    m20240914_153249_create_reaction::Reaction, m20240914_160657_create_comment::Comment,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Event::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Event::Description).string().not_null())
                    .col(ColumnDef::new(Event::EventDate).date().not_null())
                    .col(ColumnDef::new(Event::CommentId).uuid())
                    .col(ColumnDef::new(Event::TagId).uuid())
                    .col(ColumnDef::new(Event::ReactionId).uuid())
                    .col(ColumnDef::new(Event::AuthorId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_comment_id")
                            .from(Event::Table, Event::CommentId)
                            .to(Comment::Table, Comment::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_tag_id")
                            .from(Event::Table, Event::TagId)
                            .to(Tag::Table, Tag::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_reaction_id")
                            .from(Event::Table, Event::ReactionId)
                            .to(Reaction::Table, Reaction::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_event_author_id")
                            .from(Event::Table, Event::AuthorId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    Description,
    EventDate,
    CommentId,
    TagId,
    ReactionId,
    AuthorId,
}

pub use sea_orm_migration::prelude::*;
mod m20240914_134946_create_user;
mod m20240914_135509_create_member;
mod m20240914_140343_create_group;
mod m20240914_145822_create_tag;
mod m20240914_153249_create_reaction;
mod m20240914_160657_create_comment;
mod m20240914_174810_create_event;
mod m20240915_113446_add_theme_to_group;
mod m20240915_125941_create_session;
mod m20240915_134829_remove_member_id_from_group;
mod m20240915_145308_add_group_id_to_member;
mod m20240915_231848_create_reaction;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240914_134946_create_user::Migration),
            Box::new(m20240914_135509_create_member::Migration),
            Box::new(m20240914_140343_create_group::Migration),
            Box::new(m20240914_145822_create_tag::Migration),
            Box::new(m20240914_153249_create_reaction::Migration),
            Box::new(m20240914_160657_create_comment::Migration),
            Box::new(m20240914_174810_create_event::Migration),
            Box::new(m20240915_113446_add_theme_to_group::Migration),
            Box::new(m20240915_125941_create_session::Migration),
            Box::new(m20240915_134829_remove_member_id_from_group::Migration),
            Box::new(m20240915_145308_add_group_id_to_member::Migration),
            Box::new(m20240915_231848_create_reaction::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;
mod m20240914_134946_create_user;
mod m20240914_135509_create_member;
mod m20240914_140343_create_group;
mod m20240914_145822_create_tag;
mod m20240914_153249_create_reaction;

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
        ]
    }
}

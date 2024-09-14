pub use sea_orm_migration::prelude::*;
mod m20240914_134946_create_user;
mod m20240914_135509_create_member;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240914_134946_create_user::Migration),
            Box::new(m20240914_135509_create_member::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240629_175455_create_role_grants;
mod m20240629_204456_create_campaigns;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        println!("Running migrations...");
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240629_175455_create_role_grants::Migration),
            Box::new(m20240629_204456_create_campaigns::Migration),
        ]
    }
}

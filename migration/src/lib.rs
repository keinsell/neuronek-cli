pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240728_000001_create_ingestion_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration), Box::new(m20240728_000001_create_ingestion_table::Migration)]
    }
}

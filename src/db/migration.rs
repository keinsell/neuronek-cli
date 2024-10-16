use crate::db::migrations;
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>>
    {
        vec![Box::new(
            migrations::m20240801_000001_create_ingestion_table::Migration,
        )]
    }
}

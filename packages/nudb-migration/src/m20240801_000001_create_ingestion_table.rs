use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingestion::Table)
                    .if_not_exists()
                    .col(pk_auto(Ingestion::Id))
                    .col(string(Ingestion::SubstanceName))
                    .col(float(Ingestion::Dosage))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own nudb-migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Ingestion::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ingestion {
    Table,
    Id,
    SubstanceName,
    /// Dosage is a representation of Mass (in terms of unit), it's a float type which represents
    /// kilograms, while this may not seem comfortable it's used to save additional definition of units
    /// which adds portion of mess and uncertainty into data model... Like, just represent
    /// everything in kilograms.
    Dosage,
    Notes,
    IngestedAt,
    CreatedAt,
    UpdatedAt,
}

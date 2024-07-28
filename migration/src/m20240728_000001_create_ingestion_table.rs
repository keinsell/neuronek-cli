use sea_orm_migration::{prelude::*, schema::*};
use sea_orm_migration::seaql_migrations::Entity;
use sea_entity::ingestion;
use crate::ColumnType::DateTime;
use crate::sea_orm::Schema;

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
                    .col(
                        ColumnDef::new(Ingestion::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .if_not_exists()
                    .col(ColumnDef::new(Ingestion::SubstanceId).integer().not_null())
                    .col(ColumnDef::new(Ingestion::DosageUnit).string().not_null())
                    .col(ColumnDef::new(Ingestion::DosageValue).double().not_null())
                    .col(ColumnDef::new(Ingestion::IngestedAt).date_time().not_null())
                    .col(ColumnDef::new(Ingestion::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Ingestion::UpdatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ingestion_substance")
                            .from(Ingestion::Table, Ingestion::SubstanceId)
                            .to(Substance::Table, Substance::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ingestion::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Ingestion {
    Table,
    Id,
    SubstanceId,
    DosageUnit,
    DosageValue,
    IngestedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Substance {
    Table,
    Id,
}
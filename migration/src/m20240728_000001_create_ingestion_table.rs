use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(Ingestion::IngestedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Ingestion::CreatedAt).timestamp_with_time_zone().not_null())
                    .col(ColumnDef::new(Ingestion::UpdatedAt).timestamp_with_time_zone().not_null())
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

#[derive(DeriveIden)]
pub enum Ingestion {
    Table,
    Id,
    SubstanceId,
    DosageUnit,
    DosageValue,
    #[sea_orm(type = ColumnType::Timestamp)]
    IngestedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Substance {
    Table,
    Id,
}
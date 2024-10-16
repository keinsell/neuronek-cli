use crate::db;
use crate::ingestion::ingestion::IngestionViewModel;
use clap::Parser;
use db::sea_orm::ColumnTrait;
use db::sea_orm::DatabaseConnection;
use db::sea_orm::EntityTrait;
use db::sea_orm::QueryFilter;
use db::sea_orm::QueryTrait;
use tabled::Table;
use tracing::instrument;

/// Retrieve and list ingestion from database, this feature includes functionality
/// of filtering and piping output to different formats such as listing ingestion
/// out in json so they can be used in different program or imported into different
/// places. All of this have sane default which allow using command without additional
/// setup or things like that.

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct ListIngestion
{
    #[arg(short = 's')]
    pub substance_name: Option<String>,
}

impl ListIngestion
{
    #[instrument(name="list_ingestion", level = Level::DEBUG)]
    pub async fn handle(&self, database_connection: &DatabaseConnection) -> ()
    {
        let ingestions = db::ingestion::Entity::find()
            .apply_if(self.substance_name.clone(), |query, v| {
                query.filter(db::ingestion::Column::SubstanceName.eq(v.clone()))
            })
            .all(database_connection)
            .await
            .unwrap();

        let view_models: Vec<IngestionViewModel> =
            ingestions.iter().map(IngestionViewModel::from).collect();

        println!("{}", Table::new(view_models));
    }
}

use clap::Parser;
use nudb_migration::sea_orm::ColumnTrait;
use nudb_migration::sea_orm::DatabaseConnection;
use nudb_migration::sea_orm::EntityTrait;
use nudb_migration::sea_orm::QueryFilter;
use nudb_migration::sea_orm::QueryTrait;
use tabled::Table;
use tracing::Level;
use tracing::instrument;

use crate::ingestion::ingestion::IngestionViewModel;

/// Retrive and list ingestions from database, this feature includes functionality
/// of filtering and piping output to different fromats such as listing ingestions
/// out in json so they can be used in differnt program or imprted into different
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
        let ingestions = nudb::ingestion::Entity::find()
            .apply_if(self.substance_name.clone(), |query, v| {
                query.filter(nudb::ingestion::Column::SubstanceName.eq(v.clone()))
            })
            .all(database_connection)
            .await
            .unwrap();

        let view_models: Vec<IngestionViewModel> =
            ingestions.iter().map(IngestionViewModel::from).collect();

        dbg!(&view_models);

        let table = Table::new(view_models);
        println!("{}", table)
    }
}

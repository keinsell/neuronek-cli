use crate::humanize::human_date_parser;
use crate::ingestion::RouteOfAdministrationClassification;
use crate::ingestion::ingestion::IngestionViewModel;
use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use nudb_migration::sea_orm::ActiveValue;
use nudb_migration::sea_orm::DatabaseConnection;
use nudb_migration::sea_orm::EntityTrait;
use tabled::Table;
use tracing::Level;
use tracing::event;
use tracing::instrument;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct LogIngestion
{
    #[arg(short = 's', long)]
    pub substance_name: String,
    #[arg(short = 'u', long, default_value_t=String::from("mg"))]
    pub dosage_unit: String,
    #[arg(short = 'v', long)]
    pub dosage_amount: f64,
    /// Date of ingestion, by default current date is used if not provided.
    ///
    /// Date can be provided as timestamp and in human-readable format such as
    /// "today 10:00", "yesterday 13:00", "monday 15:34" which will be later
    /// parsed into proper timestamp.
    #[arg(
                short='t',
                long,
                default_value_t=Local::now(),
                default_value="now",
                value_parser=human_date_parser,
            )]
    pub ingestion_date: DateTime<Local>,
    #[clap(short = 'r', long, default_value_t, value_enum)]
    pub route_of_administration: RouteOfAdministrationClassification,
}

impl LogIngestion
{
    #[instrument(name = "log_ingestion", level = Level::INFO)]
    pub async fn handle(command: &Self, database_connection: &DatabaseConnection)
    {
        dbg!("This should log ingestion {:?}", command);

        let insert_ingestion = nudb::ingestion::Entity::insert(nudb::ingestion::ActiveModel {
            id: ActiveValue::default(),
            substance_name: ActiveValue::Set(command.substance_name.to_lowercase()),
            route_of_administration: ActiveValue::Set(command.route_of_administration.serialize()),
            // TODO: Dodac parsowanie unitow masy i zapisywac informacje w kilogramach, output do uzytkownika powinien byc automatycznie skracany np. 0.0001 do mg czy g.
            dosage: ActiveValue::Set(command.dosage_amount as f32),
            notes: ActiveValue::NotSet,
            ingested_at: ActiveValue::Set(command.ingestion_date.naive_local()),
            updated_at: ActiveValue::Set(Local::now().naive_local()),
            created_at: ActiveValue::Set(Local::now().naive_local()),
        })
        .exec_with_returning(database_connection)
        .await
        .unwrap();

        event!(Level::INFO, "Ingestion Logged {:?}", &insert_ingestion);

        // Create an Ingestion struct to display
        let ingestion_to_display = IngestionViewModel::from(&insert_ingestion);

        // Create and print the table
        let table = Table::new(vec![ingestion_to_display]);
        println!("{}", table);
    }
}

mod test
{

    #[test]
    fn should_log_ingestion() -> Result<(), Box<dyn std::error::Error>>
    {
        let mut cmd = Command::cargo_bin("neuronek")?;

        let substance_name = "Aspirin";
        let ingestion_date = Local::now().to_string();

        cmd.arg("log-ingestion")
            .arg("-s")
            .arg(substance_name)
            .arg("-v")
            .arg("500")
            .arg("-u")
            .arg("mg")
            .arg("-t")
            .arg(&ingestion_date);

        // Simulate `neuronek` command and ensure it succeeds
        cmd.assert()
            .success()
            .to_string()
            .contains("Ingestion Logged");

        Ok(())
    }
}

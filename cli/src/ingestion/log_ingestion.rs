use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use clap::Subcommand;
use nudb_migration::sea_orm::ActiveValue;
use nudb_migration::sea_orm::EntityTrait;
use tracing::event;
use tracing::instrument;
use tracing::Level;

use super::ingestion::RouteOfAdministrationClassification;

#[derive(Subcommand)]
pub enum IngestionCommands
{
    Log(LogIngestion),
}

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
    // Date of ingestion, by default
    // current date is used if not provided.
    //
    // Date can be provided as timestamp and in human-readable format such as
    // "today 10:00", "yesterday 13:00", "monday 15:34" which will be later
    // parsed into proper timestamp.
    #[arg(
                short='t',
                long,
                default_value_t=Local::now(),
                default_value=Local::now().to_string()
            )]
    pub ingestion_date: DateTime<Local>,
    // #[arg(short = 'r', long)]
    // pub route_of_administration: RouteOfAdministrationClassification,
}

#[instrument(name = "log_ingestion", level = Level::INFO)]
pub fn log_ingestion(command: &LogIngestion)
{
    dbg!("This should log ingestion {:?}", command);

    nudb::ingestion::Entity::insert(nudb::ingestion::ActiveModel {
        id: ActiveValue::default(),
        substance_name: ActiveValue::Set(command.substance_name.to_lowercase()),
        // TODO: Jak zserializowac to gowno?
        route_of_administration: todo!(),
        // TODO: Dodac parsowanie unitow masy i zapisywac informacje w kilogramach, output do uzytkownika powinien byc automatycznie skracany np. 0.0001 do mg czy g.
        dosage: todo!(),
        notes: todo!(),
        ingested_at: todo!(),
        updated_at: todo!(),
        created_at: todo!(),
    });

    event!(Level::INFO, "Ingestion Logged");
}

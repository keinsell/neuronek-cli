#![feature(error_reporter, async_fn_traits)]

use crate::core::Command;
use crate::database::DATABASE_CONNECTION;
use crate::settings::ensure_xdg_directories;
use async_std::task;
use clap::Parser;
use clap::Subcommand;
use db::migration::MigratorTrait;
use miette::set_panic_hook;
use sea_orm_migration::IntoSchemaManagerConnection;

mod core;
mod database;
mod db;
mod humanize;
mod ingestion;
mod logger;
mod settings;
mod ui;

#[derive(Parser)]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    about = "Dosage journal that knows!",
    long_about = "🧬 Intelligent dosage tracker application with purpose to monitor supplements, nootropics and psychoactive substances along with their long-term influence on one's mind and body."
)]
struct CommandLineInterface
{
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count, default_value_t=0)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands
{
    LogIngestion(ingestion::log_ingestion::LogIngestion),
    ListIngestion(ingestion::list_ingestion::ListIngestion),
}

fn main()
{
    // #exception #panic
    // Initialize miette's custom panic hook, so any panics in this program
    // will be displayed with enhanced, readable formatting.
    // This should be set as early as possible, ideally at the start of the program,
    // to ensure that any panics that occur will be formatted by miette.
    set_panic_hook();
    ensure_xdg_directories().expect("XDG Directories could not been created");

    let cli = CommandLineInterface::parse();

    logger::setup_logger(Some(cli.verbosity));

    // TODO(1.3): Setup tracing and telemetry
    // TODO(1.4): Parse and load configuration
    // TODO(1.5): Create, Migrate or Load database

    // #database
    let db_connection = &DATABASE_CONNECTION;

    // #database #database-migration
    async_std::task::block_on(async {
        let pending_migrations = db::migration::Migrator::get_pending_migrations(
            &db_connection.into_schema_manager_connection(),
        )
        .await
        .expect("Failed to read pending migrations");

        if !pending_migrations.is_empty()
        {
            println!("There are {} migrations pending.", pending_migrations.len());
            // TODO: Do prejudicial backup of data
            println!("Applying migrations...");
            db::Migrator::up(db_connection.into_schema_manager_connection(), None)
                .await
                .expect("Failed to apply migrations");
        }
    });

    match &cli.command
    {
        | Some(Commands::LogIngestion(log_ingestion)) =>
        {
            log_ingestion.handle(db_connection).unwrap()
        }
        | Some(Commands::ListIngestion(command)) => task::block_on(command.handle(db_connection)),
        | _ => println!("No command provided"),
    }
}

use crate::database::DATABASE_CONNECTION;
use crate::ingestion::log_ingestion::log_ingestion;
use crate::settings::ensure_xdg_directories;
use clap::Parser;
use clap::Subcommand;
use log::info;
use miette::set_panic_hook;
use nudb_migration::IntoSchemaManagerConnection;
use nudb_migration::MigratorTrait;

mod database;
mod ingestion;
mod logger;
mod settings;

/// Top-level interface definitiion for Neuronek
#[derive(Parser)]
#[command(
    version = "0.0.1-dev",
    about = "Dosage journal that knows!",
    long_about = "🧬 Intelligent dosage tracker application with purpose to monitor supplements, nootropics and psychoactive substances along with their long-term influence on one's mind and body."
)]
struct CommandLineInterface
{
    /// Enable debugging (verbose) information
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count, default_value_t=0)]
    verbosity: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands
{
    LogIngestion(ingestion::log_ingestion::LogIngestion),
    /// does testing things
    Test
    {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main()
{
    let cli = CommandLineInterface::parse();

    // TODO(1): Setup Application Environment
    // TODO(1.1): Setup logger
    logger::setup_logger(Some(cli.verbosity));
    // TODO(1.2): Setup rendering panics with miette for more informative and pretty panics.
    set_panic_hook();
    ensure_xdg_directories().expect("XDG Directories could not been created");
    // TODO(1.3): Setup tracing and telemetry
    // TODO(1.4): Parse and load configuration
    // TODO(1.5): Create, Migrate or Load database

    // TODO(2.0): Setup Database Connection
    let db_connection = &DATABASE_CONNECTION;

    // TODO(2.1): Setup Migration and Ensure Database is up to date
    let handle_migration = async_std::task::spawn(async {
        let pending_migrations = nudb_migration::Migrator::get_pending_migrations(
            &db_connection.into_schema_manager_connection(),
        )
        .await
        .unwrap_or_else(|err| {
            println!("Failed to read pending migrations");
            panic!("{}", err)
        });

        if !pending_migrations.is_empty()
        {
            println!("There are {} migrations pending.", pending_migrations.len());
            // TODO(2.2): Do prejudicial backup of data
            println!("Applying migrations...");
            nudb_migration::Migrator::up(db_connection.into_schema_manager_connection(), None)
                .await
                .unwrap();
        }
    });

    async_std::task::block_on(async {
        handle_migration.await;
    });

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command
    {
        | Some(Commands::LogIngestion(command)) => log_ingestion(command),
        | Some(Commands::Test { list }) =>
        {
            if *list
            {
                println!("Printing testing lists...");
                info!("Hello World")
            }
            else
            {
                println!("Not printing testing lists...");
            }
        }
        | None =>
        {}
    }
}

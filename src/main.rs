use async_std::task;
use miette::set_panic_hook;

mod db {
    use platform_dirs::AppDirs;
    use sea_migrations::*;
    use sea_orm::{prelude::*, Database};

    lazy_static::lazy_static! {
    #[derive(Clone, Copy)]
    static ref DATABASE_URL: String = {
         let data_directory = AppDirs::new(Some("xyz.neuronek.cli"), true)
             .unwrap()
             .data_dir;
         let filename = data_directory.join("db");
         format!("sqlite://{}", filename.display())
     };

    static ref DATABASE_CONNECTION: DatabaseConnection = {
             async_std::task::block_on(async {
     let data_directory = AppDirs::new(Option::from("xyz.neuronek.cli"), true).unwrap().data_dir;
     let filename = data_directory.clone().with_file_name("db");
             let db_url = format!("sqlite://{}", filename.to_str().unwrap());

            println!("Connecting to database at {}", db_url);

                 Database::connect(db_url).await.unwrap()
             })
         };
     }

    pub(super) async fn migrate_database() {
        let pending_migrations = Migrator::get_pending_migrations(&DATABASE_CONNECTION.clone())
            .await
            .unwrap_or_else(|err| {
                println!("Failed to read pending migrations");
                panic!("{}", err)
            });

        if !pending_migrations.is_empty() {
            println!("There are {} migrations pending.", pending_migrations.len());
            println!("Applying migrations...");
            Migrator::up(&DATABASE_CONNECTION.clone(), Option::None)
                .await
                .unwrap();
        } else {
            println!("Everything is up to date!")
        }
    }
}

mod cli {
    use clap::{Parser, Subcommand};
    use std::path::{PathBuf, StripPrefixError};
    use substance::{create_substance, SubstanceCommands};

    pub(super) mod substance {
        use clap::{Error, Parser, Subcommand};
        use sea_orm::DbErr;

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct CreateSubstance {
            #[arg(short, long)]
            name: String,
        }

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct UpdateSubstance {
            #[arg(short, long)]
            id: String,
            #[arg(short, long)]
            pub name: Option<String>,
        }

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct DeleteSubstance {
            #[arg(short, long)]
            pub id: String,
        }

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct ListSubstance {
            #[arg(short = 'l', long)]
            pub limit: String,
        }

        #[derive(Subcommand)]
        pub enum SubstanceCommands {
            Create(CreateSubstance),
            Update(UpdateSubstance),
            Delete(DeleteSubstance),
            List(ListSubstance),
        }

        #[derive(Parser)]
        #[command(args_conflicts_with_subcommands = true)]
        pub(crate) struct SubstanceCommand {
            #[command(subcommand)]
            pub command: SubstanceCommands,
        }

        pub async fn create_substance(create_substance: CreateSubstance) -> Result<String, DbErr> {
            todo!()
        }
    }

    #[derive(Subcommand)]
    pub(super) enum ProgramCommand {
        Substance(substance::SubstanceCommand),
    }

    #[derive(Parser)]
    #[command(
        version = "0.0.1-dev",
        about = "Dosage journal that knows!",
        long_about = "🧬 Intelligent dosage tracker application with purpose to monitor supplements, nootropics and psychoactive substances along with their long-term influence on one's mind and body."
    )]

    pub(super) struct Program {
        /// Optional name to operate on
        pub name: Option<String>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        pub config: Option<PathBuf>,

        /// Turn debugging information on
        #[arg(short, long, action = clap::ArgAction::Count)]
        pub debug: u8,

        #[command(subcommand)]
        pub command: ProgramCommand,
    }

    pub(super) async fn run_program() {
        let cli = Program::parse();

        match cli.command {
            ProgramCommand::Substance(substance_command) => match substance_command.command {
                SubstanceCommands::Create(_command) => {
                    todo!()
                }
                SubstanceCommands::Delete(_delete_substance) => {
                    todo!()
                }
                SubstanceCommands::Update(_update_substance) => {
                    todo!()
                }
                SubstanceCommands::List(_) => todo!(),
            },
        }
    }
}

fn main() {
    // set_hook();
    set_panic_hook();
    task::spawn(async {
        db::migrate_database().await;
    });

    task::block_on(async {
        cli::run_program().await;
    });
}

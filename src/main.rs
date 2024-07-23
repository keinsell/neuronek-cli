use async_std::task;
use miette::set_panic_hook;

mod entities;

mod db {
    use platform_dirs::AppDirs;
    use sea_migrations::Migrator;
    use sea_orm::{Database, DatabaseConnection};
    use sea_orm_migration::prelude::*;
    use std::fs::{self, File};

    fn get_database_uri() -> String {
        let application_directories = AppDirs::new(Some("xyz.neuronek.cli"), true).unwrap();

        dbg!(&application_directories);

        let database_file_path = application_directories.data_dir.join("data.db");
        dbg!(&database_file_path);

        fs::create_dir_all(&application_directories.data_dir).unwrap();

        if !&database_file_path.exists() {
            File::create(&database_file_path).unwrap();
        }

        let db_path = database_file_path.into_os_string().into_string();
        let db_uri = "sqlite://".to_string() + &db_path.unwrap();

        dbg!(&db_uri);

        db_uri
    }

    lazy_static::lazy_static! {
    #[derive(Clone, Debug)]
     pub static ref DATABASE_CONNECTION: DatabaseConnection = {
             async_std::task::block_on(async {
                 let db_url = get_database_uri();
                 dbg!(&db_url);
                 println!("Connecting to database at {:#?}", &db_url);
                 Database::connect(db_url).await.unwrap()
             })
         };
     }

    pub(super) async fn migrate_database(database_connection: &DatabaseConnection) {
        let pending_migrations =
            Migrator::get_pending_migrations(&database_connection.into_schema_manager_connection())
                .await
                .unwrap_or_else(|err| {
                    println!("Failed to read pending migrations");
                    panic!("{}", err)
                });

        if !pending_migrations.is_empty() {
            println!("There are {} migrations pending.", pending_migrations.len());
            println!("Applying migrations...");
            Migrator::up(database_connection.into_schema_manager_connection(), None)
                .await
                .unwrap();
        } else {
            println!("Everything is up to date!")
        }
    }
}

mod cli {
    use clap::{Parser, Subcommand};
    use std::{ops::Deref, path::PathBuf};

    use crate::db;

    pub(super) mod substance {
        use std::borrow::Borrow;

        use crate::entities::{self, substance};
        use clap::{Parser, Subcommand};
        use sea_orm::{
            ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, TryIntoModel,
        };

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct CreateSubstance {
            #[arg(short, long)]
            pub name: String,
        }

        #[derive(Parser, Debug)]
        #[command(version,about,long_about=None)]
        pub struct UpdateSubstance {
            #[arg(short, long)]
            pub id: i32,
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
            #[arg(short = 'l', long, default_value_t = 10)]
            pub limit: u64,
            #[arg(short = 'p', long, default_value_t = 0)]
            pub page: u64,
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

        pub async fn create_substance(
            create_substance_command: CreateSubstance,
            db_conn: &DatabaseConnection,
        ) -> Result<entities::substance::Model, DbErr> {
            let substance_active_model = entities::substance::ActiveModel {
                name: sea_orm::ActiveValue::set(create_substance_command.name),
                ..Default::default()
            };
            let substance_model = substance_active_model.insert(db_conn).await.unwrap();
            substance_model.try_into_model()
        }

        pub async fn update_substance(
            update_substance: UpdateSubstance,
            db_conn: &DatabaseConnection,
        ) -> Result<entities::substance::Model, DbErr> {
            let active_model = entities::substance::ActiveModel {
                id: Set(update_substance.id),
                name: update_substance
                    .name
                    .map(ActiveValue::set)
                    .unwrap_or(ActiveValue::not_set()),
                // ..Default::default()
            };

            active_model.update(db_conn).await.map_err(|err| {
                println!("{}", err);
                err
            })
        }

        pub async fn execute_substance_command(
            command: SubstanceCommands,
            database_connection: &DatabaseConnection,
        ) {
            match command {
                SubstanceCommands::Create(payload) => {
                    create_substance(payload, database_connection)
                        .await
                        .expect("Should create substance");
                }
                SubstanceCommands::Update(_) => todo!(),
                SubstanceCommands::Delete(_) => todo!(),
                SubstanceCommands::List(query) => {
                    list_substances(query, database_connection).await;
                }
            }
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
            ProgramCommand::Substance(substance_command) => {
                substance::execute_substance_command(
                    substance_command.command,
                    db::DATABASE_CONNECTION.deref(),
                )
                .await;
            }
        }
    }
}

fn main() {
    // set_hook();
    set_panic_hook();

    task::block_on(async {
        db::migrate_database(&db::DATABASE_CONNECTION).await;
        cli::run_program().await;
    });
}

#[cfg(test)]
mod tests {
    use self::cli::substance::{update_substance, CreateSubstance};
    use super::*;
    use crate::cli::substance::{create_substance, CreateSubstance};
    use sea_orm::sea_query::TableCreateStatement;
    use sea_orm::{
        ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DbBackend, MockDatabase,
        MockExecResult, Schema,
    };

    /// Utility to use a database that behaves like a real one
    /// instead mock-up in which we know inputs and outputs.
    async fn use_memory_sqlite() -> DatabaseConnection {
        Database::connect("sqlite::memory:").await.unwrap()
    }

    async fn setup_schema(db: &DatabaseConnection) {
        let schema = Schema::new(DbBackend::Sqlite);
        let stmt: TableCreateStatement =
            schema.create_table_from_entity(entities::substance::Entity);
        let _result = db.execute(db.get_database_backend().build(&stmt)).await;
    }

    #[async_std::test]
    async fn test_create_substance() {
        let caffeine_fixture = entities::substance::Model {
            id: 1,
            name: "caffeine".to_owned(),
        };

        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        let command = CreateSubstance {
            name: "caffeine".to_string(),
        };

        let result = create_substance(command, &db).await;
        assert!(result.is_ok());

        let substance = result.unwrap();
        assert_eq!(substance, caffeine_fixture);
    }

    #[async_std::test]
    async fn test_create_substance_with_mock() {
        let caffeine_fixture = entities::substance::Model {
            id: 78,
            name: "caffeine".to_owned(),
        };

        // Create a mock in-memory SQLite database
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[caffeine_fixture.clone()]])
            .append_exec_results([MockExecResult {
                last_insert_id: 78,
                rows_affected: 1,
            }])
            .into_connection();

        // Create the command to create a substance
        let command = CreateSubstance {
            name: "Caffeine".to_string(),
        };

        // Call the create_substance function with the command and the reference to the database
        let result = create_substance(command, &db).await;
        assert!(result.is_ok());

        let substance = result.unwrap();
        assert_eq!(substance, caffeine_fixture);
    }

    #[async_std::test]
    async fn test_update_substance_should_fail() {
        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        let command = UpdateSubstance {
            id: 1,
            name: Option::from("Coffee".to_string()),
        };

        let result = update_substance(command, &db).await;
        assert!(result.is_err());
    }

    #[async_std::test]
    async fn test_update_substance() {
        let caffeine_fixture = entities::substance::Model {
            id: 1,
            name: "caffeine".to_owned(),
        };

        let db = use_memory_sqlite().await;
        setup_schema(&db).await;

        create_substance(
            CreateSubstance {
                name: caffeine_fixture.name.clone(),
            },
            &db,
        )
        .await
        .expect("Substance should be created");

        let command = UpdateSubstance {
            id: 1,
            name: Option::from("Coffee".to_string()),
        };

        let result = update_substance(command, &db).await;
        assert!(result.is_ok());

        let substance = result.unwrap();
        assert_eq!(
            substance,
            substance::Model {
                id: 1,
                name: "Coffee".to_owned()
            }
        );
    }
}

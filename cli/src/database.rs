use crate::settings::SETTINGS;
use nudb_migration::sea_orm::Database;
use nudb_migration::sea_orm::DatabaseBackend::Sqlite;
use nudb_migration::sea_orm::DatabaseConnection;
use nudb_migration::sea_orm::MockDatabase;

lazy_static::lazy_static! {
#[derive(Clone, Debug)]
 pub static ref DATABASE_CONNECTION: DatabaseConnection = {
         async_std::task::block_on(async {
             let db_url = SETTINGS.sqlite_uri.clone();
             println!("Connecting to database at {:#?}", &db_url);

            if cfg!(test) {
                return MockDatabase::new(Sqlite).into_connection()
            }

             Database::connect(db_url).await.unwrap()
         })
     };
 }

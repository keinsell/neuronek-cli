use crate::settings::SETTINGS;
use log::info;
use sea_orm::Database;
use sea_orm::DatabaseBackend::Sqlite;
use sea_orm::DatabaseConnection;
use sea_orm::MockDatabase;

lazy_static::lazy_static! {
#[derive(Clone, Debug)]
 pub static ref DATABASE_CONNECTION: DatabaseConnection = {
         async_std::task::block_on(async {
             let db_url = SETTINGS.sqlite_uri.clone();
            info!("🗄️ Database: {:#?}", &db_url);

            if cfg!(test) {
                return MockDatabase::new(Sqlite).into_connection()
            }

             Database::connect(db_url).await.unwrap()
         })
     };
 }

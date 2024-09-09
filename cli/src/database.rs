use crate::settings::SETTINGS;
use nudb_migration::sea_orm::Database;
use nudb_migration::sea_orm::DatabaseConnection;

lazy_static::lazy_static! {
#[derive(Clone, Debug)]
 pub static ref DATABASE_CONNECTION: DatabaseConnection = {
         async_std::task::block_on(async {
             let db_url = SETTINGS.sqlite_uri.clone();
             dbg!(&db_url);
             println!("Connecting to database at {:#?}", &db_url);
             Database::connect(db_url).await.unwrap()
         })
     };
 }

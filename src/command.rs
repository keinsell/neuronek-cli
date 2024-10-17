use miette::Error;
use miette::Result;
use nudb_migration::sea_orm::DatabaseConnection;

pub trait Command
{
    fn handle(&self, db: &DatabaseConnection) -> Result<(), Error>;
}

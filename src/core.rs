use miette::Error;
use sea_orm::DatabaseConnection;

pub trait Command
{
    fn handle(&self, db: &DatabaseConnection) -> miette::Result<(), Error>;
}

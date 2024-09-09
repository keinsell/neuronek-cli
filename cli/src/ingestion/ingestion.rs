use serde::Deserialize;
use serde::Serialize;

// There are two views of ingestion, the one is a fully detailed analytics and second one is
// simplified record from database.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RouteOfAdministrationClassification
{
    Oral,
}

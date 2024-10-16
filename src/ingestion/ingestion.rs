// There are two views of ingestion, the one is a fully detailed analytics and second one is
// simplified record from database.

use crate::db;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono_humanize::HumanTime;
use tabled::Tabled;

pub(super) type Entity = db::ingestion::Model;

#[derive(Tabled, Debug, Clone)]
pub struct IngestionViewModel
{
    pub(crate) id: i32,
    pub(crate) substance_name: String,
    pub(crate) route_of_administration: String,
    pub(crate) dosage: f32,
    #[tabled(display_with = "option_display")]
    pub(crate) notes: Option<String>,
    #[tabled(display_with = "human_time_display")]
    pub(crate) ingested_at: chrono::NaiveDateTime,
    #[tabled(display_with = "human_time_display")]
    pub(crate) updated_at: chrono::NaiveDateTime,
    #[tabled(display_with = "human_time_display")]
    pub(crate) created_at: chrono::NaiveDateTime,
}

impl From<&Entity> for IngestionViewModel
{
    fn from(value: &Entity) -> Self
    {
        IngestionViewModel {
            id: value.id,
            substance_name: value.substance_name.clone(),
            route_of_administration: value.route_of_administration.clone(),
            dosage: value.dosage,
            notes: value.notes.clone(),
            ingested_at: value.ingested_at.naive_local(),
            updated_at: value.updated_at.naive_local(),
            created_at: value.created_at.naive_local(),
        }
    }
}

fn option_display(opt: &Option<String>) -> String
{
    match opt
    {
        | Some(s) => s.clone(),
        | None => "".to_string(),
    }
}

fn human_time_display(dt: &NaiveDateTime) -> String
{
    HumanTime::from(Local.from_local_datetime(dt).unwrap()).to_string()
}

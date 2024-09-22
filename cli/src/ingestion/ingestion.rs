// There are two views of ingestion, the one is a fully detailed analytics and second one is
// simplified record from database.

use chrono::Local;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono_humanize::HumanTime;
use tabled::Tabled;

#[derive(Tabled, Debug)]
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

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
pub struct LogIngestion {
    #[arg(short, long)]
    pub substance_name: String,
    #[arg(short = 'u', long, default_value_t=String::from("mg"))]
    pub dosage_unit: String,
    #[arg(short = 'v', long)]
    pub dosage_amount: f64,
    /// Date of ingestion, by default
    /// current date is used if not provided.
    ///
    /// Date can be provided as timestamp and in human-readable format such as
    /// "today 10:00", "yesterday 13:00", "monday 15:34" which will be later
    /// parsed into proper timestamp.
    #[arg(
                short='t',
                long,
                value_parser=parse_humanized_date,
                default_value_t=Local::now(),
                default_value="now"
            )]
    pub ingestion_date: DateTime<Local>,
}

pub fn log_ingestion(command: LogIngestion) {
    dbg!("This should log ingestion {:?}", command);
    todo!()
}

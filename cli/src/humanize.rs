use chrono::DateTime;
use chrono::Local;
use parse_datetime::parse_datetime;

pub fn human_date_parser(s: &str) -> Result<DateTime<Local>, String>
{
    parse_datetime(s)
        .map_err(|e| format!("Failed to parse date: {}", e))
        .map(|dt| {
            let now = Local::now();
            if dt.date_naive() == now.date_naive()
            {
                DateTime::from(dt)
            }
            else
            {
                now
            }
        })
}

#[test]
fn test_human_date_parser_today()
{
    use chrono::Local;
    let input = "today";
    let now = Local::now().naive_local().date();
    let human_time = human_date_parser(input).unwrap().naive_local().date();
    assert_eq!(human_time, now);
}

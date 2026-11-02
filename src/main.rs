use chrono::{Local, NaiveDate};

fn main() {
    let date = NaiveDate::parse_from_str("2026-12-28", "%Y-%m-%d").unwrap();
    println!("{}", get_day_from_now(date));
}

fn get_day_from_now(date: NaiveDate) -> i64 {
    let now = Local::now().date_naive();
    (date - now).num_days()
}

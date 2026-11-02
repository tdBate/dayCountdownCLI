use chrono::{Local, NaiveDate};

fn main() {
    let date = NaiveDate::parse_from_str("2026-03-28", "%Y-%m-%d").unwrap();
    let now = Local::now().date_naive();
    println!("{}", (date - now).num_days() + 365);
}

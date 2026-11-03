use std::io::{self, Write};

use chrono::{Local, NaiveDate};

struct Items {
    data: Vec<CountdownItem>,
    next_id: u32,
}

#[derive(Clone)]
struct CountdownItem {
    id: u32,
    title: String,
    date: NaiveDate,
}

fn main() {
    let date = NaiveDate::parse_from_str("2026-12-28", "%Y-%m-%d").unwrap();

    let mut items: Items = Items {
        data: vec![],
        next_id: 0,
    };

    let item = CountdownItem {
        id: 0,
        title: String::from("asd"),
        date: date.clone(),
    };

    items.add_item(item.clone());
    items.add_item(item.clone());

    items.delete_item(0);

    for a in items.data {
        a.print_state();
    }

    propmt_item();
}

fn propmt_item() {
    print!("Enter title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("failed to readline");
    println!("");

    let mut date_string: String = String::from("");
    print!("Enter date: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut date_string)
        .expect("failed to readline");
    println!("");

    let date = match NaiveDate::parse_from_str(&date_string.trim(), "%Y-%m-%d") {
        Ok(a) => a,
        Err(e) => {
            println!("Error{}", e);
            return;
        }
    };

    let instance = CountdownItem {
        id: 0,
        title: title.trim().to_string(),
        date: date,
    };
}

impl Items {
    fn add_item(&mut self, mut item: CountdownItem) {
        item.id = self.next_id;
        self.next_id += 1;
        self.data.push(item);
    }

    fn delete_item(&mut self, id: u32) -> Option<CountdownItem> {
        let index = self.data.iter().position(|item| item.id == id)?;
        Some(self.data.remove(index))
    }
}

impl CountdownItem {
    fn get_day_from_now(&self) -> i64 {
        let then = self.date;
        let now = Local::now().date_naive();
        (then - now).num_days()
    }

    fn print_state(&self) {
        println!("{}-{} , {}", self.title, self.id, self.get_day_from_now());
    }
}

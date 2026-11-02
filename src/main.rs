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

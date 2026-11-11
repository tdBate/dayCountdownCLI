use chrono::{Local, NaiveDate};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    sync::LazyLock,
};

static FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let save_dir = Path::new("data");
    save_dir.join("data.json")
});

#[derive(Parser, Debug)]
#[command(version, about = "A date countdown cli app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    ///shows all the recorded countdown items
    Show {},

    ///adds an item
    Add {},
}

#[derive(Serialize, Deserialize)]
struct Items {
    data: Vec<CountdownItem>,
    next_id: u32,
}

#[derive(Clone, Serialize, Deserialize)]
struct CountdownItem {
    id: u32,
    title: String,
    date: NaiveDate,
}

fn main() {
    let file_content = fs::read_to_string(&*FILE_PATH).unwrap();
    let mut items = Items {
        data: serde_json::from_str(file_content.as_str()).unwrap(),
        next_id: 0,
    };

    let cli = Cli::parse();

    match &cli.command {
        Commands::Show {} => {
            items.print_data();
        }
        Commands::Add {} => {
            items.add_item(propmt_item());
            items.save();
        }
    }
}

fn propmt_item() -> CountdownItem {
    print!("Enter title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("failed to readline");
    println!("");

    let date = loop {
        let mut date_string: String = String::from("");
        print!("Enter date: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut date_string)
            .expect("failed to readline");
        println!("");

        match NaiveDate::parse_from_str(&date_string.trim(), "%Y-%m-%d") {
            Ok(a) => break a,
            Err(e) => {
                println!("Wrong format try: (YYYY-MM-DD)");
                continue;
            }
        };
    };

    CountdownItem {
        id: 0,
        title: title.trim().to_string(),
        date: date,
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

    fn save(&self) {
        fs::write(
            &*FILE_PATH,
            serde_json::to_string_pretty(&self.data).unwrap(),
        );
    }

    fn print_data(&self) {
        println!("Data:");
        self.data
            .iter()
            .for_each(|item| println!("  {}", item.to_string()));
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

    fn to_string(&self) -> String {
        String::from(format!("{} {}", self.title, self.get_day_from_now()))
    }
}

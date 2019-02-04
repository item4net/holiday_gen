extern crate chrono;
extern crate csv;
extern crate serde_json;

use chrono::NaiveDate;
use csv::Reader;

use std::fs::{File, create_dir_all};
use std::vec::Vec;
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    for year in 2018..2020 {
        let mut db: HashMap<NaiveDate, Vec<String>> = HashMap::new();
        let mut reader = Reader::from_path(format!("./data/{}.csv", year))?;
        let none: Vec<String> = vec![];
        for result in reader.records() {
            let record = result?;
            let date = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;
            let column = db.entry(date).or_insert(Vec::new());
            (*column).push(record[1].to_owned());
        }

        for ordinal in 1..368 {
            match NaiveDate::from_yo_opt(year, ordinal) {
                Some(date) => {
                    let dir = date.format("holiday/%Y/%m/%d").to_string();
                    let file = format!("{}/index.json", dir);
                    create_dir_all(dir)?;
                    let mut file = File::create(file)?;
                    serde_json::to_writer(&mut file, match db.get(&date) {
                        Some(names) => names,
                        None => &none,
                    })?;
                },
                None => break,
            }
        }
    }
    Ok(())
}

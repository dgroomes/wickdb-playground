use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde::Deserialize;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let summary = count_lines(file);

    println!("Number of ZIP areas: {}", summary.zip_areas);
    println!("Total population: {}", summary.population);

    Ok(())
}

fn count_lines(file: File) -> Summary {
    let reader = BufReader::new(file);

    let mut population: u64 = 0;
    let mut zip_areas: u32 = 0;
    for line in reader.lines() {
        match line {
            Ok(json) => {
                let zip_area: ZipArea = serde_json::from_str(&json).unwrap();
                population = population + zip_area.pop;
                zip_areas = zip_areas + 1;
            }
            Err(_) => panic!("Found error while reading a line from the file")
        }
    }

    Summary { zip_areas, population }
}

pub struct Summary {
    pub zip_areas: u32,
    pub population: u64,
}

#[derive(Debug, Deserialize)]
struct ZipArea {
    city: String,
    state: String,
    pop: u64,
}

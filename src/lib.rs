use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde::Deserialize;
use wickdb::file::FileStorage;
use wickdb::{BytewiseComparator, LevelFilter, Options, ReadOptions, WickDB, WriteOptions, DB};
use std::env::temp_dir;

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

const AGAWAM_MA_01001_ZIP_CODE: &str = "01001";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let summary = count_lines(file);

    println!("Number of ZIP areas: {}", summary.zip_areas);
    println!("Total population: {}", summary.population);

    // Create an embedded wickdb storage file (what is the correct word?) and insert some data
    let mut options = Options::<BytewiseComparator>::default();
    options.logger_level = LevelFilter::Debug;
    let dir = temp_dir().join("test_zips_wickdb");
    let db = WickDB::open_db(options, &dir, FileStorage::default()).unwrap();

    let _01001_zip_data = r#"{ "_id" : "01001", "city" : "AGAWAM", "loc" : [ -72.622739, 42.070206 ], "pop" : 15338, "state" : "MA" }"#;

    db.put(WriteOptions::default(), AGAWAM_MA_01001_ZIP_CODE.as_bytes(), _01001_zip_data.as_bytes())
        .expect("Failed to put a ZIP area entry into wickdb");

    // Read the entry back out of wickdb
    // I don't know yet how to make this a string to print it...
    let found = db.get(ReadOptions::default(), AGAWAM_MA_01001_ZIP_CODE.as_bytes())
        .expect("Failed to read a ZIP area entry from wickdb")
        .expect("Record read from wickdb was empty");

    // Convert the raw bytes to a string
    let stringified = &mut String::new();
    found.as_slice().read_to_string(stringified);

    println!("Successfully read the ZIP area data from wickdb. {}", stringified);

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

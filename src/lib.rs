use std::env::temp_dir;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use wickdb::file::FileStorage;
use wickdb::{BytewiseComparator, LevelFilter, Options, ReadOptions, WickDB, WriteOptions, DB};

use zip_file::ZipAreaFileSource;

mod zip;
mod zip_file;

pub struct Config {
    pub filename: String,
    pub log_level: LevelFilter,
}

const AGAWAM_MA_01001_ZIP_CODE: &str = "01001";

/// Run the program. Editorialization: a function called "run" is probably at home in "main.rs" not
/// "lib.rs".
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Summarizing ZIP code data from the file '{}'...",
        config.filename
    );

    let file = File::open(config.filename).expect("File could not be opened.");
    let summary = count_lines(file);

    println!("Number of ZIP areas: {}", summary.zip_areas);
    println!("Total population: {}", summary.population);

    // Create an embedded wickdb storage file (what is the correct word?) and insert some data
    let mut options = Options::<BytewiseComparator>::default();
    options.logger_level = config.log_level;
    let dir = temp_dir().join("test_zips_wickdb");
    let db = WickDB::open_db(options, &dir, FileStorage::default()).unwrap();

    let _01001_zip_data = r#"{ "_id" : "01001", "city" : "AGAWAM", "loc" : [ -72.622739, 42.070206 ], "pop" : 15338, "state" : "MA" }"#;

    db.put(
        WriteOptions::default(),
        AGAWAM_MA_01001_ZIP_CODE.as_bytes(),
        _01001_zip_data.as_bytes(),
    )
    .expect("Failed to put a ZIP area entry into wickdb");

    // Read the entry back out of wickdb and convert it to a string
    let stringified = &mut String::new();
    db.get(ReadOptions::default(), AGAWAM_MA_01001_ZIP_CODE.as_bytes())
        .expect("Failed to read a ZIP area entry from wickdb")
        .expect("Record read from wickdb was empty")
        .as_slice()
        .read_to_string(stringified)
        .expect("Failed to serialize raw bytes to string");

    println!(
        "Successfully read the ZIP area data from wickdb. {}",
        stringified
    );

    Ok(())
}

fn count_lines(file: File) -> Summary {
    let zip_file_source = ZipAreaFileSource::new(file);
    let mut population: u32 = 0;
    let mut zip_areas: u32 = 0;
    for zip_area in zip_file_source {
        population = population + zip_area.pop;
        zip_areas = zip_areas + 1;
    }

    Summary {
        zip_areas,
        population,
    }
}

pub struct Summary {
    pub zip_areas: u32,
    pub population: u32,
}

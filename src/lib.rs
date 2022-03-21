extern crate core;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use wickdb::file::FileStorage;
use wickdb::{BytewiseComparator, Iterator, LevelFilter, ReadOptions, WickDB, WriteOptions, DB};

use zip_file::ZipAreaFileSource;

use crate::zip_json::ZipAreaJsonRecord;

mod db;
mod zip;
mod zip_file;
mod zip_json;

pub struct Config {
    pub filename: String,
    pub log_level: LevelFilter,
}

/// Run the program. Editorialization: a function called "run" is probably at home in "main.rs" not
/// "lib.rs".
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Create an embedded wickdb storage file (what is the correct word?) and insert some data
    let db = db::create(config.log_level);

    ingest_from_file(config, &db);

    let summary = summarize(db);

    println!("Number of ZIP areas: {}", summary.zip_areas);
    println!("Total population: {}", summary.population);

    Ok(())
}

/// Ingest all ZIP code data from the JSON file into the wickdb embedded database.
fn ingest_from_file(config: Config, db: &WickDB<FileStorage, BytewiseComparator>) {
    println!(
        "Ingesting ZIP code data from the file '{}'...",
        config.filename
    );

    let file = File::open(config.filename).expect("File could not be opened.");
    let zip_file_source = ZipAreaFileSource::new(file);

    for (json, zip_area) in zip_file_source {
        // Put the raw JSON as the value and the ZIP code as the key.
        //
        // Remember, wickdb only knows about bytes! It doesn't have a concept of data types like
        // integers, characters, or strings. So, we have to think in terms of bytes when putting and
        // getting data to and from wickdb.
        let key = zip_area.code.to_be_bytes();
        let value = json.as_bytes();
        db.put(WriteOptions::default(), &key, value)
            .expect("Failed to put a ZIP area entry into wickdb");
    }
}

/// Read all ZIP records from the wickdb database and summarize the data.
fn summarize(db: WickDB<FileStorage, BytewiseComparator>) -> Summary {
    println!("Summarizing ZIP code data from the wickdb embedded database...");
    let mut population: u32 = 0;
    let mut zip_areas: u32 = 0;
    let mut iter = db
        .iter(ReadOptions::default())
        .expect("Failed to get iterator over the wickdb database");
    iter.seek_to_first();

    // I'm not really sure why this is necessary, but I get an off-by-one error when I don't call
    // "next" here. I thought it would be enough to call "next" in the while loop, but I think the
    // first ZIP area gets double counted unless I do this.
    iter.next();

    while iter.valid() {
        // Read the entry back out of wickdb and convert it to a string
        let json = &mut String::new();
        iter.value()
            .read_to_string(json)
            .expect("Failed to serialize raw bytes to string");

        let zip_area_json: ZipAreaJsonRecord = serde_json::from_str(json).expect(
            "Failed to deserialize a wickdb value. Deserializing from JSON to ZipAreaJsonRecord",
        );
        let zip_area = zip_area_json.domain();

        population = population + zip_area.pop;
        zip_areas = zip_areas + 1;
        iter.next() // There should be a proper Rust iterator for wickdb but couldn't find it immediately. This worked.
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

use std::{env, process};
use std::str::FromStr;

use wickdb::LevelFilter;

use wickdb_playground::{ingest_from_file, summarize};

mod db;

/// Read ZIP code data from a JSON file, insert it into an embedded wickdb database, and summarize
/// the ZIP population data.
///
/// The style of this 'main' method is mostly taken from https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Too few arguments. Usage: cargo run <zip code file> <log level>");
        process::exit(1);
    }

    let filename = args[1].clone();
    let log_level = args[2].clone();
    let log_level =
        LevelFilter::from_str(&log_level).expect("Could not parse log level from string.");

    // Create an embedded wickdb storage file (what is the correct word?) and insert some data
    let db = db::create(log_level);

    ingest_from_file(filename, &db);

    let summary = summarize(db);

    println!("Number of ZIP areas: {}", summary.zip_areas);
    println!("Total population: {}", summary.population);
}

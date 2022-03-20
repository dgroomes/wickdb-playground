use std::str::FromStr;
use std::{env, process};

use wickdb::LevelFilter;

use wickdb_playground::run;
use wickdb_playground::Config;

/*
  Mostly taken from https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
  Also, I don't know how to write idiomatic comments in Rust!

  Execute the program from the read_file/ root with `cargo run zips.json`
*/
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Counting lines in file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

/*
In my opinion, main.rs should be responsible for turning program arguments into "prepared configuration"
data. If main.rs is so thin that it doesn't even do this work, that's a waste.
 */
pub fn parse_config(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
        return Err("Too few arguments. Usage: cargo run <zip code file> <log level>");
    }

    let filename = args[1].clone();
    let log_level = args[2].clone();
    let log_level =
        LevelFilter::from_str(&log_level).expect("Could not parse log level from string.");

    Ok(Config {
        filename,
        log_level,
    })
}

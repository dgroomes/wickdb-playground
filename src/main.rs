use std::{env, process};

use wickdb_playground::Config;
use wickdb_playground::run;

/*
  Mostly taken from https://doc.rust-lang.org/stable/book/ch12-03-improving-error-handling-and-modularity.html
  Also, I don't know how to write idiomatic comments in Rust!

  Execute the program from the read_file/ root with `cargo run zips.json`
*/
fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Counting lines in file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

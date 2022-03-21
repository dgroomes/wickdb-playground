# wickdb-playground

📚 Learning and exploring the pure Rust LSM-tree embedded storage engine [`wickdb`](https://github.com/Fullstop000/wickdb).

**NOTE**: This project was developed on macOS and for my own personal use.

## Instructions

Follow these instructions to build and run the program.

1. Build and run
   * `cargo run zips.json WARN`
   * Altogether, it should look something like this:
     ```text
     ❯ cargo run zips.json WARN
        Compiling wickdb-playground v0.1.0 (/Users/davidgroomes/repos/personal/wickdb-playground)
         Finished dev [unoptimized + debuginfo] target(s) in 0.98s
          Running `target/debug/wickdb-playground zips.json WARN`
     Ingesting ZIP code data from the file 'zips.json'...
     Summarizing ZIP code data from the wickdb embedded database...
     Number of ZIP areas: 29353
     Total population: 248408400
     ```
2. Try with more granular logging 
   * `cargo run zips.json DEBUG`
   * Notice how much information is logged from wickdb. Read the logs and build your understanding of what wickdb is
     doing. 
3. Experiment!
   * Change some code, build and run it, and continue experimenting. Occasionally, run the formatter so the source code
     adheres closer to idiomatic Rust. Use the following command.
   * `cargo fmt`

## Wish list

General clean ups, TODOs and things I wish to implement for this project:

* DONE I'm not sure how to consume wickdb in my project because it doesn't look like it's published as a library crate in crates.io.
  I've found a crate called ["lsm"](https://github.com/kaimast/lsm-rs) when I search using `cargo search wickdb`. Can I
  build wickdb locally to a local Crates repository, like I would in the Java world with Gradle/Maven? I also though about
  cloning wickdb into this repo but I don't to end up with a jumble of Git commit history. I would prefer to have a clean
  fork that is separate from this repo. By clean I mean very little source changes from the upstream, so that it's easy to
  sync the fork in the future.
  * Update: I have decided to try a submodule again. The Rust way to depend on other unpublished packages is to use relative
    paths to these packages on your filesystem. So, I don't really want my Cargo.toml to depends on something like `../my-cloned-open-source-repos/wickdb`
    because that would be custom to my current system and wouldn't work on any other computer. So if I add it as a submodule
    instead, then I can depend on it with a path that will always work: `wickdb`.
* Implement everything!
* DONE Write all ZIP area records to the wickdb datastore
* DONE Parameterize the wickdb log level as a program argument. Debug logs don't work well for a first demo
* Format numbers for US localization (the number 1234 should format as "1,234"). I see two third-party crates for this
  ("thousands" and "num_fmt") and they are both updated last 2019. I'd rather not depend on these for a demo app like
  this. This is a weak spot for Rust compared to most AAA languages. In Java, formatting a number is as easy as `NumberFormat.getInstance().format(myNumber);`
  in the easy case.

## Reference

* [GitHub repo: *wickdb*](https://github.com/Fullstop000/wickdb)
* [GitHub repo: *mongodb-playground*](https://github.com/dgroomes/mongodb-playground)
  * This is my own repo where I have sample programs that use a ZIP code data set from the official MongoDB docs. I've
    found that ZIP code data is intuitive and effective when playing with databases–or, more generally–*storage engines*
    like wickdb.
* [*Designing Data-Intensive Applications: Chapter 3. Storage and Retrieval*](https://learning.oreilly.com/library/view/designing-data-intensive-applications/9781491903063/ch03.html)
  * I highly recommend this book and specifically this chapter. It concisely describes Sorted String Tables (SSTables),
    Log-Structured Merge-Trees (LSM-Tree) and B-Trees ([the B is ambiguous](https://stackoverflow.com/a/2263867)) and then
    compares and contrasts their core advantages.

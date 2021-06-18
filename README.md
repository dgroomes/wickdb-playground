# wickdb-playground

WORK IN PROGRESS

📚 Learning and exploring the pure Rust LSM-tree embedded storage engine [`wickdb`](https://github.com/Fullstop000/wickdb).

---
**NOTE**:

This project was developed on macOS and for my own personal use.

---

## Instructions

* Build and run
  * `cargo run zips.json`

## Wish list

General clean ups, TODOs and things I wish to implement for this project:

* I'm not sure how to consume wickdb in my project because it doesn't look like it's published as a library crate in crates.io.
  I've found a crate called ["lsm"](https://github.com/kaimast/lsm-rs) when I search using `cargo search wickdb`. Can I
  build wickdb locally to a local Crates repository, like I would in the Java world with Gradle/Maven? I also though about
  cloning wickdb into this repo but I don't to end up with a jumble of Git commit history. I would prefer to have a clean
  fork that is separate from this repo. By clean I mean very little source changes from the upstream, so that it's easy to
  sync the fork in the future.
* Implement everything!

## Reference

* [GitHub repo: *wickdb*](https://github.com/Fullstop000/wickdb)
* [GitHub repo: *mongodb-playground*](https://github.com/dgroomes/mongodb-playground)
  * This is my own repo where I have sample programs that use a ZIP code data set from the official MongoDB docs. I've
    found that ZIP code data is intuitive and effective when playing with databases–or, more generally–*storage engines*
    like wickdb.

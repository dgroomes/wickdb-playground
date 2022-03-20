use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use crate::zip::ZipArea;

/// This is an intermediate representation of the ZIP area domain type. It's used during JSON
/// (de)serialization. It represents the schema of the ZIP code JSON data that we expect in the JSON
/// file.
///
/// For example:
/// ```json
/// { "_id" : "01001", "city" : "AGAWAM", "loc" : [ -72.622739, 42.070206 ], "pop" : 15338, "state" : "MA" }
/// ```
#[derive(Debug, Deserialize, Serialize)]
struct ZipAreaJsonRecord {
    _id: String,
    city: String,
    state: String,
    pop: u32,
}

impl ZipAreaJsonRecord {
    /// Convert the intermediate type to the proper domain type
    fn domain(&self) -> ZipArea {
        ZipArea {
            code: self
                ._id
                .parse()
                .expect("Failed to parse ZIP code (string to integer)"),
            city: self.city.clone(),
            state: self.state.clone(),
            pop: self.pop,
        }
    }
}

// A struct that represents the file containing the original raw ZIP code data.
#[allow(dead_code)]
pub struct ZipAreaFileSource {
    lines: Lines<BufReader<File>>,
}

impl ZipAreaFileSource {
    pub fn new(file: File) -> ZipAreaFileSource {
        let reader = BufReader::new(file);
        let lines = reader.lines();
        ZipAreaFileSource { lines }
    }
}

// A convenient iterator over the ZIP area file source. This handles deserialization from JSON.
impl Iterator for ZipAreaFileSource {
    type Item = ZipArea;

    fn next(&mut self) -> Option<Self::Item> {
        let option = self.lines.next();
        let return_option: Option<ZipArea> = match option {
            None => Option::None,
            Some(Result::Ok(json)) => {
                let zip_area_json_record: ZipAreaJsonRecord = serde_json::from_str(&json)
                    .expect("Failed to deserialize ZIP area JSON document");
                let zip_area = zip_area_json_record.domain();
                Option::Some(zip_area)
            }
            Some(Err(_)) => {
                panic!("Error while reading lines from the zips.json file")
            }
        };

        return return_option;
    }
}

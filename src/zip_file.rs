use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use crate::zip::ZipArea;

// A struct that represents the file containing the original raw ZIP code data.
#[allow(dead_code)]
pub struct ZipAreaFileSource {
    lines: Lines<BufReader<File>>
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
            None => {
                Option::None
            }
            Some(Result::Ok(json)) => {
                let zip_area = serde_json::from_str(&json).expect("Failed to deserialize ZIP area JSON document");
                Option::Some(zip_area)
            }
            Some(Err(_)) => {
                panic!("Error while reading lines from the zips.json file")
            }
        };

        return return_option;
    }
}

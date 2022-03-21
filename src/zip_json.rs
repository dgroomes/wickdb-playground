use serde::Deserialize;
use serde::Serialize;

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
pub struct ZipAreaJsonRecord {
    pub _id: String,
    pub city: String,
    pub state: String,
    pub pop: u32,
}

impl ZipAreaJsonRecord {
    /// Convert the intermediate type to the proper domain type
    pub(crate) fn domain(&self) -> ZipArea {
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

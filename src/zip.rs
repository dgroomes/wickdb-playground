use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ZipArea {
    pub city: String,
    pub state: String,
    pub pop: u64,
}

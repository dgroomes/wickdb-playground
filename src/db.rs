use std::env::temp_dir;

use wickdb::db::WickDB;
use wickdb::options::Options;
use wickdb::storage::file::FileStorage;
use wickdb::BytewiseComparator;

use crate::LevelFilter;

/// Create a wickdb embedded database
pub fn create(log_level: LevelFilter) -> WickDB<FileStorage, BytewiseComparator> {
    let mut options = Options::<BytewiseComparator>::default();
    options.logger_level = log_level;
    let dir = temp_dir().join("test_zips_wickdb");
    let db = WickDB::open_db(options, &dir, FileStorage::default()).unwrap();
    db
}

pub mod rocks;

use crate::config;
use rocks::RocksDb;

pub struct Storage {
    _rocks: RocksDb,
}

impl Storage {
    pub fn new(config: config::StorageConfig) -> Self {
        Self {
            _rocks: RocksDb::new(config.data_dir.to_string()),
        }
    }
}

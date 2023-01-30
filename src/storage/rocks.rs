use rocksdb::{DBWithThreadMode, MultiThreaded};
use std::sync::Arc;

pub struct RocksDb {
    _db: Arc<DBWithThreadMode<MultiThreaded>>,
}

impl RocksDb {
    pub fn new(data_dir: String) -> Self {
        Self {
            _db: Arc::new(DBWithThreadMode::open_default(data_dir.as_str()).unwrap()),
        }
    }
}

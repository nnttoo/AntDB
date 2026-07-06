use std::sync::Arc;

use crate::db::AntDB;

pub struct AntDBHash {
    db: Arc<AntDB>,
}

impl AntDBHash {
    pub fn new(db: Arc<AntDB>) -> Arc<Self> {
        Arc::new(Self { db: db })
    }
}

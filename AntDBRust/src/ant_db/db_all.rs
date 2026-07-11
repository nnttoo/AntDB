use std::sync::Arc;

use super::db::AntDB;
use super::db_hashmap::AntDBHash;
use super::db_string::AntDBString;

pub struct AntDBAll {
    pub db: Arc<AntDB>,
    pub db_hash: Arc<AntDBHash>,
    pub db_string: Arc<AntDBString>,
}

impl AntDBAll {
    pub fn new() -> Arc<Self> {
        let db = AntDB::create_arc();

        Arc::new(Self {
            db: db.clone(),
            db_hash: AntDBHash::new(db.clone()),
            db_string: AntDBString::new(db),
        })
    }
}

use std::{collections::HashMap, sync::Arc};

use crate::{BoxError, db::{AntDB, CacheItem, CacheType}};

pub struct AntDBHash {
    db: Arc<AntDB>,
}

impl AntDBHash {

    pub fn hset(&self, key: String, field: String, value: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let entry = hmap_lock.entry(key).or_insert(CacheItem {
            value: CacheType::Hash(HashMap::new()),
            expires_at: None,
        });

        match &mut entry.value {
            CacheType::Hash(hash_map) => {
                hash_map.insert(field, value);
            }
            CacheType::String(_) => {
                let mut hash_map = HashMap::new();
                hash_map.insert(field, value);
                entry.value = CacheType::Hash(hash_map);
            }
        }

        Ok(())
    }

    pub fn new(db: Arc<AntDB>) -> Arc<Self> {
        Arc::new(Self { db: db })
    }
}

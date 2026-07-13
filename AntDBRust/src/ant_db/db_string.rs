use super::{
    db::{AntDB, CacheItem, CacheType},
    db_string_child::AntDBStringChild,
};
use std::sync::Arc;

use crate::BoxError;

pub struct AntDBString {
    db: Arc<AntDB>,
}

impl AntDBString {
    pub fn new(db: Arc<AntDB>) -> Arc<Self> {
        Arc::new(Self { db: db })
    }

    pub fn set(&self, key: String, val: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        hmap_lock.insert(
            key,
            CacheItem {
                value: CacheType::String(AntDBStringChild::new(val)),
                expires_at: None,
            },
        );

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<String, BoxError> {
        let data = {
            let Ok(hmap_lock) = self.db.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(key) else {
                return Err(Box::from("no key font"));
            };

            data.clone()
        };

        if self.db.expire_delete(&key, &data) {
            return Err(Box::from("key is expire"));
        }

        let CacheType::String(value_str) = data.value else {
            return Err(Box::from("data is not string"));
        };

        let Ok(str) = value_str.get() else {
            return Err(Box::from("error read string"));
        };

        Ok(str)
    }

    pub fn setex(&self, key: String, ttl: u64, val: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        hmap_lock.insert(
            key,
            CacheItem {
                value: CacheType::String(AntDBStringChild::new(val)),
                expires_at: Some(CacheItem::set_expire(ttl)),
            },
        );

        Ok(())
    }
}

use super::{
    db::{AntDB, CacheItem, CacheType},
    db_string_child::AntDBStringChild,
};
use std::sync::Arc;

use crate::{BoxError, ant_db::db_hashmap_child::ValPairs};

/// `AntDBString` provides an interface for handling Redis-compatible 
/// String data operations (such as `SET`, `GET`, etc.).
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

    pub fn multiple_set(&self, values: Vec<ValPairs>) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        for item in values {
            let key = item.key;
            let val = item.value;

            hmap_lock.insert(
                key,
                CacheItem {
                    value: CacheType::String(AntDBStringChild::new(val)),
                    expires_at: None,
                },
            );
        }

        Ok(())
    }

    pub fn multiple_get(&self, keys: Vec<String>) -> Result<Vec<Option<String>>, BoxError> {
        let keyslen = keys.len();
        let mut r: Vec<Option<String>> = Vec::with_capacity(keyslen);

        let item_check = {
            let mut ic: Vec<(String, Option<CacheItem>)> = Vec::with_capacity(keyslen);
            let Ok(hmap_lock) = self.db.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            for item in keys {
                let Some(data) = hmap_lock.get(&item) else {
                    ic.push(((&item).clone(), None));
                    continue;
                };

                ic.push((item, Some(data.clone())));
            }

            ic
        };

        for (key, data) in item_check {
            let strdata: Option<String> = 'block: {
                let Some(data) = data else {
                    break 'block None;
                };

                if self.db.expire_delete(&key, &data) {
                    break 'block None;
                }

                let CacheType::String(value_str) = data.value else {
                    break 'block None;
                };

                let Ok(str) = value_str.get() else {
                    break 'block None;
                };

                Some(str)
            };

            r.push(strdata);
        }

        Ok(r)
    }

    pub fn incr(&self, key: String) -> Result<i64, BoxError> {
        let mut i = (|| {
            let Ok(val) = self.get(&key) else {
                return 0;
            };

            let Ok(val) = val.parse::<i64>() else {
                return 0;
            };

            val
        })();

        i = i + 1;

        let Ok(mut hlock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        Ok(0)
    }
}

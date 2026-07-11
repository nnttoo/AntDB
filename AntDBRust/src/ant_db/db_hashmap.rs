use std::sync::Arc;

use super::{
    db::{AntDB, CacheItem, CacheType},
    db_hashmap_child::AntDBHashChild,
};

use crate::BoxError;

pub struct AntDBHash {
    db: Arc<AntDB>,
}

impl AntDBHash {
    pub fn new(db: Arc<AntDB>) -> Arc<Self> {
        Arc::new(Self { db: db })
    }

    pub fn hset(&self, key: String, field: String, value: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let entry = hmap_lock.entry(key).or_insert(CacheItem {
            value: CacheType::Hash(AntDBHashChild::new()),
            expires_at: None,
        });

        match &mut entry.value {
            CacheType::Hash(hash_map) => match hash_map.insert(field, value) {
                Ok(_) => {}
                Err(e) => {
                    return Err(e);
                }
            },
            CacheType::String(_) => {
                let hash_map = AntDBHashChild::new();
                match hash_map.insert(field, value) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                };
                entry.value = CacheType::Hash(hash_map);
            }
        }

        Ok(())
    }

    pub fn hget(&self, key: String, field: String) -> Result<String, BoxError> {
        let data = {
            let Ok(hmap_lock) = self.db.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Err(Box::from("no key found"));
            };

            data.clone()
        };

        if self.db.expire_delete(&key, &data) {
            return Err(Box::from("key is expire"));
        }

        let CacheType::Hash(hchild) = &data.value else {
            return Err(Box::from("value is not hashmap"));
        };

        let Some(val) = hchild.get(&field) else {
            return Err(Box::from("no field found"));
        };

        Ok(val)
    }

    fn hdel_if_empty(&self, key: &str, child: &AntDBHashChild) {
        let Ok(len) = child.len() else {
            return;
        };

        if len == 0 {
            let list_del = vec![key.to_string()];

            _ = self.db.del(list_del);
        }
    }

    pub fn hdel(&self, key: String, fields: Vec<String>) -> Result<i64, BoxError> {
        let child = {
            let Ok(hmap_lock) = self.db.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(child) = hmap_lock.get(&key) else {
                return Ok(0);
            };

            child.clone()
        };

        if self.db.expire_delete(&key, &child) {
            return Err(Box::from("key is expire"));
        }

        let CacheType::Hash(child_hash) = &child.value else {
            return Err(Box::from("cacheitem is not hashmap"));
        };

        let deleted = match child_hash.del(fields) {
            Ok(deleted) => deleted,
            Err(e) => {
                return Err(e);
            }
        };

        self.hdel_if_empty(&key, &child_hash);

        Ok(deleted)
    }
}

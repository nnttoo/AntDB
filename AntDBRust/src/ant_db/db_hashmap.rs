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
    fn get_cache_item(&self, key: &str) -> Result<CacheItem, BoxError> {
        let item = {
            let Ok(hmap_lock) = self.db.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(key) else {
                return Err(Box::from("no key found"));
            };
            data.clone()
        };

        // Delete if expire
        if self.db.expire_delete(&key, &item) {
            return Err(Box::from("key is expire"));
        }

        Ok(item.clone())
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
        let item = match self.get_cache_item(&key) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };

        let CacheType::Hash(hchild) = &item.value else {
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
        let item = match self.get_cache_item(&key) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };

        let CacheType::Hash(child_hash) = &item.value else {
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

    pub fn hlen(&self, key: &str) -> Result<i64, BoxError> {
        let item = match self.get_cache_item(&key) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };

        let CacheType::Hash(child_hash) = &item.value else {
            return Err(Box::from("cacheitem is not hashmap"));
        };

        let len = match child_hash.len() {
            Ok(l) => l,
            Err(e) => return Err(e),
        };

        match i64::try_from(len) {
            Ok(nilai) => Ok(nilai),
            Err(_) => return Err(Box::from("error parse len".to_string())),
        }
    }

    pub fn hexist(&self, key: &str, field: &str) -> Result<bool, BoxError> {
        let item = match self.get_cache_item(&key) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };

        let CacheType::Hash(child_hash) = &item.value else {
            return Err(Box::from("cacheitem is not hashmap"));
        };

        return child_hash.exist(field);
    }

    
    pub fn hmget(&self, key: &str, fields: Vec<String>) -> Result<Vec<Option<String>>, BoxError> {

        let item = match self.get_cache_item(&key) {
            Err(e) => return Err(e),
            Ok(data) => data,
        };

        let CacheType::Hash(child_hash) = &item.value else {
            return Err(Box::from("cacheitem is not hashmap"));
        };

        return child_hash.hmget(fields)
    }

    
}

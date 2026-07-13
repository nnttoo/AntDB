use std::sync::Arc;

use super::{
    db::{AntDB, CacheItem, CacheType},
    db_hashmap_child::{AntDBHashChild, ValPairs},
};

use crate::BoxError;

pub struct AntDBHash {
    db: Arc<AntDB>,
}

impl AntDBHash {
    pub fn new(db: Arc<AntDB>) -> Arc<Self> {
        Arc::new(Self { db: db })
    }
    fn get_hchild(&self, key: &str) -> Result<AntDBHashChild, BoxError> {
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

        let CacheType::Hash(hchild) = &item.value else {
            return Err(Box::from("value is not hashmap"));
        };

        Ok(hchild.clone())
    }

    pub fn hset(&self, key: String, valpair: Vec<ValPairs>) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let entry = hmap_lock.entry(key).or_insert(CacheItem {
            value: CacheType::Hash(AntDBHashChild::new()),
            expires_at: None,
        });

        let hash_map = match &entry.value {
            CacheType::Hash(hm) => hm.clone(),
            CacheType::String(_) => {
                let hash_map = AntDBHashChild::new();
                entry.value = CacheType::Hash((hash_map).clone());
                hash_map.clone()
            }
        };

        hash_map.insert(valpair)?;
        Ok(())
    }

    pub fn hget(&self, key: String, field: String) -> Result<String, BoxError> {
        let hchild = self.get_hchild(&key)?;
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
        let child_hash = self.get_hchild(&key)?;
        let deleted = child_hash.del(fields)?;

        self.hdel_if_empty(&key, &child_hash);

        Ok(deleted)
    }

    pub fn hlen(&self, key: &str) -> Result<i64, BoxError> {
        let child_hash = self.get_hchild(&key)?;

        let len = child_hash.len()?;
        Ok(i64::try_from(len)?)
    }

    pub fn hexist(&self, key: &str, field: &str) -> Result<bool, BoxError> {
        let child_hash = self.get_hchild(&key)?;
        return child_hash.exist(field);
    }

    pub fn hmget(&self, key: &str, fields: Vec<String>) -> Result<Vec<Option<String>>, BoxError> {
        let child_hash = self.get_hchild(&key)?;
        return child_hash.hmget(fields);
    }

    pub fn hkeys(&self, key: &str) -> Result<Vec<String>, BoxError> {
        let child_hash = self.get_hchild(&key)?;
        return child_hash.hkeys();
    }

    pub fn hvals(&self, key: &str) -> Result<Vec<String>, BoxError> {
        let child_hash = self.get_hchild(&key)?;
        return child_hash.hvals();
    }

    pub fn hgetall(&self, key: &str) -> Result<Vec<ValPairs>, BoxError> {
        let child_hash = self.get_hchild(&key)?;
        return child_hash.hgetall();
    }
}

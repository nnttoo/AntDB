use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use crate::BoxError;
#[derive(Clone)]
pub enum CacheType {
    String(String),
    Hash(HashMap<String, String>),
}
#[derive(Clone)]
struct CacheItem {
    value: CacheType,
    expires_at: Option<Instant>, // Jika None, berarti data permanen
}

impl CacheItem {
    pub fn set_expire(second: u64) -> Instant {
        Instant::now() + Duration::from_secs(second)
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expires_at {
            Instant::now() > expiry
        } else {
            false
        }
    }
}

type HashDB = Arc<RwLock<HashMap<String, CacheItem>>>;
pub struct AntDB {
    hash_map: HashDB,
}

pub type AntDbArc = Arc<AntDB>;

impl AntDB {
    pub fn create_arc() -> AntDbArc {
        Arc::new(AntDB {
            hash_map: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn set(&self, key: String, val: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error locck"));
        };

        hmap_lock.insert(
            key,
            CacheItem {
                value: CacheType::String(val),
                expires_at: None,
            },
        );

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<String, BoxError> {
        let (is_expire, value) = {
            let Ok(hmap_lock) = self.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Err(Box::from("no key font"));
            };

            let value = data.value.clone();
            let is_expire = data.is_expired();
            (is_expire, value)
        };

        if is_expire {
            if let Ok(mut hmap_lock) = self.hash_map.write() {
                hmap_lock.remove(&key);
            }

            return Err(Box::from("key is expire"));
        }

        let CacheType::String(value_str) = value else {
            return Err(Box::from("data is not string"));
        };

        Ok(value_str)
    }

    pub fn setex(&self, key: String, ttl: u64, val: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        hmap_lock.insert(
            key,
            CacheItem {
                value: CacheType::String(val),
                expires_at: Some(CacheItem::set_expire(ttl)),
            },
        );

        Ok(())
    }

    pub fn expire(&self, key: String, ttl: u64) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let Some(data) = hmap_lock.get_mut(&key) else {
            return Err(Box::from("no key found"));
        };

        data.expires_at = Some(CacheItem::set_expire(ttl));

        Ok(())
    }

    pub fn hset(&self, key: String, field: String, value: String) -> Result<(), BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
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

    pub fn hget(&self, key: String, field: String) -> Result<String, BoxError> {
        let (is_expire, value) = {
            let Ok(hmap_lock) = self.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Err(Box::from("no key found"));
            };

            let is_expire = data.is_expired();
            let value = data.value.clone();

            (is_expire, value)
        };

        if is_expire {
            if let Ok(mut hmap_lock) = self.hash_map.write() {
                hmap_lock.remove(&key);
            }

            return Err(Box::from("key is expire"));
        }

        let CacheType::Hash(hash_map) = value else {
            return Err(Box::from("data is not hash"));
        };

        let Some(value) = hash_map.get(&field) else {
            return Err(Box::from("field not found"));
        };

        Ok(value.clone())
    }

    pub fn exist(&self, key: String) -> Result<i64, BoxError> {
        let Ok(hmap_lock) = self.hash_map.read() else {
            return Err(Box::from("error lock"));
        };

        let Some(data) = hmap_lock.get(&key) else {
            return Ok(0);
        };

        if data.is_expired() {
            return Ok(0);
        }

        Ok(1)
    }

    pub fn del(&self, key: String) -> Result<String, BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        if hmap_lock.remove(&key).is_some() {
            Ok("1".to_string())
        } else {
            Ok("0".to_string())
        }
    }
}

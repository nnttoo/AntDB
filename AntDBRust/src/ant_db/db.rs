use super::{db_hashmap_child::AntDBHashChild, db_string_child::AntDBStringChild};

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use crate::BoxError;

#[derive(Clone)]
pub enum CacheType {
    String(AntDBStringChild),
    Hash(AntDBHashChild),
}
#[derive(Clone)]
pub struct CacheItem {
    pub value: CacheType,
    pub expires_at: Option<Instant>, // Jika None, berarti data permanen
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

pub type HashDB = Arc<RwLock<HashMap<String, CacheItem>>>;
pub struct AntDB {
    pub hash_map: HashDB,
}

impl AntDB {
    pub fn create_arc() -> Arc<Self> {
        Arc::new(AntDB {
            hash_map: Arc::new(RwLock::new(HashMap::new())),
        })
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

    pub fn expire_delete(&self, key: &str, data: &CacheItem) -> bool {
        if !data.is_expired() {
            return false;
        }

        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return false;
        };

        hmap_lock.remove(key);
        true
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

    pub fn del(&self, keys: Vec<String>) -> Result<i64, BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let mut total_deleted = 0;

        // Looping
        for key in keys {
            if hmap_lock.remove(&key).is_some() {
                total_deleted += 1;
            }
        }

        Ok(total_deleted)
    }

    pub fn pttl(&self, key: String) -> Result<i64, BoxError> {
        // ttl_value
        // -2 NotFound/Expire
        // -1 Permanent

        // Create a temporary scope for the read lock to prevent deadlocks
        let data = {
            let Ok(hmap_lock) = self.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Ok(-2);
            };

            data.clone()
        };

        let Some(expiry) = data.expires_at else {
            return Ok(-1); //  is permanent
        };

        if self.expire_delete(&key, &data) {
            return Ok(-2); // Expire -2
        }

        let now = Instant::now();
        let duration = expiry.duration_since(now);
        let ttl_value = duration.as_millis() as i64;

        Ok(ttl_value)
    }

    pub fn persist(&self, key: String) -> Result<i64, BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let Some(data) = hmap_lock.get(&key) else {
            return Err(Box::from("key not found"));
        };

        let Some(_) = &data.expires_at else {
            return Ok(0);
        };

        let mut data_clone = data.clone();
        data_clone.expires_at = None;
        hmap_lock.insert(key, data_clone);

        Ok(1)
    }
}

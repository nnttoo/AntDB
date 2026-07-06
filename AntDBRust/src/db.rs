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
        let data = {
            let Ok(hmap_lock) = self.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Err(Box::from("no key font"));
            };

            data.clone()
        };

        if self.expire_delete(key, &data) {
            return Err(Box::from("key is expire"));
        }

        let CacheType::String(value_str) = data.value else {
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

    fn expire_delete(&self, key: String, data: &CacheItem) -> bool {
        if !data.is_expired() {
            return false;
        }

        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return false;
        };

        hmap_lock.remove(&key);
        true
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
        let mut r_value  = "".to_string();
        let r_expire_at: Option<Instant>;
        let mut r_error: Option<String>  = Some("not field found".to_string());

        {
            let Ok(hmap_lock) = self.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Err(Box::from("no key found"));
            };

            
            r_expire_at = (&data).expires_at;

            if !data.is_expired() {
                if let CacheType::Hash(hash_map) = &data.value {
                    if let Some(str_value) = hash_map.get(&field) {
                        r_value = str_value.clone(); 
                        r_error = None;

                    }
                } 
            }   
        };

        let dumy_data = CacheItem { 
            value: CacheType::String(String::new()), 
            expires_at: r_expire_at
        };

        if self.expire_delete(key,&dumy_data) {
            return Err(Box::from("key is expire"));
        } 

        if let Some(error_str) = r_error{
             return Err(Box::from(error_str));
        }

        Ok(r_value) 

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

    pub fn del(&self, key: String) -> Result<i64, BoxError> {
        let Ok(mut hmap_lock) = self.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        if hmap_lock.remove(&key).is_some() {
            Ok(1)
        } else {
            Ok(0)
        }
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

        if self.expire_delete(key, &data) {
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

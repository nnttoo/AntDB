use std::{collections::HashMap, sync::Arc, time::Instant};

use crate::{
    BoxError,
    db::{AntDB, CacheItem, CacheType},
};

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
        let mut r_value = "".to_string();
        let r_expire_at: Option<Instant>;
        let mut r_error: Option<String> = Some("not field found".to_string());

        {
            let Ok(hmap_lock) = self.db.hash_map.read() else {
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
            expires_at: r_expire_at,
        };

        if self.db.expire_delete(key, &dumy_data) {
            return Err(Box::from("key is expire"));
        }

        if let Some(error_str) = r_error {
            return Err(Box::from(error_str));
        }

        Ok(r_value)
    }
}

use std::{sync::Arc };

use crate::{
    BoxError,
    db::{AntDB, CacheItem, CacheType},
    db_hashmap_child::AntDBHashChild,
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
                    Ok(_)=>{},
                    Err(e)=>{return Err(e)}
                };
                entry.value = CacheType::Hash(hash_map);
            }
        }

        Ok(())
    }

    pub fn hget(&self, key: String, field: String) -> Result<String, BoxError> {  
        let data =  {
            let Ok(hmap_lock) = self.db.hash_map.read() else {
                return Err(Box::from("error lock"));
            };

            let Some(data) = hmap_lock.get(&key) else {
                return Err(Box::from("no key found"));
            }; 

            data.clone()
        }; 

        if self.db.expire_delete(key, &data) {
            return Err(Box::from("key is expire"));
        }  

        let CacheType::Hash(hchild) = &data.value else {
            return Err(Box::from("value is not hashmap"));
        };

        let Some(val) = hchild.get(&field) else{
            return Err(Box::from("no field found"));
        };

        Ok(val)
    }

    pub fn hdel(&self, key: String, field: Vec<String>) -> Result<u64, BoxError> {
        let Ok(hmap_lock) = self.db.hash_map.write() else {
            return Err(Box::from("error lock"));
        };

        let mut total_deleted = 0;

        Ok(0)
    }
}

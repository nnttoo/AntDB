use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::BoxError;

#[derive(Clone)]
pub struct AntDBHashChild {
    hashchild: Arc<RwLock<HashMap<String, String>>>,
}

impl AntDBHashChild {
    pub fn new() -> Self {
        AntDBHashChild {
            hashchild: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, field: String, value: String) -> Result<(), BoxError> {
        let Ok(mut hchild) = self.hashchild.write() else {
            return Err(Box::from("error lock hchild"));
        };

        hchild.insert(field, value);

        Ok(())
    }

    pub fn get(&self, field: &str) -> Option<String> {
        let Ok(hchild) = self.hashchild.read() else {
            return None;
        };

        let Some(val) = hchild.get(field) else {
            return None;
        };

        Some(val.clone())
    }

    pub fn del(&self, fields: Vec<String>) -> Result<i64, BoxError> {
        let mut total_deleted = 0;

        let Ok(mut hchild) = self.hashchild.write() else {
            return Err(Box::from("error lock hchild"));
        };

        for field in fields {
            if hchild.remove(&field).is_some() {
                total_deleted += 1;
            }
        }

        Ok(total_deleted)
    }

    pub fn len(&self) -> Result<usize, BoxError> {
        let Ok(hchild) = self.hashchild.read() else {
            return Err(Box::from("error lock hchild"));
        };

        Ok(hchild.len())
    }

    pub fn exist(&self, field: &str) -> Result<bool, BoxError> {
        let Ok(hchild) = self.hashchild.read() else {
            return Err(Box::from("error lock hchild"));
        };

        Ok(hchild.contains_key(field))
    }

    pub fn hmget(&self, fields: Vec<String>) -> Result<Vec<Option<String>>, BoxError> {
        let mut result: Vec<Option<String>> = Vec::with_capacity(fields.len());

        let Ok(hchild) = self.hashchild.read() else {
            return Err(Box::from("error lock hchild"));
        };

        for field in fields {
            let val = hchild.get(&field).cloned();
            result.push(val);
        }

        Ok(result)
    }

    pub fn hkeys(&self)->Result<Vec<String>, BoxError>{
         let Ok(hchild) = self.hashchild.read() else {
            return Err(Box::from("error lock hchild"));
        };

        let keys : Vec<String> = hchild.keys().cloned().collect(); 
        Ok(keys)
    }
}

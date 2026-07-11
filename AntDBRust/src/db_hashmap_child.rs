use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::BoxError;


#[derive(Clone)]
pub struct AntDBHashChild{
    hashchild: Arc<RwLock<HashMap<String,String>>>
}

impl AntDBHashChild {
    pub fn new()->Self{
        AntDBHashChild{
            hashchild : Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn insert(&self, field: String, value : String)->Result<(), BoxError>{
        let Ok(mut hchild)=self.hashchild.write() else {
            return Err(Box::from("error lock hchild"));
        };

        hchild.insert(field, value);

        Ok(())
    }

    pub fn get(&self, field: &str)->Option<String>{
        let Ok(hchild)=self.hashchild.read() else {
            return None;
        };

        let Some(val) = hchild.get(field) else {
            return None;
        };

        Some(val.clone())
    }
}

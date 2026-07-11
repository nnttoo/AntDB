use std::sync::{Arc, RwLock};

use crate::BoxError;

#[derive(Clone)]
pub struct AntDBStringChild {
    string_child: Arc<RwLock<String>>,
}

impl AntDBStringChild {
    pub fn new(value: String) -> Self {
        Self {
            string_child: Arc::new(RwLock::new(value)),
        }
    } 

    pub fn get(&self) -> Result<String, BoxError> {
        let Ok(str_read) = self.string_child.read() else {
            return Err(Box::from("error lock hchild"));
        };

        Ok(str_read.clone())
    }
}

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

    pub fn set(&self, value: String) -> Result<(), BoxError> {
        let Ok(mut str_write) = self.string_child.write() else {
            return Err(Box::from("error lock hchild"));
        };

        *str_write = value;

        Ok(())
    }

    pub fn get(&self) -> Result<String, BoxError> {
        let Ok(str_read) = self.string_child.read() else {
            return Err(Box::from("error lock hchild"));
        };

        Ok(str_read.clone())
    }
}

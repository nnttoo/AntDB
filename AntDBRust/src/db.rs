use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};

#[derive(Clone)]
struct CacheItem {
    value: String,
    expires_at: Option<Instant>, // Jika None, berarti data permanen
}

pub type HashDB = Arc<Mutex<HashMap<String, CacheItem>>>;
pub struct AntDB {
    pub hash_map: HashDB,
}

pub type AntDbArc = Arc<AntDB>;

impl AntDB {
    pub fn create_arc() -> AntDbArc {
        Arc::new(AntDB {
            hash_map: Arc::new(Mutex::new(HashMap::new())),
        })
    }
}

use std::sync::Arc;

use crate::{db::{AntDB}, db_hashmap::AntDBHash};


pub struct  AntDBAll {
    pub db : Arc<AntDB>,
    pub db_hash : Arc<AntDBHash>
}

impl  AntDBAll  {
    pub fn new()->Arc<Self>{
        let db = AntDB::create_arc();

        Arc::new(Self  { 
            db: db.clone(), 
            db_hash: AntDBHash::new(db)
        })
    }
}
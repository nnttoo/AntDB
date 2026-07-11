use resp::Value;

use super::tools::get_list_fields;
use crate::app_ctx::AppCtxArc;

pub struct ServerAntDbRespHashMap {
    pub app_ctx: AppCtxArc,
}

impl ServerAntDbRespHashMap {
    pub fn new(appctx: AppCtxArc) -> Self {
        Self { app_ctx: appctx }
    }

    pub fn hget(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'hget' command".to_string());
        }

        let key_variant = values.remove(0);
        let field_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(field)) = (key_variant, field_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };
        let db = &self.app_ctx.ant_db.db_hash;

        match db.hget(key, field) {
            Ok(value) => Value::Bulk(value),
            Err(_) => Value::Null,
        }
    }

    pub fn hset(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 3 {
            return Value::Error("ERR wrong number of arguments for 'hset' command".to_string());
        }

        let key_variant = values.remove(0);
        let field_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(field), Value::Bulk(value)) =
            (key_variant, field_variant, val_variant)
        else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };
        let db = &self.app_ctx.ant_db.db_hash;

        match db.hset(key, field, value) {
            Ok(_) => Value::String("OK".to_string()),
            Err(e) => Value::Error(e.to_string()),
        }
    }

    pub fn hdel(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'hdel' command".to_string());
        }

        let key_variant = values.remove(0); 
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let fields = get_list_fields(&values);

        let db = &self.app_ctx.ant_db.db_hash;

        match db.hdel(key, fields) {
            Ok(deleted) => Value::Integer(deleted),
            Err(e) => Value::Error(e.to_string()),
        }
    }
}

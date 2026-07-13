use super::tools::get_list_fields;
use crate::{ant_db::db_hashmap_child::ValPairs, ant_resp::value::Value, app_ctx::AppCtxArc};

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

        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let valpair = {
            let mut vpair: Vec<ValPairs> = Vec::new();
            while values.len() >= 2 {
                let field_variant = values.remove(0);
                let val_variant = values.remove(0);

                if let (Value::Bulk(field), Value::Bulk(value)) = (field_variant, val_variant) {
                    vpair.push(ValPairs{
                        key : field,
                        value : value,
                    });
                }  
            }

            vpair
        };

        let vpairlen = valpair.len() as i64;
        let db = &self.app_ctx.ant_db.db_hash; 
        match db.hset(key, valpair) {
            Ok(_) => Value::Integer(vpairlen),
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

    pub fn hlen(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR Resp argument is empty".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_hash;

        match db.hlen(&key) {
            Ok(len) => Value::Integer(len),
            Err(_) => Value::Integer(0),
        }
    }

    pub fn hexist(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR Resp argument is empty".to_string());
        }

        let key_variant = values.remove(0);
        let field_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(field)) = (key_variant, field_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_hash;
        match db.hexist(&key, &field) {
            Ok(exist) => {
                if exist {
                    return Value::Integer(1);
                } else {
                    return Value::Integer(0);
                }
            }
            Err(_) => {
                return Value::Integer(0);
            }
        }
    }
    pub fn hmget(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'hdel' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let fields = get_list_fields(&values);
        let field_len = fields.len();

        let mut val_arr: Vec<Value> = Vec::with_capacity(field_len);
        let db = &self.app_ctx.ant_db.db_hash;
        match db.hmget(&key, fields) {
            Ok(hmget_result) => {
                for f in hmget_result {
                    let value = match f {
                        Some(val) => Value::String(val),
                        None => Value::Null,
                    };
                    val_arr.push(value);
                }
            }
            Err(_) => {
                for _ in 0..field_len {
                    val_arr.push(Value::Null);
                }
            }
        };

        Value::Array(val_arr)
    }

    pub fn hkeys(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'hdel' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let mut val_arr: Vec<Value> = Vec::new();

        let db = &self.app_ctx.ant_db.db_hash;
        let Ok(hkeys) = db.hkeys(&key) else {
            return Value::Array(val_arr);
        };

        for f in hkeys {
            val_arr.push(Value::String(f));
        }

        Value::Array(val_arr)
    }

    pub fn hvals(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'hdel' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let mut val_arr: Vec<Value> = Vec::new();

        let db = &self.app_ctx.ant_db.db_hash;
        let Ok(hkeys) = db.hvals(&key) else {
            return Value::Array(val_arr);
        };

        for f in hkeys {
            val_arr.push(Value::String(f));
        }

        Value::Array(val_arr)
    }

    pub fn hgetall(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'hdel' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_hash;

        let mut val_arr: Vec<Value> = Vec::new();
        let Ok(hgetall) = db.hgetall(&key) else {
            return Value::Array(val_arr);
        };

        for item in hgetall {
            val_arr.push(Value::String(item.key));
            val_arr.push(Value::String(item.value));
        }

        Value::Array(val_arr)
    }
}

 
use crate::app_ctx::AppCtxArc;
use crate::{
    ant_resp::value::{Value}
};
use super::{
    tools::get_list_fields,
    
};

pub struct ServerAntDbResp {
    pub app_ctx: AppCtxArc,
}

impl ServerAntDbResp {
    pub fn new(appctx: AppCtxArc) -> Self {
        Self { app_ctx: appctx }
    }

    pub fn ping(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::String("PONG".to_string());
        }

        // Ambil argumen pertama
        let arg = values.remove(0);

        // Pastikan jika tipenya Bulk (dari redis-benchmark), kita kembalikan
        // sebagai String atau Bulk yang bersih agar dibaca sebagai valid reply oleh benchmark.
        match arg {
            Value::Bulk(text) => Value::String(text), // Mengubah Bulk menjadi Simple String seringkali lebih aman untuk benchmark
            Value::String(text) => Value::String(text),
            _ => Value::String("PONG".to_string()),
        }
    }

    pub fn set(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'set' command".to_string());
        }
        let key_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(value)) = (key_variant, val_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.set(key, value) {
            Ok(()) => Value::String("OK".to_string()),
            _ => Value::Null,
        }
    }

    pub fn get(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'get' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key_bytes) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.get(key_bytes) {
            Ok(data) => Value::String(data),
            Err(_) => Value::Null,
        }
    }

    pub fn setex(&self, mut values: Vec<Value>) -> Value {
        // 1. Validasi jumlah argumen (SETEX butuh 3 argumen: key, ttl, value)
        if values.len() < 3 {
            return Value::Error("ERR wrong number of arguments for 'setex' command".to_string());
        }

        let key_variant = values.remove(0);
        let ttl_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(ttl_bytes), Value::Bulk(value)) =
            (key_variant, ttl_variant, val_variant)
        else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let Ok(ttl) = ttl_bytes.parse::<u64>() else {
            return Value::Error("error parse ttl".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.setex(key, ttl, value) {
            Ok(_) => Value::String("OK".to_string()),
            Err(e) => Value::Error(e.to_string()),
        }
    }

    pub fn expire(&self, mut values: Vec<Value>) -> Value {
        // 1. Validasi jumlah argumen
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'expire' command".to_string());
        }

        // 2. Ekstrak data secara konsisten menggunakan remove(0)
        let key_variant = values.remove(0);
        let ttl_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(ttl_str)) = (key_variant, ttl_variant) else {
            return Value::Error("parse error".to_string());
        };

        let Ok(ttl) = ttl_str.parse::<u64>() else {
            return Value::Error("error parse ttl".to_string());
        };

        let db = &self.app_ctx.ant_db.db;
        match db.expire(key, ttl) {
            Ok(_) => Value::String("OK".to_string()),
            Err(_) => Value::Null,
        }
    }

    pub fn exists(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'exists' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db;
        match db.exist(key) {
            Ok(result) => Value::Integer(result),
            Err(_) => Value::Integer(0),
        }
    }

    pub fn del(&self, values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'del' command".to_string());
        }
        let keys = get_list_fields(&values);
        let db = &self.app_ctx.ant_db.db;

        // Oper Vec<String> langsung ke db.del yang baru
        match db.del(keys) {
            Ok(result) => Value::Integer(result),
            Err(_) => Value::Integer(0),
        }
    }

    pub fn ttl(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'del' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db;
        match db.pttl(key) {
            Ok(ms_result) => {
                // return without convertion if -2 or -1
                let final_ttl = if ms_result < 0 {
                    ms_result
                } else {
                    // convert with roundup as redis standard
                    (ms_result + 999) / 1000
                };
                Value::Integer(final_ttl)
            }
            Err(_) => Value::Integer(-2),
        }
    }

    pub fn pttl(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'pttl' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db;
        match db.pttl(key) {
            Ok(ms_result) => Value::Integer(ms_result), // Langsung ambil milidetik murninya
            Err(_) => Value::Integer(-2),
        }
    }

    pub fn persist(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'pttl' command".to_string());
        }

        let Value::Bulk(key) = values.remove(0) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };
        let db = &self.app_ctx.ant_db.db;
        match db.persist(key) {
            Ok(r) => Value::Integer(r),
            Err(d) => Value::Error(d.to_string()),
        }
    }
}

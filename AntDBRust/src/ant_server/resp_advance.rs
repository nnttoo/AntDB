use crate::{
    ant_resp::value::Value,
    ant_server::tools::{get_list_fields, get_list_valpair},
    app_ctx::AppCtxArc,
};

pub struct ServerAntDbRespAdvance {
    pub app_ctx: AppCtxArc,
}

impl ServerAntDbRespAdvance {
    pub fn new(appctx: AppCtxArc) -> Self {
        Self { app_ctx: appctx }
    }

    pub fn mset(&self, values: Vec<Value>) -> Value {
        let vecval = get_list_valpair(values);

        let db = &self.app_ctx.ant_db.db_string;

        match db.multiple_set(vecval) {
            Ok(_) => Value::String("OK".to_string()),
            Err(e) => Value::Error(e.to_string()),
        }
    }

    pub fn mget(&self, values: Vec<Value>) -> Value {
        let veckey = get_list_fields(&values);
        let veclen = (&veckey).len();
        let db = &self.app_ctx.ant_db.db_string;

        match db.multiple_get(veckey) {
            Ok(v) => {
                let mut vecval: Vec<Value> = Vec::with_capacity(veclen);
                for i in v {
                    match i {
                        Some(str) => {
                            vecval.push(Value::String(str));
                        }
                        None => {
                            vecval.push(Value::Null);
                        }
                    }
                }

                Value::Array(vecval)
            }
            Err(e) => Value::Error(e.to_string()),
        }
    }

    pub fn incr(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'get' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key_bytes) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.incr(&key_bytes) {
            Ok(data) => Value::Integer(data),
            Err(_) => Value::Null,
        }
    }

    pub fn decr(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'get' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key_bytes) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.decr(&key_bytes) {
            Ok(data) => Value::Integer(data),
            Err(_) => Value::Null,
        }
    }

    pub fn append(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'set' command".to_string());
        }
        let key_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(value)) = (key_variant, val_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.append(key, value) {
            Ok(n) => Value::Integer(n),
            _ => Value::Null,
        }
    }

     pub fn getset(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'set' command".to_string());
        }
        let key_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(value)) = (key_variant, val_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_string;

        match db.getset(key, value) {
            Ok(n) => {
                match n {
                    Some(n)=>Value::String(n),
                    None=>Value::Null
                }
            },
            _ => Value::Null,
        }
    }
}

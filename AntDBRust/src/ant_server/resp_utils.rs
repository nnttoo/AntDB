use crate::{ant_resp::value::Value, app_ctx::AppCtxArc};

pub struct ServerAntDbRespUtils {
    pub app_ctx: AppCtxArc,
}

impl ServerAntDbRespUtils {
    pub fn new(appctx: AppCtxArc) -> Self {
        Self { app_ctx: appctx }
    }

    pub fn keys(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'keys' command".to_string());
        }
        let key_variant = values.remove(0);

        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db_utils;

        match db.keys(&key) {
            Ok(n) => {
                let mut vallist: Vec<Value> = Vec::new();
                for v in n {
                    vallist.push(Value::String(v));
                }

                Value::Array(vallist)
            }
            _ => Value::Null,
        }
    }
}

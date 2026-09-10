use crate::{ant_resp::value::Value, ant_server::tools::get_list_valpair, app_ctx::AppCtxArc};
 
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
}

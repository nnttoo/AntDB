use std::sync::Arc;

use crate::app_ctx::AppCtxArc;


pub struct ServerAntDb{
    pub app_ctx: AppCtxArc
}

pub type  ServerAntDbArc = Arc<ServerAntDb>;

impl ServerAntDb {
    pub fn to_arc(self)->ServerAntDbArc{
        Arc::new(self)
    }

    pub fn start_server(self : ServerAntDbArc){
        
    }
}
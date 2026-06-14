use std::sync::Arc;

use crate::{config::{ServerConfig, ServerConfigArc}, db::{AntDB, AntDbArc}};


pub struct AppCtx{
    pub server_config : ServerConfigArc,
    pub ant_db : AntDbArc
}

pub type  AppCtxArc = Arc<AppCtx>;

impl AppCtx {
    fn to_arc(self)->AppCtxArc{
        Arc::new(self)
    }

    pub async fn get_appctx()->AppCtxArc{
        let server_config = ServerConfig::get_or_create_server_config().await;
        
        let app_ctx = AppCtx{
            server_config : server_config,
            ant_db : AntDB::create_arc()
        };

        app_ctx.to_arc()
    }
}

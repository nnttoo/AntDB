use std::sync::Arc;

use crate::config::{ServerConfig, ServerConfigArc};


pub struct AppCtx{
    pub server_config : ServerConfigArc
}

type  AppCtxArc = Arc<AppCtx>;

impl AppCtx {
    fn to_arc(self)->AppCtxArc{
        Arc::new(self)
    }

    pub async fn get_appctx()->AppCtxArc{
        let server_config = ServerConfig::get_or_create_server_config().await;
        
        let app_ctx = AppCtx{
            server_config : server_config
        };

        app_ctx.to_arc()
    }
}

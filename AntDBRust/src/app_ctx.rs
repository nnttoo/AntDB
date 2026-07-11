use std::sync::Arc;

use crate::{ant_db::db_all::AntDBAll, config::{ServerConfig, ServerConfigArc}, };
 


pub struct AppCtx{
    pub server_config : ServerConfigArc,
    pub ant_db : Arc<AntDBAll>
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
            ant_db : AntDBAll::new()
        };

        app_ctx.to_arc()
    }
}

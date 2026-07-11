pub type BoxError = Box<dyn std::error::Error>; 

use crate::app_ctx::AppCtx;
use crate::server::ServerAntDb;
mod  app_ctx;
mod utils_tools;
mod config;
mod db;
mod db_hashmap;
mod db_hashmap_child;
mod db_string_child;
mod db_string;
mod db_all;
mod server;



#[tokio::main]
async  fn main() {
    let appctx = AppCtx::get_appctx().await;
    
    let server = ServerAntDb{
        app_ctx : appctx,
    }.to_arc();

    _=server.start_server().await;
}

pub type BoxError = Box<dyn std::error::Error>; 

use crate::{ant_server::server::ServerAntDb, app_ctx::AppCtx}; 
mod  app_ctx;
mod utils_tools;
mod ant_db;
mod config; 
mod ant_server; 



#[tokio::main]
async  fn main() {
    let appctx = AppCtx::get_appctx().await;
    
    let server = ServerAntDb::new(appctx).to_arc(); 
    _=server.start_server().await;
}

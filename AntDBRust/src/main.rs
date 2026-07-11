pub type BoxError = Box<dyn std::error::Error>; 

use crate::app_ctx::AppCtx;
use crate::server::ServerAntDb;
mod  app_ctx;
mod utils_tools;
mod ant_db;
mod config; 
mod server;
mod server_tools;
mod server_resp;



#[tokio::main]
async  fn main() {
    let appctx = AppCtx::get_appctx().await;
    
    let server = ServerAntDb::new(appctx).to_arc(); 
    _=server.start_server().await;
}

pub type BoxError = Box<dyn std::error::Error>;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::app_ctx::AppCtx;
mod  app_ctx;
mod utils_tools;
mod config;
mod db;
mod server;


#[tokio::main]
async  fn main() {
    let appctx = AppCtx::get_appctx().await;
    
    println!("Hello, world! port nya {}",appctx.server_config.port);
}

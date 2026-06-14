use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::fs::{read_to_string, write};

use crate::utils_tools::{BoxError, get_exe_folder, simple_file_exist};

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u32,
}

pub type ServerConfigArc = Arc<ServerConfig>;

const JSON_CONFIG_FILE_NAME: &str = "server_config.json";

impl ServerConfig {
    pub fn to_arc(self) -> Arc<Self> {
        Arc::new(self)
    }

    ///
    /// Haryanto 14/06/2026
    /// read ServerConfig grom json path
    ///
    async fn from_json(json_path: &PathBuf) -> Result<ServerConfig, BoxError> {
        if !simple_file_exist(&json_path) {
            return Err(Box::from("file not found"));
        }

        let json_str = read_to_string(json_path).await?;
        let json_obj = serde_json::from_str::<ServerConfig>(&json_str)?;

        Ok(json_obj)
    } 

    ///Write ServerConfig to json on working directory
    async fn create_and_write(json_path : &PathBuf) -> ServerConfig {
        let result = ServerConfig { port: 6379 };
 
        let json_str = serde_json::to_string_pretty(&result);
        if json_str.is_ok() {
            let json_str = json_str.unwrap();
            _ = write(json_path, json_str).await;
        }

        result
    }

    pub async fn get_or_create_server_config() -> ServerConfigArc {
        let json_path = get_exe_folder().join(JSON_CONFIG_FILE_NAME);
        match Self::from_json(&json_path).await {
            Ok(w) => {
                return w.to_arc();
            }
            _ => {}
        }

        
        // json path to working directory
        let json_path: PathBuf = JSON_CONFIG_FILE_NAME.into();
        match Self::from_json(&json_path).await {
            Ok(w) => {
                return w.to_arc();
            }
            _ => {}
        }

        let server_config = Self::create_and_write(&json_path).await; 
        server_config.to_arc()
    }
}

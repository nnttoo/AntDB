use std::{io::BufReader, sync::Arc};

use resp::{Decoder, Value};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{app_ctx::AppCtxArc, utils_tools::BoxError};

pub struct ServerAntDb {
    pub app_ctx: AppCtxArc,
}

pub type ServerAntDbArc = Arc<ServerAntDb>;

impl ServerAntDb {
    pub fn to_arc(self) -> ServerAntDbArc {
        Arc::new(self)
    }

    pub async fn start_server(self: ServerAntDbArc) -> Result<(), BoxError> {
        let add = format!("127.0.0.1:{}", self.app_ctx.server_config.port);

        println!("server is started {}", &add);
        let listener = TcpListener::bind(add).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let self_clone = self.clone();
            tokio::spawn(async move {
                self_clone.child_open(socket).await;
            });
        }

        Ok(())
    }

    async fn child_open(self: ServerAntDbArc, mut socket: TcpStream) {
        let mut buffer = [0; 1024];
        loop {
            let Ok(n) = socket.read(&mut buffer).await else {
                return;
            };

            if n == 0 {
                return;
            }

            let mut decoder = Decoder::new(BufReader::new(&buffer[..n]));
            let Ok(decoded) = decoder.decode() else {
                continue;
            };

            match decoded {
                Value::Array(mut values) => {
                    if values.is_empty() {
                        continue;
                    }

                    let Value::Bulk(cmd_bytes) = values.remove(0) else {
                        continue;
                    };

                    let command_name = cmd_bytes.to_uppercase();
                    match command_name.as_str() {
                        "CLIENT" => {
                            let _ = socket
                                .write_all(&Value::String("OK".to_string()).encode())
                                .await;
                        }
                        "INFO" => {
                            let info_data = "# Server\r\nAntDB_version:7.0.0\r\n";
                            let response = Value::Bulk(info_data.to_string());
                            let _ = socket.write_all(&response.encode()).await;
                        } 
                        "PING" => {
                            println!("ping detected");
                            let response = Value::String("PONG".to_string());
                            let encoded_response = response.encode();
                            _ = socket.write_all(&encoded_response).await;
                        }
                        "COMMAND" => {
                            // Balas dengan array kosong agar client tidak error membaca daftar command
                            let response = Value::Array(vec![]);
                            let _ = socket.write_all(&response.encode()).await;
                        }
                        "SET" => {
                            let response = &self.resp_set(values);
                            _ = socket.write_all(&response.encode()).await;
                        }
                        
                        "SETEX" => {
                            let response = &self.resp_setex(values);
                            _ = socket.write_all(&response.encode()).await;
                        } 
                        "EXPIRE" => {
                            let response = &self.resp_expire(values);
                            _ = socket.write_all(&response.encode()).await;
                        }
                        "GET" => {
                            let response = &self.resp_get(values);
                            _ = socket.write_all(&response.encode()).await;
                        }
                        "HSET" => {
                            let response = &self.resp_hset(values);
                            _ = socket.write_all(&response.encode()).await;
                        }
                        "HGET" => {
                            let response = &self.resp_hget(values);
                            _ = socket.write_all(&response.encode()).await;
                        }
                        _ => {
                            println!("command unhandled : {}", command_name);
                            let err_msg = format!("ERR unknown command '{}'", command_name);
                            let _ = socket.write_all(&Value::Error(err_msg).encode()).await;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn resp_set(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'set' command".to_string());
        }
        let key_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(value)) = (key_variant, val_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        match self.app_ctx.ant_db.set(key, value) {
            Ok(()) => Value::String("OK".to_string()),
            _ => Value::Null,
        }
    }

    pub fn resp_get(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'get' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key_bytes) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        match self.app_ctx.ant_db.get(key_bytes) {
            Ok(data) => Value::String(data),
            Err(_) => Value::Null,
        }
    }

    fn resp_hget(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'hget' command".to_string());
        }

        let key_variant = values.remove(0);
        let field_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(field)) = (key_variant, field_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        match self.app_ctx.ant_db.hget(key, field) {
            Ok(value) => Value::Bulk(value),
            Err(_) => Value::Null,
        }
    }

    fn resp_hset(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 3 {
            return Value::Error("ERR wrong number of arguments for 'hset' command".to_string());
        }

        let key_variant = values.remove(0);
        let field_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(field), Value::Bulk(value)) =
            (key_variant, field_variant, val_variant)
        else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        match self.app_ctx.ant_db.hset(key, field, value) {
            Ok(_) => Value::String("OK".to_string()),
            Err(e) => Value::Error(e.to_string()),
        }
    }

    fn resp_setex(&self, mut values: Vec<Value>) -> Value {
        // 1. Validasi jumlah argumen (SETEX butuh 3 argumen: key, ttl, value)
        if values.len() < 3 {
            return Value::Error("ERR wrong number of arguments for 'setex' command".to_string());
        }

        let key_variant = values.remove(0);
        let ttl_variant = values.remove(0);
        let val_variant = values.remove(0);

        let (Value::Bulk(key), 
            Value::Bulk(ttl_bytes), 
            Value::Bulk(value)) =
            (key_variant, ttl_variant, val_variant)
        else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let Ok(ttl) = ttl_bytes.parse::<u64>() else {
            return Value::Error("error parse ttl".to_string());
        };

        match self.app_ctx.ant_db.setex(key,ttl, value) {
            Ok(_) => Value::String("OK".to_string()),
            Err(e) => Value::Error(e.to_string()),
        }

    }

    pub fn resp_expire(&self, mut values: Vec<Value>) -> Value {
        // 1. Validasi jumlah argumen
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'expire' command".to_string());
        }

        // 2. Ekstrak data secara konsisten menggunakan remove(0)
        let key_variant = values.remove(0);
        let ttl_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(ttl_str)) = (key_variant, ttl_variant) else {
            return Value::Error("parse error".to_string());
        };

         let Ok(ttl) = ttl_str.parse::<u64>() else {
            return Value::Error("error parse ttl".to_string());
        };

        match self.app_ctx.ant_db.expire(key,ttl) {
            Ok(_) => Value::String("OK".to_string()),
            Err(_) => Value::Null,
        }

    }
}

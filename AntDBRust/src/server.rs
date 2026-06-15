use std::{
    io::{BufReader, Cursor},
    sync::Arc,
};

use resp::{Decoder, Value};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_util::bytes::BytesMut;

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

    async fn child_open(self: ServerAntDbArc, mut socket: tokio::net::TcpStream) {
        let mut buf = BytesMut::with_capacity(65536);

        loop {
            match socket.read_buf(&mut buf).await {
                Ok(0) => return,
                Ok(_) => {}
                Err(_) => return,
            }

            let mut consumed = 0;

            loop {
                let remaining = &buf[consumed..];
                if remaining.is_empty() {
                    break;
                }

                let mut cursor = Cursor::new(remaining);
                let buf_reader = BufReader::new(&mut cursor);
                let mut decoder = Decoder::new(buf_reader);

                match decoder.decode() {
                    Ok(decoded) => {
                        // KUNCI OPTIMASI: cursor.position() akan melompat maju karena di-prefetch BufReader.
                        // Jika Decoder kamu mengekspos akses ke interior BufReader, kamu bisa kurangi dengan sisa buffer.
                        // Namun jika tidak, cara paling aman tanpa merusak posisi TCP stream adalah Solusi 1.
                        consumed += cursor.position() as usize;

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
                                        let response = self.resp_ping(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "COMMAND" => {
                                        let response = Value::Array(vec![]);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "SET" => {
                                        let response = &self.resp_set(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "SETEX" => {
                                        let response = &self.resp_setex(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "EXPIRE" => {
                                        let response = &self.resp_expire(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "GET" => {
                                        let response = &self.resp_get(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "HSET" => {
                                        let response = &self.resp_hset(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "HGET" => {
                                        let response = &self.resp_hget(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "DEL" => {
                                        let response = &self.resp_del(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    "EXISTS" => {
                                        let response = &self.resp_exists(values);
                                        let _ = socket.write_all(&response.encode()).await;
                                    }
                                    _ => {
                                        println!("command unhandled : {}", command_name);
                                        let err_msg =
                                            format!("ERR unknown command '{}'", command_name);
                                        let _ =
                                            socket.write_all(&Value::Error(err_msg).encode()).await;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            if consumed > 0 {
                let _ = buf.split_to(consumed);
            }
        }
    }

    fn resp_ping(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::String("PONG".to_string());
        }

        println!("DEBUG: PING detected with {} arguments", values.len());

        // Ambil argumen pertama (apapun isinya, entah Bulk atau String)
        // lalu echo balikkan langsung ke klien tanpa mengubah tipenya.
        values.remove(0)
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

        let (Value::Bulk(key), Value::Bulk(ttl_bytes), Value::Bulk(value)) =
            (key_variant, ttl_variant, val_variant)
        else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let Ok(ttl) = ttl_bytes.parse::<u64>() else {
            return Value::Error("error parse ttl".to_string());
        };

        match self.app_ctx.ant_db.setex(key, ttl, value) {
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

        match self.app_ctx.ant_db.expire(key, ttl) {
            Ok(_) => Value::String("OK".to_string()),
            Err(_) => Value::Null,
        }
    }

    pub fn resp_exists(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'exists' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        match self.app_ctx.ant_db.exist(key) {
            Ok(result) => Value::Integer(result),
            Err(_) => Value::Integer(0),
        }
    }

    pub fn resp_del(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'del' command".to_string());
        }

        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        match self.app_ctx.ant_db.del(key) {
            Ok(result) => Value::Integer(result.parse::<i64>().unwrap_or(0)),
            Err(_) => Value::Integer(0),
        }
    }
}

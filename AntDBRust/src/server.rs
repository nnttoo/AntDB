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
                let buf_reader = BufReader::with_capacity(1, &mut cursor);
                let mut decoder = Decoder::new(buf_reader);

                match decoder.decode() {
                    Ok(decoded) => {
                        consumed += cursor.position() as usize;

                        //println!("Decoded type: {:?}", &decoded);

                        match decoded {
                            Value::Array(mut values) => {
                                if values.is_empty() {
                                    continue;
                                }
                                let Value::Bulk(cmd_bytes) = values.remove(0) else {
                                    continue;
                                };
                                let command_name = cmd_bytes.to_uppercase();

                                let val_response = match command_name.as_str() {
                                    "CLIENT" => Value::String("OK".to_string()),
                                    "INFO" => Value::Bulk(
                                        "# Server\r\nAntDB_version:7.0.0\r\n".to_string(),
                                    ),
                                    "PING" => self.resp_ping(values),
                                    "COMMAND" => Value::Array(vec![]),
                                    "SET" => self.resp_set(values),
                                    "SETEX" => self.resp_setex(values),
                                    "EXPIRE" => self.resp_expire(values),
                                    "GET" => self.resp_get(values),
                                    "HSET" => self.resp_hset(values),
                                    "HGET" => self.resp_hget(values),
                                    "DEL" => self.resp_del(values),
                                    "EXISTS" => self.resp_exists(values),
                                    
                                    "TTL" => self.resp_ttl(values),
                                    _ => {
                                        println!("command unhandled : {}", command_name);
                                        let err_msg =
                                            format!("ERR unknown command '{}'", command_name);
                                        Value::Error(err_msg)
                                    }
                                };

                                _ = socket.write_all(&val_response.encode()).await;
                            }
                            _ => {
                                println!("mismatch ");
                                println!("Decoded type: {:?}", &decoded);
                            }
                        }
                    }
                    Err(e) => {
                        println!("ERROR DECODE {}", e);
                        let mut val = Value::Array(vec![]);
                        if remaining.starts_with(b"PING\r\n") || remaining.starts_with(b"ping\r\n")
                        {
                            println!("PING DETECTED");
                            val = Value::String("PONG".to_string());
                        }

                        _ = socket.write_all(&val.encode()).await;
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

        // Ambil argumen pertama
        let arg = values.remove(0);

        // Pastikan jika tipenya Bulk (dari redis-benchmark), kita kembalikan
        // sebagai String atau Bulk yang bersih agar dibaca sebagai valid reply oleh benchmark.
        match arg {
            Value::Bulk(text) => Value::String(text), // Mengubah Bulk menjadi Simple String seringkali lebih aman untuk benchmark
            Value::String(text) => Value::String(text),
            _ => Value::String("PONG".to_string()),
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

    fn resp_get(&self, mut values: Vec<Value>) -> Value {
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
            Ok(result) => Value::Integer(result),
            Err(_) => Value::Integer(0),
        }
    }

    pub fn resp_ttl(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'del' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db;
        match db.ttl(key) {
            Ok(result) => Value::Integer(result),
            Err(_) => Value::Integer(0),
        }
    }
}

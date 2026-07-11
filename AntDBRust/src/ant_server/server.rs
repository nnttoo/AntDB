use std::{
    io::{BufReader, Cursor},
    sync::Arc,
};

use resp::{Decoder, Value};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_util::bytes::BytesMut;

use crate::{app_ctx::AppCtxArc , utils_tools::BoxError};
use super::{
    server_resp::ServerAntDbResp
};

pub struct ServerAntDb {
    pub app_ctx: AppCtxArc,
    pub resp: ServerAntDbResp,
}

pub type ServerAntDbArc = Arc<ServerAntDb>;

impl ServerAntDb {
    pub fn new(app_ctx: AppCtxArc) -> Self {
        Self {
            app_ctx: app_ctx.clone(),
            resp: ServerAntDbResp::new(app_ctx),
        }
    }

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
                                    "PING" => self.resp.ping(values),
                                    "COMMAND" => Value::Array(vec![]),
                                    "SET" => self.resp.set(values),
                                    "SETEX" => self.resp.setex(values),
                                    "EXPIRE" => self.resp.expire(values),
                                    "GET" => self.resp.get(values),
                                    "HSET" => self.resp_hset(values),
                                    "HGET" => self.resp_hget(values),
                                    "DEL" => self.resp.del(values),
                                    "EXISTS" => self.resp.exists(values),

                                    "TTL" => self.resp.ttl(values),
                                    "PTTL" => self.resp_pttl(values),
                                    "PERSIST" => self.resp_persist(values),
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
    fn resp_hget(&self, mut values: Vec<Value>) -> Value {
        if values.len() < 2 {
            return Value::Error("ERR wrong number of arguments for 'hget' command".to_string());
        }

        let key_variant = values.remove(0);
        let field_variant = values.remove(0);

        let (Value::Bulk(key), Value::Bulk(field)) = (key_variant, field_variant) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };
        let db = &self.app_ctx.ant_db.db_hash;

        match db.hget(key, field) {
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
        let db = &self.app_ctx.ant_db.db_hash;

        match db.hset(key, field, value) {
            Ok(_) => Value::String("OK".to_string()),
            Err(e) => Value::Error(e.to_string()),
        }
    }

    pub fn resp_pttl(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'pttl' command".to_string());
        }
        let key_variant = values.remove(0);
        let Value::Bulk(key) = key_variant else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };

        let db = &self.app_ctx.ant_db.db;
        match db.pttl(key) {
            Ok(ms_result) => Value::Integer(ms_result), // Langsung ambil milidetik murninya
            Err(_) => Value::Integer(-2),
        }
    }

    pub fn resp_persist(&self, mut values: Vec<Value>) -> Value {
        if values.is_empty() {
            return Value::Error("ERR wrong number of arguments for 'pttl' command".to_string());
        }

        let Value::Bulk(key) = values.remove(0) else {
            return Value::Error("ERR syntax error or invalid argument type".to_string());
        };
        let db = &self.app_ctx.ant_db.db;
        match db.persist(key) {
            Ok(r) => Value::Integer(r),
            Err(d) => Value::Error(d.to_string()),
        }
    }
}

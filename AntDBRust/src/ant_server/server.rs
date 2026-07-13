use std::{io::Cursor, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_util::bytes::BytesMut;

use super::resp::ServerAntDbResp;
use crate::{
    ant_resp::value::{Value, parse_resp},
    ant_server::resp_hashmap::ServerAntDbRespHashMap,
    app_ctx::AppCtxArc,
    utils_tools::BoxError,
};

pub struct ServerAntDb {
    pub app_ctx: AppCtxArc,
    resp: ServerAntDbResp,
    resp_hashmap: ServerAntDbRespHashMap,
}

pub type ServerAntDbArc = Arc<ServerAntDb>;

impl ServerAntDb {
    pub fn new(app_ctx: AppCtxArc) -> Self {
        Self {
            app_ctx: app_ctx.clone(),
            resp: ServerAntDbResp::new(app_ctx.clone()),
            resp_hashmap: ServerAntDbRespHashMap::new(app_ctx),
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

    // Haryanto 11 July 2026

    fn create_response(&self, command_name: &str, values: Vec<Value>) -> Value {
        match command_name {
            "CLIENT" => Value::String("OK".to_string()),
            "INFO" => Value::Bulk("# Server\r\nAntDB_version:7.0.0\r\n".to_string()),
            "PING" => self.resp.ping(values),
            "COMMAND" => Value::Array(vec![]),
            "SET" => self.resp.set(values),
            "SETEX" => self.resp.setex(values),
            "EXPIRE" => self.resp.expire(values),
            "GET" => self.resp.get(values),

            "HSET" => self.resp_hashmap.hset(values),
            "HGET" => self.resp_hashmap.hget(values),
            "HDEL" => self.resp_hashmap.hdel(values),
            "HLEN" => self.resp_hashmap.hlen(values),
            "HEXISTS" => self.resp_hashmap.hexist(values),
            "HMGET" => self.resp_hashmap.hmget(values),
            "HKEYS" => self.resp_hashmap.hkeys(values),
            "HVALS" => self.resp_hashmap.hvals(values),
            "HGETALL" => self.resp_hashmap.hgetall(values),

            "DEL" => self.resp.del(values),
            "EXISTS" => self.resp.exists(values),
            "TTL" => self.resp.ttl(values),
            "PTTL" => self.resp.pttl(values),
            "PERSIST" => self.resp.persist(values),
            _ => {
                println!("command unhandled : {}", command_name);
                let err_msg = format!("ERR unknown command '{}'", command_name);
                Value::Error(err_msg)
            }
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

                // Menggunakan custom parser baru kita langsung ke cursor
                match parse_resp(&mut cursor) {
                    Ok(decoded) => {
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

                                let val_response = self.create_response(&command_name, values);
                                _ = socket.write_all(&val_response.encode()).await;
                            }
                            _ => {
                                println!("mismatch ");
                                println!("Decoded type: {:?}", &decoded);
                            }
                        }
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        }

                        // 2. Jika ternyata raw/inline PING, konsumsi 6 bytes lalu lanjut parse sisanya
                        if remaining.starts_with(b"PING\r\n") || remaining.starts_with(b"ping\r\n")
                        {
                            println!("PING DETECTED");
                            let val = Value::String("PONG".to_string());
                            _ = socket.write_all(&val.encode()).await;

                            consumed += 6; // Majukan pointer buffer sebanyak 6 byte ("PING\r\n")
                            continue; // Lanjutkan parsing data berikutnya yang ada di buffer
                        }

                        // if new protocol incomning break
                        println!("ERROR DECODE {}", e);
                        let val = Value::Array(vec![]);
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
 
}

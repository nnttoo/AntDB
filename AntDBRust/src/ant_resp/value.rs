// Haryanto 11 July 2026

use std::io::{self, BufRead};

/// Represents a RESP value matching the original `resp::Value` structure.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Value {
    Null,
    Integer(i64),
    String(String),
    Error(String),
    Bulk(String),
    BufBulk(Vec<u8>),
    Array(Vec<Value>),
}

impl Value {
    /// Encodes the `Value` into its RESP binary representation.
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.buf_serialize(&mut buf);
        buf
    }

    fn buf_serialize(&self, buf: &mut Vec<u8>) {
        const CRLF: &[u8] = b"\r\n";
        match self {
            Value::Null => {
                buf.extend_from_slice(b"$-1\r\n");
            }
            Value::String(s) => {
                buf.push(b'+');
                buf.extend_from_slice(s.as_bytes());
                buf.extend_from_slice(CRLF);
            }
            Value::Error(e) => {
                buf.push(b'-');
                buf.extend_from_slice(e.as_bytes());
                buf.extend_from_slice(CRLF);
            }
            Value::Integer(n) => {
                buf.push(b':');
                buf.extend_from_slice(n.to_string().as_bytes());
                buf.extend_from_slice(CRLF);
            }
            Value::Bulk(s) => {
                buf.push(b'$');
                buf.extend_from_slice(s.len().to_string().as_bytes());
                buf.extend_from_slice(CRLF);
                buf.extend_from_slice(s.as_bytes());
                buf.extend_from_slice(CRLF);
            }
            Value::BufBulk(bytes) => {
                buf.push(b'$');
                buf.extend_from_slice(bytes.len().to_string().as_bytes());
                buf.extend_from_slice(CRLF);
                buf.extend_from_slice(bytes);
                buf.extend_from_slice(CRLF);
            }
            Value::Array(elements) => {
                buf.push(b'*');
                buf.extend_from_slice(elements.len().to_string().as_bytes());
                buf.extend_from_slice(CRLF);
                for item in elements {
                    item.buf_serialize(buf);
                }
            }
        }
    }
}

/// Parses a single RESP value from a reader implementing `BufRead`.
pub fn parse_resp<R: BufRead>(reader: &mut R) -> io::Result<Value> {
    let mut prefix = [0u8; 1];
    reader.read_exact(&mut prefix)?;

    match prefix[0] {
        b'+' => {
            let line = read_line(reader)?;
            let s = String::from_utf8(line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(Value::String(s))
        }
        b'-' => {
            let line = read_line(reader)?;
            let s = String::from_utf8(line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(Value::Error(s))
        }
        b':' => {
            let line = read_line(reader)?;
            let s = std::str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let n = s
                .parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            Ok(Value::Integer(n))
        }
        b'$' => {
            let line = read_line(reader)?;
            let len_str = std::str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let len = len_str
                .parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            if len == -1 {
                return Ok(Value::Null);
            }
            if len < 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid Bulk String length",
                ));
            }

            let len = len as usize;
            let mut buf = vec![0u8; len];
            reader.read_exact(&mut buf)?;

            let mut crlf = [0u8; 2];
            reader.read_exact(&mut crlf)?;
            if crlf != [b'\r', b'\n'] {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Missing CRLF after Bulk String content",
                ));
            }

            match String::from_utf8(buf) {
                Ok(s) => Ok(Value::Bulk(s)),
                Err(e) => Ok(Value::BufBulk(e.into_bytes())),
            }
        }
        b'*' => {
            let line = read_line(reader)?;
            let len_str = std::str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let len = len_str
                .parse::<i64>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            if len == -1 {
                return Ok(Value::Null);
            }
            if len < 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid Array length",
                ));
            }

            let len = len as usize;
            let mut elements = Vec::with_capacity(len);
            for _ in 0..len {
                elements.push(parse_resp(reader)?);
            }

            Ok(Value::Array(elements))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Unknown RESP prefix: {}", prefix[0] as char),
        )),
    }
}

fn read_line<R: BufRead>(reader: &mut R) -> io::Result<Vec<u8>> {
    let mut line = Vec::new();
    reader.read_until(b'\n', &mut line)?;

    if line.ends_with(&[b'\r', b'\n']) {
        line.truncate(line.len() - 2);
        Ok(line)
    } else {
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Line did not end with CRLF",
        ))
    }
}

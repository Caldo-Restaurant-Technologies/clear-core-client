use std::{
    fmt,
    io::{BufRead, BufReader,  Write},
    net::{TcpStream, ToSocketAddrs},
};

use std::error;
use std::fmt::Formatter;

use crate::BUFFER_LENGTH;

pub const STX: u8 = 2;
pub const CR: u8 = 13;
pub const RESULT_IDX: usize = 3;
pub const FAILED_REPLY: u8 = b'?';

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: error::Error + Send + Sync + 'static> From<T> for Error {
    fn from(value: T) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

pub fn check_reply(reply: &[u8]) -> Result<(), Error> {
    if reply[RESULT_IDX] == FAILED_REPLY {
        Err(Error {
            message: std::str::from_utf8(reply)?.to_string(),
        })
    } else {
        Ok(())
    }
}

pub struct Controller {
    stream: TcpStream,
}

impl Controller {
    pub fn new<T>(addr: T) -> std::io::Result<Self>
    where
        T: ToSocketAddrs + Send + 'static,
    {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    pub fn send_recv(&mut self, msg: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut reader = BufReader::new(&mut self.stream);
        let mut response = Vec::with_capacity(BUFFER_LENGTH);
        reader.get_mut().write_all(msg)?;
        reader.read_until(b'\0',&mut response)?;
        Ok(response)
    }
}

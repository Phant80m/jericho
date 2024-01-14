use super::TcpSender;
use serde::Serialize;
use std::io::{self, Write};
use std::net::TcpStream;

impl<T> TcpSender<T>
where
    T: Serialize,
{
    pub fn new(content: T) -> Self {
        TcpSender { content }
    }

    pub fn send(&self, stream: &mut TcpStream) -> io::Result<()> {
        let serialized_message = serde_json::to_string(self).unwrap();
        stream.write_all(serialized_message.as_bytes())?;
        stream.write_all(b"\n")?;
        Ok(())
    }
}

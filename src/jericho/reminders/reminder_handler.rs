use super::Reminder;
use crate::server::TcpSender;
use std::io;
use std::net::TcpStream;

impl Reminder {
    pub fn new(title: String, timestamp: String) -> Self {
        Self { title, timestamp }
    }

    pub fn send(&self) -> io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;

        let tcp_sender = TcpSender::new(self);
        tcp_sender.send(&mut stream)?;

        Ok(())
    }
}

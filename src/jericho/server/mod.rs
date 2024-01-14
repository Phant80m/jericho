mod tcp_sender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpSender<T>
where
    T: Serialize,
{
    content: T,
}

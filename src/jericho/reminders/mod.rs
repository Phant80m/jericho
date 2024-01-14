use serde::{Deserialize, Serialize};
mod reminder_handler;
#[derive(Debug, Serialize, Deserialize)]
pub struct Reminder {
    title: String,
    timestamp: String,
}

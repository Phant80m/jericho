use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use dialog::DialogBox;
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::io::{self, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Serialize, Deserialize)]
struct TcpReceive<T> {
    content: T,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Reminder {
    title: String,
    timestamp: String,
}

impl Reminder {
    fn new(title: String, timestamp: String) -> Self {
        Reminder { title, timestamp }
    }

    fn get_timestamp(&self) -> Result<NaiveDateTime> {
        let format = "%d/%m/%Y:%H:%M";
        NaiveDateTime::parse_from_str(&self.timestamp, format).map_err(|e| anyhow!(e))
    }
}

fn handle_client(
    stream: TcpStream,
    reminder_queue: Arc<Mutex<BinaryHeap<Reminder>>>,
) -> Result<()> {
    let reader = BufReader::new(&stream);

    for line in reader.lines() {
        let line = line.unwrap();
        let tcpreceive: TcpReceive<Reminder> = serde_json::from_str(&line).unwrap();
        let mut data: Reminder = tcpreceive.content;
        data = Reminder::new(data.title, data.timestamp); // gen uuid
        let mut queue = reminder_queue.lock().expect("Mutex lock failed");
        queue.push(data.clone());
        let data = data.clone();
        println!("Reminder set: {} at {}", data.title, data.timestamp);
    }

    Ok(())
}

fn handle_reminders(reminder_queue: Arc<Mutex<BinaryHeap<Reminder>>>) {
    loop {
        thread::sleep(std::time::Duration::from_secs(1));

        let current_time = Local::now().naive_local();
        let mut queue = reminder_queue.lock().expect("Mutex lock failed");

        while let Some(reminder) = queue.peek() {
            if let Ok(reminder_time) = reminder.get_timestamp() {
                if reminder_time <= current_time {
                    println!(
                        "Reminder '{}' was displayed at: {}",
                        reminder.title, reminder.timestamp
                    );
                    dialog::Message::new(format!(
                        "Reminder: {} set for {}",
                        reminder.title, reminder.timestamp
                    ))
                    .title(reminder.title.clone())
                    .show()
                    .expect("Could not display dialog box");
                    queue.pop(); // Remove the reminder from the queue
                } else {
                    break;
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let reminder_queue = Arc::new(Mutex::new(BinaryHeap::new()));

    let reminder_queue_clone = Arc::clone(&reminder_queue);
    thread::spawn(move || handle_reminders(reminder_queue_clone));

    for stream in listener.incoming() {
        let stream = stream?;
        let reminder_queue_clone = Arc::clone(&reminder_queue);
        std::thread::spawn(move || handle_client(stream, reminder_queue_clone));
    }

    Ok(())
}

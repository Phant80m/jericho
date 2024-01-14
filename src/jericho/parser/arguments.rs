use super::{Args, Command};
use crate::reminders::Reminder;
use anyhow::Result;
use clap::Parser;
use daemonize::Daemonize;
use std::process::Command as PCommand;
impl Args {
    pub fn build() -> Self {
        Args::parse()
    }
    pub fn handle(&self) -> Result<()> {
        match &self.subcommand {
            Some(Command::Set { title, timestamp }) => {
                Reminder::new(title.to_owned(), timestamp.to_owned()).send()?;
            }
            Some(Command::Init) => find_and_run_jericho_daemon(),
            _ => {}
        }
        Ok(())
    }
}

fn find_and_run_jericho_daemon() {
    let jericho_daemon_path = which::which("jericho_daemon")
        .expect("jericho_daemon not found in PATH")
        .to_string_lossy()
        .to_string();

    // Create a Daemonize instance to daemonize the process
    let daemonize = Daemonize::new()
        .pid_file("/tmp/jericho_daemon.pid")
        .chown_pid_file(true)
        .user(std::env::var("USER").unwrap().as_str());

    match daemonize.start() {
        Ok(_) => {
            // Inside the daemonized process

            let status = PCommand::new(&jericho_daemon_path)
                .spawn()
                .map(|mut child| child.wait())
                .and_then(|status| {
                    status.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
                })
                .expect("Failed to execute jericho_daemon");

            if !status.success() {
                eprintln!("jericho_daemon exited with non-zero status");
            }
        }
        Err(e) => eprintln!("Error daemonizing: {}", e),
    }
}

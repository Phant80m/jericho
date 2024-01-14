pub mod parser;
pub mod reminders;
pub mod server;

use crate::parser::Args;
use anyhow::Result;
fn main() -> Result<()> {
    Args::build().handle()?;
    Ok(())
}

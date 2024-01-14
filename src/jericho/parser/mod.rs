use clap::{Parser, Subcommand};
mod arguments;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// -> Set a reminder
    #[clap(name = "set-reminder")]
    Set { title: String, timestamp: String },
    #[clap(name = "init")]
    /// init the daemon
    Init,
}

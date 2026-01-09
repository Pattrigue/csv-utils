use clap::Parser;

use crate::commands::Command;

#[derive(Parser, Debug)]
#[command(name = "csvu", version, about = "CSV utilities")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

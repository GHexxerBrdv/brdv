use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commit_data;
mod controller;

use crate::commit_data::Commit;
use crate::controller::{commit, init, log};

#[derive(Parser)]
#[command(name = "brdv", about = "A Project repository manager")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Init,
    Commit {
        #[arg(short, long)]
        message: String,
    },
    Log,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => init(),
        Command::Commit { message } => commit(&message),
        Command::Log => log(),
    };

    Ok(())
}

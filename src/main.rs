use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commit_data;
mod controller;
mod updation;

use crate::controller::{commit, init, log};
use crate::updation::update;

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
    Update,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let _ = match cli.command {
        Command::Init => init(),
        Command::Commit { message } => commit(&message),
        Command::Log => log(),
        Command::Update => update(),
    };

    Ok(())
}

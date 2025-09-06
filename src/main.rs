mod cli;
mod commands;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{add_task, clear_tasks, complete_task, delete_task, list_tasks};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { description } => {
            if let Err(e) = add_task(description) {
                eprintln!("Error adding task: {}", e);
                std::process::exit(1);
            }
        }
        Commands::List => {
            if let Err(e) = list_tasks() {
                eprintln!("Error listing tasks: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Done { id } => {
            if let Err(e) = complete_task(*id) {
                eprintln!("Error completing task: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Delete { id } => {
            if let Err(e) = delete_task(*id) {
                eprintln!("Error deleting task: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Clear => {
            if let Err(e) = clear_tasks() {
                eprintln!("Error clearing tasks: {}", e);
                std::process::exit(1);
            }
        }
    }
}

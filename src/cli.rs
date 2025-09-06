use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "A simple command-line task manager")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Description of the task
        description: String,
    },
    /// List all tasks
    List,
    /// Mark a task as done
    Done {
        /// ID of the task to mark as done
        id: u32,
    },
    /// Delete a task
    Delete {
        /// ID of the task to delete
        id: u32,
    },
    /// Clear all tasks
    Clear,
}

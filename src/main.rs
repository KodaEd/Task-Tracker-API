use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
    },
    Update {
        id: usize,
        description: String
    },
    Delete {
        id: usize
    },
    MarkInProgress {
        id: usize
    },
    MarkDone {
        id: usize
    },
    List {
        status: Option<String>
    },
}
#[derive(Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    status: Option<String>
}

fn main() {
    // Checks if the local JSON file is there
    let file_string = fs::read_to_string("db.json");
    let _tasks: Vec<Task> = match &file_string {
        Ok(some_string) => {
            // If it can be deserialised
            match serde_json::from_str(some_string) {
                Ok(deserialized_tasks) => deserialized_tasks,
                Err(..) => {
                    println!("Failed to deserialize tasks, Creating a new database");
                    Vec::new()
                }
            }
        }
        Err(..) => Vec::new(),
    };

    // Get the args of the program
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { description } => {
            println!("Adding task: {}", description);
        }
        Commands::Update { id, description } => {
            println!("Updating task {} with new description: {}", id, description);
        }
        Commands::Delete { id } => {
            println!("Deleting task: {}", id);
        }
        Commands::MarkInProgress { id } => {
            println!("Marking task {} as in progress", id);
        }
        Commands::MarkDone { id } => {
            println!("Marking task {} as done", id);
        }
        Commands::List { status } => {
            if let Some(status) = status {
                println!("Listing tasks with status: {:?}", status);
            } else {
                println!("Listing all tasks");
            }
        }
    }
}

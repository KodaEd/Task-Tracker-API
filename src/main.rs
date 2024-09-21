use std::fs;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


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

#[derive(Clone)]
struct TaskDetails {
    description: String,
    status: Option<String>
}

fn main() {
    // Checks if the local JSON file is there
    let file_string = fs::read_to_string("db.json");
    let tasks: HashMap<usize, TaskDetails> = match &file_string {
        Ok(some_string) => {
            // If it can be deserialised
            match serde_json::from_str::<Vec<Task>>(some_string) {
                Ok(deserialized_tasks) => {
                    let tasks_map: HashMap<usize, TaskDetails> = HashMap::new();
                    for x in deserialized_tasks.into_iter() {
                        tasks_map.insert(x.id, TaskDetails { description: x.description, status: x.status });
                    }
                    tasks_map
                },
                Err(..) => {
                    println!("Failed to deserialize tasks, Creating a new database");
                    HashMap::new()
                }
            }
        }
        Err(..) => HashMap::new(),
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

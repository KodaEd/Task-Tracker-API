use std::{fs, ops::{Deref, Not}};

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

#[derive(Clone, Debug)]
struct TaskDetails {
    description: String,
    status: Option<String>
}

fn main() {
    // Checks if the local JSON file is there
    let file_string = fs::read_to_string("db.json");
    let mut tasks: HashMap<usize, TaskDetails> = match &file_string {
        Ok(some_string) => {
            // If it can be deserialised
            match serde_json::from_str::<Vec<Task>>(some_string) {
                Ok(deserialized_tasks) => {
                    let mut tasks_map: HashMap<usize, TaskDetails> = HashMap::new();
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
            for i in 0..usize::max_value() {
                if tasks.contains_key(&i) {
                    continue;
                };
                tasks.insert(i, TaskDetails { description: description.to_string(), status: None });
            }
        }
        Commands::Update { id, description } => {
            // find the id
            match tasks.get_mut(id) {
                Some(value) => {
                    value.description = description.to_string()
                },
                None => println!("Some Error")
            }
        }
        Commands::Delete { id } => {
            if tasks.contains_key(id) {
                tasks.remove(id);
            } else {
                println!("Id not found");
            }
        }
        Commands::MarkInProgress { id } => {
            match tasks.get_mut(id) {
                Some(value) => {
                    value.status = Some("In Progress".to_string())
                },
                None => println!("Some Error")
            }
        }
        Commands::MarkDone { id } => {
            match tasks.get_mut(id) {
                Some(value) => {
                    value.status = Some("Done".to_string())
                },
                None => println!("Some Error")
            }
        }
        Commands::List { status } => {
            for (key, value) in &tasks {
                if *status == value.status {
                    println!("{} {:?}", key, value)
                }
            }
        }
    }
}

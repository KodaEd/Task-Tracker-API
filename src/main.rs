use std::{fs, ops::Not};

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
        status: String
    },
}
#[derive(Clone, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    status: String
}

#[derive(Clone, Debug)]
struct TaskDetails {
    description: String,
    status: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Checks if the local JSON file is there
    let file_string = fs::read_to_string("db.json");

    // Brings all the tasks into a hash map for quicker and easier updating
    let mut tasks: HashMap<usize, TaskDetails> = match &file_string {
        Ok(some_string) => {
            // If it can be deserialised
            match serde_json::from_str::<Vec<Task>>(some_string) {
                Ok(deserialized_tasks) => {
                    let mut tasks_map: HashMap<usize, TaskDetails> = HashMap::new();
                    // Placing all the info into the map
                    for x in deserialized_tasks.into_iter() {
                        tasks_map.insert(x.id, TaskDetails { description: x.description, status: x.status });
                    }
                    tasks_map
                },
                // Default case for an error with the reading.
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
            // Finds the first available id.
            for i in 0.. {
                if tasks.contains_key(&i) {
                    continue;
                };
                tasks.insert(i, TaskDetails { description: description.to_string(), status: String::from("Todo") });
                break;
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
                    value.status = String::from("In Progress")
                },
                None => println!("Some Error")
            }
        }
        Commands::MarkDone { id } => {
            match tasks.get_mut(id) {
                Some(value) => {
                    value.status = String::from("Done")
                },
                None => println!("Some Error")
            }
        }
        Commands::List { status } => {
            if (*status == String::from("in-progress") || *status == String::from("todo") || *status == String::from("done")).not() {
                println!("Error");
            }

            for (key, value) in &tasks {
                if *status == value.status {
                    println!("{} {:?}", key, value)
                }
            }
        }
    }

    println!("Reached here!");

    // Putting it back into json
    // Make it into a vec of values
    let mut result: Vec<Task> = Vec::new();
    for (key, value) in &tasks {
        result.push(Task{id: *key, description: value.description.clone(), status: value.status.clone()});
    }

    let result_string = serde_json::to_string(&result)?;
    fs::write("db.json", result_string)?;

    Ok(())
}

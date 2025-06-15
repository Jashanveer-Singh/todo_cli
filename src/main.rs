use std::{env, process};
use todo::run;
use todo::task::{TaskManager, Task};

fn main() {
    
    let data_file_path = dirs::data_dir()
        .unwrap_or_else(||{
            eprintln!("Error finding data directory: No data directory found!");
            process::exit(1);
        })
        .join("todo")
        .join("tasks");

    let tasks:Vec<Task> = TaskManager::load(&data_file_path).unwrap_or_else(|error| {
        eprintln!("Error parsing tasks: {error}");
        process::exit(1);
    });

    if let Err(error) = run(env::args(), tasks, data_file_path) {
        eprintln!("Application Error: {error}");
        process::exit(1);
    };
}

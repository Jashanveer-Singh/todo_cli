pub mod task;
pub mod error;

use error::MyError;
use task::{Task, TaskManager};
use std::{
    collections::HashSet,
    path::PathBuf,
    io::{self, Write},
};


const HELP: &str = r#"todo - A simple command-line task manager 📝

USAGE:
    todo <COMMAND> [ARGS]

COMMANDS:
    add <task>        Add a single new task
    show              Display all tasks
    update            Update an existing task
    delete            Delete a task

Use of any other command will display this help message.

EXAMPLES:
    todo add Read a book
    todo show
    todo update
    todo delete"#;

pub fn run(
    mut args: impl Iterator<Item = String>,
    mut tasks: Vec<Task>,
    data_file_path: PathBuf,
) -> Result<(), MyError> {
    args.next();

    match &args.next().as_deref() {
        Some("add") => {
            tasks.push(Task::build(args)?);
            tasks.save(&data_file_path)?;
        }
        Some("show") => {
           tasks.display()?;
        }
        Some("delete") => {
            tasks.display()?;
            print!("\nChoose Tasks to delete: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)?;
            let to_remove: HashSet<usize> = input
                .split(',')
                .map(|num_str| num_str.trim().parse::<usize>())
                .try_fold(HashSet::new(), |mut acc,parsed_num| -> Result<HashSet<usize>, MyError> {
                    acc.insert(parsed_num?);
                    Ok(acc)
                })?;

            let mut idx = 0;

            tasks.retain(move |_| {
                idx += 1;
                !to_remove.contains(&idx)
            });

            tasks.save(&data_file_path)?;
        }
        Some("update") => {
            tasks.display()?;
            print!("\nChoose to task to update: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)?;
            let to_update: HashSet<usize> = input
                .split(',')
                .map(|num| num.trim().parse::<usize>())
                .try_fold(HashSet::new(), |mut acc,parsed_num| -> Result<HashSet<usize>, MyError> {
                    acc.insert(parsed_num?);
                    Ok(acc)
                })?;
            for i in to_update {
                if i <= tasks.len() {
                    tasks[i - 1].update();
                }
            }

            tasks.save(&data_file_path)?;
        }
        Some(_) => {
            println!("Invalid command!\n\n{HELP}");
        }
        None => println!("{HELP}"),
    }

    Ok(())
}


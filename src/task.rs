use crate::error::MyError;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::PathBuf,
    io::Write,
};

#[derive(Serialize, Deserialize)]
enum Status {
    Waiting,
    Ongoing,
    Completed,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Waiting => write!(f, "Waiting"),
            Status::Ongoing => write!(f, "Ongoing"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    status: Status,
}

impl Task {
    pub fn new(args: impl Iterator<Item = String>) -> Self {
        let title = args.collect::<Vec<String>>().join(" ");
        Task {
            title,
            status: Status::Waiting,
        }
    }

    pub fn update(&mut self) {
        if let Status::Waiting = self.status {
            self.status = Status::Ongoing;
        } else {
            self.status = Status::Completed;
        }
    }
}

pub trait TaskManager: Sized {
    fn load(data_file_path: &PathBuf) -> Result<Self, MyError>;
    fn save(&mut self, data_file_path: &PathBuf)-> Result<(), MyError>;
    fn display(&self);
}

impl TaskManager for Vec<Task> {
    fn load(data_file_path: &PathBuf) -> Result<Self, MyError > {
        if !data_file_path.exists() {
            return Ok(vec![]);
        }

         fs::read_to_string(&data_file_path)?
            .lines()
            .map(|line| serde_json::from_str(&line).map_err(MyError::from))
            .collect()

    }

    fn save(&mut self, data_file_path: &PathBuf) -> Result<(), MyError> {
        if !data_file_path.parent().unwrap().exists() {
            fs::create_dir_all(data_file_path.parent().unwrap())?;
        }
        let mut data_file = File::create(data_file_path)?;

        for task in self {
            let json_task = serde_json::to_string(task)?;
            writeln!(data_file, " {}", json_task)?;
        }
        Ok(())
    }

    fn display(&self) {
        print!("SrNo | Task                                     | Status\n");
        print!("-----+------------------------------------------+-------------------\n");
        for (i, task) in self.iter().enumerate() {
            print!("{:<4} | {:<40} | {}\n", i + 1, task.title, task.status);
        }
    }
}


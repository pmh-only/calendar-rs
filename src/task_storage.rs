use std::{fs, error::Error};

use crate::task::Task;

pub struct TaskStorage {
  pub tasks: Vec<Task>
}

impl TaskStorage {
  pub fn new () -> TaskStorage {
    TaskStorage { tasks: vec![] }
  }

  pub fn load_from_disk (&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
    let raw_file = fs::read_to_string(file_path)?;
    let tasks = raw_file
      .split('\n')
      .map(|x| Task::try_from(x))
      .filter(|x| x.is_ok())
      .map(|x| x.unwrap())
      .collect();

    self.tasks = tasks;

    Ok(())
  }

  pub fn save_from_disk (&self, file_path: &str) -> Result<(), Box<dyn Error>> {
    let raw_file = self.tasks.iter()
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
      .join("\n");

    fs::write(file_path, raw_file + "\n")?;
    Ok(())
  }
}

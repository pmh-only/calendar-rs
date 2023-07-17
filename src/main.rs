use std::error::Error;

use task::{TaskStatus, TaskProximity};
use task_storage::TaskStorage;

use crate::task::Task;

mod task;
mod datetime;

mod task_storage;

fn main() -> Result<(), Box<dyn Error>> {
  let mut task_storage = TaskStorage::new();
  let task = Task {
    datetime: (
      (2023, 10, 22),
      (23, 10)
    ).try_into()?,
    status: TaskStatus::Todo,
    proximity: TaskProximity::Medium,
    content: "Hello.".to_owned()
  };

  task_storage.tasks.push(task);
  task_storage.save_from_disk("./test.txt")?;

  Ok(())
}

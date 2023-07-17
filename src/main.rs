use datatypes::task::{Task, TaskConversationError};

use crate::datatypes::task::TaskProximity;

mod datatypes;

fn main() -> Result<(), TaskConversationError> {
  let task_str = "2023-01-01.10:30 VH Td hi";
  let task = Task::try_from(task_str.to_string())?;

  println!("{}", matches!(task.proximity, TaskProximity::VeryHigh));

  Ok(())
}

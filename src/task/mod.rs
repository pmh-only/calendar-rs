use std::fmt::Display;

use crate::datetime::{Datetime, error::DatetimeError};

use self::error::TaskConversationError;

pub mod error;

pub enum TaskProximity {
  VeryHigh,
  High,
  Medium,
  Low,
  VeryLow
}

impl TryFrom<&str> for TaskProximity {
  type Error = TaskConversationError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "VH" => Ok(TaskProximity::VeryHigh),
      "H" => Ok(TaskProximity::High),
      "M" => Ok(TaskProximity::Medium),
      "L" => Ok(TaskProximity::Low),
      "VL" => Ok(TaskProximity::VeryLow),

      _ => Err(TaskConversationError::RangeError)
    }
  }
}

impl Into<String> for TaskProximity {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Display for TaskProximity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      TaskProximity::VeryHigh => "VH",
      TaskProximity::High => "H",
      TaskProximity::Medium => "M",
      TaskProximity::Low => "L",
      TaskProximity::VeryLow => "VL"
    })
  }
}

pub enum TaskStatus {
  Todo,
  InProgress,
  Finished,
  Deleted
}

impl TryFrom<&str> for TaskStatus {
  type Error = TaskConversationError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Td" => Ok(TaskStatus::Todo),
      "In" => Ok(TaskStatus::InProgress),
      "Fn" => Ok(TaskStatus::Finished),
      "Dl" => Ok(TaskStatus::Deleted),

      _ => Err(TaskConversationError::RangeError)
    }
  }
}

impl Into<String> for TaskStatus {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Display for TaskStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      TaskStatus::Todo => "Td",
      TaskStatus::InProgress => "In",
      TaskStatus::Finished => "Fn",
      TaskStatus::Deleted => "Dl"
    })
  }
}

pub struct Task {
  pub datetime: Datetime,
  pub proximity: TaskProximity,
  pub status: TaskStatus,
  pub content: String
}

impl TryFrom<&str> for Task {
  type Error = TaskConversationError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let splited: Vec<&str> = value.split(' ').collect();

    if splited.len() < 4 {
      return Err(TaskConversationError::TooFewArgumentsError);
    }

    let parsed_datetime =
      Datetime::try_from(splited[0]);

    if parsed_datetime.is_err() {
      return match parsed_datetime.err().unwrap() {
        DatetimeError::RangeError =>
          Err(TaskConversationError::RangeError),
        
        DatetimeError::ParseError(parse_int_error) =>
          Err(TaskConversationError::ParseError(parse_int_error)),

        DatetimeError::TooFewArgumentsError =>
          Err(TaskConversationError::TooFewArgumentsError)
      };
    }

    let parsed_proximity =
      TaskProximity::try_from(splited[1])?;

    let parsed_status =
      TaskStatus::try_from(splited[2])?;

    Ok(Task {
      datetime: parsed_datetime.unwrap(),
      proximity: parsed_proximity,
      status: parsed_status,
      content: splited[3].into()
    })
  }
}

impl Into<String> for Task {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Display for Task {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f, "{} {} {} {}",
      self.datetime,
      self.proximity,
      self.status,
      self.content
    )
  }
}

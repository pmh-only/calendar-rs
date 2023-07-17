use std::{num::ParseIntError, fmt::{Display, Formatter, self}};

#[derive(Debug)]
pub enum TaskConversationError {
  RangeError,
  ParseError(ParseIntError),
  TooFewArgumentsError
}

impl Display for TaskConversationError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      TaskConversationError::RangeError =>
        String::from("This Task has values that are out of range."),
      
      TaskConversationError::ParseError(parse_int_error) =>
        format!("Error occurred during parse integer: {}", parse_int_error),

      TaskConversationError::TooFewArgumentsError =>
        String::from("Too few arguments.")
    })
  }
}

use std::{error::Error, num::ParseIntError, fmt::{Display, Formatter, self}};

#[derive(Debug)]
pub enum DatetimeError {
  RangeError,
  ParseError(ParseIntError),
  TooFewArgumentsError
}

impl Display for DatetimeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", match self {
      DatetimeError::RangeError =>
        String::from("This Datetime has values that are out of range."),
      
      DatetimeError::ParseError(parse_int_error) =>
        format!("Error occurred during parse integer: {}", parse_int_error),

      DatetimeError::TooFewArgumentsError =>
        String::from("Too few arguments.")
    })
  }
}

impl Error for DatetimeError {}

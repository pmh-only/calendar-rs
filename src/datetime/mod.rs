use std::fmt::Display;

use self::error::DatetimeError;

pub mod error;

pub struct Date {
  pub year: u16,
  pub month: u8,
  pub day: u8
}

impl Date {
  pub fn new (year: u16, month: u8, day: u8) -> Result<Date, DatetimeError> {
    let created = Date { year, month, day };

    if !created.check_range() {
      return Err(DatetimeError::RangeError);
    }

    Ok(created)
  }
  
  fn check_range (&self) -> bool {
    (1..13).contains(&self.month) &&
    (1..self.get_possible_day_max() + 1).contains(&self.day)
  }

  fn check_leap_year (&self) -> bool {
    self.year % 4 == 0 && (
      self.year % 100 != 0 ||
      self.year % 400 == 0
    )
  }

  fn get_possible_day_max (&self) -> u8 {
    match self.month {
      1 | 3 | 5 | 7 | 8 | 10 | 12
        => 31,
      
      4 | 6 | 9 | 11
        => 30,

      2 => if self.check_leap_year() { 29 } else { 28 }

      _ => 0
    }
  }
}

impl TryFrom<(i32, i32, i32)> for Date {
  type Error = DatetimeError;
  fn try_from(value: (i32, i32, i32)) -> Result<Self, Self::Error> {
    return Date::new(
      value.0 as u16,
      value.1 as u8,
      value.2 as u8);
  }
}

impl TryFrom<&str> for Date {
  type Error = DatetimeError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let splited: Vec<&str> = value.split("-").collect();
    
    if splited.len() < 3 {
      return Err(DatetimeError::TooFewArgumentsError);
    }

    let parsed_year = splited[0].parse::<u16>();
    if parsed_year.is_err() {
      return Err(DatetimeError::ParseError(parsed_year.err().unwrap()));
    }
    
    let parsed_month = splited[1].parse::<u8>();
    if parsed_month.is_err() {
      return Err(DatetimeError::ParseError(parsed_month.err().unwrap()));
    }
    
    let parsed_day = splited[2].parse::<u8>();
    if parsed_day.is_err() {
      return Err(DatetimeError::ParseError(parsed_day.err().unwrap()));
    }

    let parsed_date = Date::new(
      parsed_year.unwrap(),
      parsed_month.unwrap(),
      parsed_day.unwrap(),
    );

    Ok(parsed_date?)
  }
}

impl Into<String> for Date {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Display for Date {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {  
    write!(f, "{}-{:0>2}-{:0>2}", self.year, self.month, self.day)
  }
}

pub struct Time {
  pub hour: u8,
  pub minute: u8
}

impl Time {
  pub fn new (hour: u8, minute: u8) -> Result<Time, DatetimeError> {
    let created = Time { hour, minute };

    if !created.check_range() {
      return Err(DatetimeError::RangeError);
    }

    Ok(created)
  }

  fn check_range (&self) -> bool {
    (0..24).contains(&self.hour) &&
    (0..60).contains(&self.minute)
  }
}

impl TryFrom<(i32, i32)> for Time {
  type Error = DatetimeError;
  fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
    return Time::new(
      value.0 as u8,
      value.1 as u8);
  }
}

impl TryFrom<&str> for Time {
  type Error = DatetimeError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let splited: Vec<&str> = value.split(":").collect();
    
    if splited.len() < 2 {
      return Err(DatetimeError::TooFewArgumentsError);
    }

    let parsed_hour = splited[0].parse::<u8>();
    if parsed_hour.is_err() {
      return Err(DatetimeError::ParseError(parsed_hour.err().unwrap()));
    }
    
    let parsed_minute = splited[1].parse::<u8>();
    if parsed_minute.is_err() {
      return Err(DatetimeError::ParseError(parsed_minute.err().unwrap()));
    }

    let parsed_time = Time::new(
      parsed_hour.unwrap(),
      parsed_minute.unwrap(),
    );

    Ok(parsed_time?)
  }
}

impl Into<String> for Time {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Display for Time {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:0>2}:{:0>2}", self.hour, self.minute)
  }
}

pub struct Datetime {
  pub date: Date,
  pub time: Time
}

impl TryFrom<((i32, i32, i32), (i32, i32))> for Datetime {
  type Error = DatetimeError;
  fn try_from(value: ((i32, i32, i32), (i32, i32))) -> Result<Self, Self::Error> {
    return Ok(Datetime { date: value.0.try_into()?, time: value.1.try_into()? })
  }
}

impl TryFrom<&str> for Datetime {
  type Error = DatetimeError;
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let splited: Vec<String> = value.split(".").map(|x| x.to_owned()).collect();
    
    if splited.len() < 2 {
      return Err(DatetimeError::TooFewArgumentsError);
    }

    Ok(Datetime {
      date: splited[0].as_str().try_into()?,
      time: splited[1].as_str().try_into()?
    })
  }
}

impl Into<String> for Datetime {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Display for Datetime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}", self.date, self.time)
  } 
}

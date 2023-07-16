use datatypes::datetime::{DatetimeError, Datetime};

mod datatypes;

fn main() -> Result<(), DatetimeError> {
  let datetime: Datetime =
    String::from("2023-1-1.10:50").try_into()?;

  println!("{}", datetime);

  Ok(())
}

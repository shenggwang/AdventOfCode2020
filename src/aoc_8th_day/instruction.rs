use crate::tools::file_handler::split_with_expression;

pub struct Instruction {
  pub command: String,
  pub number: isize,
}

impl From<String> for Instruction {
  fn from(s: String) -> Self {
    let v = split_with_expression(&s, " ").expect("Unable to split");
    let command: String = v[0].clone();
    let number: isize = v[1].parse().unwrap();
    Instruction {
      command,
      number
    }
  }
}
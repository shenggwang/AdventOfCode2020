use std::{
  path::Path
};

use crate::aoc_4th_day::passport::Passport;

pub fn compute1() -> usize {
  let path = Path::new("data/4rd_day/input.txt");
  return 2020;
}

pub fn compute2() -> usize {
  let path = Path::new("data/4rd_day/input.txt");
  return 2020;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_passport_valid() {
  
    let path = Path::new("data/4rd_day/test_input.txt");
    let mut pp:Passport = Passport::new();
    pp.byr(String::from("value"));
    pp.print_value();
    assert_eq!(0, 0);
  }
}

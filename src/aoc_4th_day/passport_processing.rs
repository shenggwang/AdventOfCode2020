use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::{
  get_buffer_file,
  split_with_expression
};

use crate::aoc_4th_day::passport::Passport;

pub fn compute1() -> usize {
  let path = "data/4th_day/input.txt";
  let value = count_valid_passport(path);
  return value;
}

pub fn compute2() -> usize {
  let path = "data/4th_day/input.txt";
  let value = count_valid_passport_strict(path);
  return value;
}

/// Returning usize because we don't know what is the total size
fn count_valid_passport(path: &str) -> usize {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut count = 0;
  let mut pp: Passport = Passport::new();
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.");
    if text.is_empty() {
      pp = Passport::new();
      continue;
    }
    let fields = split_with_expression(&text, " ").expect("Unable to split the String");
    pp = handle_fields(&fields, pp);
    if pp.is_valid() {
      count += 1;
      pp = Passport::new();
    }
  }
  return count;
}

/// Returning usize because we don't know what is the total size
fn count_valid_passport_strict(path: &str) -> usize {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut count = 0;
  let mut pp: Passport = Passport::new();
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.");
    if text.is_empty() {
      pp = Passport::new();
      continue;
    }
    let fields = split_with_expression(&text, " ").expect("Unable to split the String");
    pp = handle_fields(&fields, pp);
    if pp.is_valid_strict() {
      count += 1;
      pp = Passport::new();
    }
  }
  return count;
}

fn handle_fields(fields: &Vec<String>, mut pp: Passport) -> Passport {
  for index in 0..fields.len() {
    let field = split_with_expression(&fields[index], ":").expect("Unable to split the String");
    let key = field[0].clone();
    let value = field[1].clone();
    match key.as_ref() {
      "byr" => pp.birth_year(value),
      "iyr" => pp.issue_year(value),
      "eyr" => pp.expiration_year(value),
      "hgt" => pp.height(value),
      "hcl" => pp.hair_color(value),
      "ecl" => pp.eye_color(value),
      "pid" => pp.passport_id(value),
      "cid" => pp.country_id(value),
      _ => println!("Key not found on match"),
    }
  }
  return pp;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_passport_is_valid() {
    
    let mut pp1:Passport = Passport::new();
    pp1.birth_year(String::from("1937"));
    assert_eq!(false, pp1.is_valid());

    let mut pp2:Passport = Passport::new();
    pp2.birth_year(String::from("1937"));
    pp2.issue_year(String::from("2017"));
    pp2.expiration_year(String::from("2020"));
    pp2.height(String::from("183cm"));
    pp2.hair_color(String::from("value"));
    pp2.eye_color(String::from("#fffffd"));
    pp2.passport_id(String::from("860033327"));
    assert_eq!(true, pp2.is_valid());

    let mut pp3:Passport = Passport::new();
    pp3.birth_year(String::from("1937"));
    pp3.issue_year(String::from("2017"));
    pp3.expiration_year(String::from("2018"));
    pp3.height(String::from("183cm"));
    pp3.hair_color(String::from("value"));
    pp3.eye_color(String::from("#fffffd"));
    pp3.passport_id(String::from("860033327"));
    assert_eq!(false, pp3.is_valid_strict());
  }

  #[test]
  fn test_count_valid_passport() {
    let path = "data/4th_day/test_input.txt";
    let value: usize = count_valid_passport(path);
    assert_eq!(value, 2);
  }

  #[test]
  fn test_count_valid_passport2() {
    let path = "data/4th_day/test_input2.txt";
    let value: usize = count_valid_passport(path);
    assert_eq!(value, 7);
  }
}

use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::{get_buffer_file, split_with_expression};

pub fn compute1() -> u32 {
  let path = "data/7th_day/input.txt";
  let value = count_shiny_gold_bag(path);
  return value;
}

pub fn compute2() -> u32 {
  let path = "data/7th_day/input.txt";
  let value = 2020;
  return value;
}

fn count_shiny_gold_bag(path: &str) -> u32 {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut count = 0;
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.");
    if contain_shiny_gold_bag(text.clone()) {
      count += shiny_gold_bag_number(text);
    }
  }
  return count;
}

fn contain_shiny_gold_bag(line: String) -> bool {
  let re = regex::Regex::new(r"shiny gold").unwrap();
  let captures = re.captures(line.as_str());
  captures.is_some()
}

fn shiny_gold_bag_number(line: String) -> u32 {
  let re = regex::Regex::new(r"\d+ shiny gold").unwrap();
  let captures = re.captures(line.as_str());
  if captures.is_some() {
    let value: &str = captures.unwrap().get(0).map_or("", |m| m.as_str());
    let number: u32 = split_with_expression(&value.to_string(), " ")
                        .expect("Unable to split")[0].parse().unwrap();
    return number;
  }
  1
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_list_answers() {
    let path = "data/7th_day/test_input.txt";
    let answers_number = count_shiny_gold_bag(path);
    assert_eq!(answers_number, 4);
  }
}

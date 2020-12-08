use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};

use crate::tools::file_handler::{get_buffer_file, split_with_expression};
use crate::aoc_7th_day::bag::BagRule;

pub fn compute1() -> u32 {
  let path = "data/7th_day/input.txt";
  let value = to_hashmap(path);
  return how_many_shiny_gold(&value) as u32;
}

pub fn compute2() -> u32 {
  let path = "data/7th_day/input.txt";
  let value = to_hashmap(path);
  return how_many_in_shiny_gold(&value) as u32;
}

fn to_hashmap(path: &str) -> HashMap<String, Vec<BagRule>> {
  let buffer: BufReader<File> = get_buffer_file(path);
  buffer
    .lines()
    .map(|line| {
      let text: String = line.expect("Unable to read line.");
      let mut splt = text.split(" contain ");
      let bag = splt.next().unwrap().trim_end_matches("s");
      let unparsed_rules = splt.next().unwrap().trim_end_matches(".");
      let rules: Vec<BagRule> = if unparsed_rules == "no other bags" {
        vec![]
      } else {
        unparsed_rules.split(", ").map(|s| s.into()).collect()
      };
      (String::from(bag), rules)
    })
    .collect()
}

fn how_many_shiny_gold(input: &HashMap<String, Vec<BagRule>>) -> usize {
  input
    .iter()
    .filter(|(bag, rules)| {
      if bag.as_str() == "shiny gold bag" {
        false
      } else {
        rules
          .iter()
          .any(|br| br.contains_recur("shiny gold bag", input))
      }
    })
    .count()
}

fn how_many_in_shiny_gold(input: &HashMap<String, Vec<BagRule>>) -> usize {
  let rules = input.get("shiny gold bag").unwrap();
  return rules
    .iter()
    .map(|br| br.bag_count(input, br.num))
    .sum::<usize>();
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

  #[test]
  fn test_how_many_shiny_gold() {
    let path = "data/7th_day/test_input.txt";
    let value = to_hashmap(path);
    assert_eq!(how_many_shiny_gold(&value), 4);
  }

  #[test]
  fn test_how_many_in_shiny_gold() {
    let path = "data/7th_day/test_input.txt";
    let value = to_hashmap(path);
    assert_eq!(how_many_in_shiny_gold(&value), 32);
  }
}

///
/// Below only count the shiny gold bag and direct related.
///
#[allow(dead_code)]
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

#[allow(dead_code)]
fn contain_shiny_gold_bag(line: String) -> bool {
  let re = regex::Regex::new(r"shiny gold").unwrap();
  let captures = re.captures(line.as_str());
  captures.is_some()
}

#[allow(dead_code)]
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

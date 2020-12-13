use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/10th_day/input.txt";
  let list = get_list_sorted(path);
  let diff_tuple = count_diffs(list);
  return diff_tuple.0 * diff_tuple.1;
}

pub fn compute2() -> u64 {
  let path = "data/10th_day/input.txt";
  let list = get_list_sorted(path);
  let value = count_distinct_arrangements(&list);
  return value as u64;
}

fn get_list_sorted(path: &str) -> Vec<u32> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut list = vec![];
  list.push(0);
  for line in buffer.lines() {
    let number: u32 = line.expect("Unable to read line.").parse().unwrap();
    list.push(number);
  }
  list.sort();
  let last_number = list[list.len() - 1];
  list.push(last_number + 3);
  return list;
}

fn count_diffs(list: Vec<u32>) -> (u32, u32) {
  let mut diff_one = 0;
  let mut diff_three = 0;
  for index in 0..list.len() - 1 {
    let diff = list[index + 1] - list[index];
    if diff == 1 {
      diff_one += 1;
    } else if diff == 3 {
      diff_three += 1;
    }
  }
  (diff_one, diff_three)
}

fn count_distinct_arrangements(list: &Vec<u32>) -> usize {
  let mut distinct_arrangements = HashMap::new();
  distinct_arrangements.insert(0, 1);
  distinct_arrangements.insert(1, 1);
  for &v in &list[2..] {
    let mut val = distinct_arrangements.get(&(v - 1)).unwrap_or(&0) + distinct_arrangements.get(&(v - 2)).unwrap_or(&0);
    if v > 2 {
      val += distinct_arrangements.get(&(v - 3)).unwrap_or(&0);
    }
    distinct_arrangements.insert(v, val);
  }
  *distinct_arrangements.get(list.last().unwrap()).unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_diffs_multiplied() {
    let path = "data/10th_day/test_input.txt";
    let list = get_list_sorted(path);
    let diff_tuple = count_diffs(list);
    
    assert_eq!(diff_tuple.0 * diff_tuple.1, 220);
  }

  #[test]
  fn test_get_list_same_answers() {
    let path = "data/10th_day/test_input.txt";
    let list = get_list_sorted(path);
    let value = count_distinct_arrangements(&list);
    assert_eq!(value, 19208);
  }
}

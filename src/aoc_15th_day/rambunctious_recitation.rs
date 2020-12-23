use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u64 {
  let path = "data/15th_day/input.txt";
  let input = get_input(path);
  //println!("shuttle {:?}", shuttle);
  let result = number_at_give_value(input, 2020);
  return result as u64;
}

pub fn compute2() -> u64 {
  let path = "data/15th_day/input.txt";
  let input = get_input(path);
  let result = number_at_give_value(input, 30000000);
  return result as u64;
}

fn get_input(path: &str) -> String {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut text: String = "".to_string();
  for line in buffer.lines() {
    text = line.expect("Unable to read line.").parse().unwrap();

  }
  return text;
}

fn number_at_give_value(input: String, index: usize) -> usize {
  let value: Vec<usize> = input
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();
  let mut map = HashMap::new();
  let len = value.len();
  for i in 0..len {
    map.entry(value[i]).or_insert(vec![]).push(i+1);
  }
  return recursive(&mut map, len + 1, value[len - 1], index);
}

fn recursive(map: &mut HashMap<usize, Vec<usize>>, round_number: usize, previous: usize, index: usize) -> usize {
  //println!("round: {}", round_number);
  let previous_list = map.entry(previous).or_insert(vec![]);
  if previous_list.len() == 1 {
    if round_number == index {
      return 0;
    }
    map.entry(0).or_insert(vec![]).push(round_number);
    return recursive(map, round_number + 1, 0, index);
  }
  //let len = previous_list.len();
  let last: usize = previous_list[1];
  let second_to_last: usize = previous_list[0];
  let value = last - second_to_last;
  if round_number == index {
    return value;
  }
  *map.entry(value).or_insert(vec![]) = vec![last, round_number];
  return recursive(map, round_number + 1, value, index);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let test_input1 = "0,3,6".to_string();
    assert_eq!(number_at_give_value(test_input1, 2020), 436);
    let test_input2 = "1,3,2".to_string();
    assert_eq!(number_at_give_value(test_input2, 2020), 1);
    let test_input3 = "2,1,3".to_string();
    assert_eq!(number_at_give_value(test_input3, 2020), 10);
    let test_input4 = "1,2,3".to_string();
    assert_eq!(number_at_give_value(test_input4, 2020), 27);
    let test_input5 = "2,3,1".to_string();
    assert_eq!(number_at_give_value(test_input5, 2020), 78);
    let test_input6 = "3,2,1".to_string();
    assert_eq!(number_at_give_value(test_input6, 2020), 438);
    let test_input7 = "3,1,2".to_string();
    assert_eq!(number_at_give_value(test_input7, 2020), 1836);
  }

  #[test]
  fn test_part2() {
    let test_input1 = "0,3,6".to_string();
    assert_eq!(number_at_give_value(test_input1, 30000000), 175594);
    let test_input2 = "1,3,2".to_string();
    assert_eq!(number_at_give_value(test_input2, 30000000), 2578);
    let test_input3 = "2,1,3".to_string();
    assert_eq!(number_at_give_value(test_input3, 30000000), 3544142);
    let test_input4 = "1,2,3".to_string();
    assert_eq!(number_at_give_value(test_input4, 30000000), 261214);
    let test_input5 = "2,3,1".to_string();
    assert_eq!(number_at_give_value(test_input5, 30000000), 6895259);
    let test_input6 = "3,2,1".to_string();
    assert_eq!(number_at_give_value(test_input6, 30000000), 18);
    let test_input7 = "3,1,2".to_string();
    assert_eq!(number_at_give_value(test_input7, 30000000), 362);
  }
}

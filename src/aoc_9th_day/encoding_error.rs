use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::VecDeque
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/9th_day/input.txt";
  let answers_number = get_number(path, 25).0;
  return answers_number;
}

pub fn compute2() -> u32 {
  let path = "data/9th_day/input.txt";
  let list = get_weakness_number_sum_elements(path, 25);
  let answers_number = sum_of_lowest_and_highest_numbers(list);
  return answers_number;
}

fn is_summable_by_values_on_queue(number: i64, queue: VecDeque<i64>) -> bool {
  for i in 0..queue.len() {
    let queue_number = queue.get(i).unwrap();
    let missing_number = number - queue_number;
    if queue_number != &missing_number && queue.contains(&missing_number) {
      return true;
    }
  }
  return false;
}

fn is_summable(number: i64, queue: VecDeque<i64>) -> bool {
  return is_summable_by_values_on_queue(number, queue);
}

fn get_number(path: &str, queue_size: usize) -> (u32, Vec<i64>) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut list = Vec::new();
  let mut preamble: VecDeque<i64> = VecDeque::with_capacity(queue_size);
  for line in buffer.lines() {
    let number: i64 = line.expect("Unable to read line.").parse().unwrap();
    if preamble.len() == queue_size {
      if !is_summable(number, preamble.clone()) {
        return (number as u32, list);
      }
      preamble.pop_front();
    }
    list.push(number);
    preamble.push_back(number);
  }
  (1, list)
}

fn get_weakness_number_sum_elements(path: &str, queue_size: usize) -> Vec<i64> {
  let tuple = get_number(path, queue_size);
  let weakness_number = tuple.0;
  let mut list = tuple.1;
  loop {
    if list.len() == 0 {
      return vec![0];
    }
    let mut total = 0;
    for index in 0..list.len() {
      total += list[index];
      if total > weakness_number as i64 {
        list.remove(0);
        break;
      } else if total == weakness_number as i64 {
        return list[0..index].to_vec();
      }
    }
  }
}

fn sum_of_lowest_and_highest_numbers(list: Vec<i64>) -> u32 {
  let min_value = list.iter().min();
  let max_value = list.iter().max();
  return (min_value.unwrap() + max_value.unwrap()) as u32;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_number() {
    let path = "data/9th_day/test_input.txt";
    let answers_number = get_number(path, 5).0;
    assert_eq!(answers_number, 127);
  }

  #[test]
  fn test_get_weakness_number_sum_elements() {
    let path = "data/9th_day/test_input.txt";
    let list = get_weakness_number_sum_elements(path, 5);
    let answers_number = sum_of_lowest_and_highest_numbers(list);
    assert_eq!(answers_number, 62);
  }
}

use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashSet
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/6th_day/input.txt";
  let list = get_list_answers(path);
  return get_answers(list);
}

pub fn compute2() -> u32 {
  let path = "data/6th_day/input.txt";
  let list = get_list_same_answers(path);
  return get_answers(list);
}

fn get_answers(list: Vec<u32>) -> u32 {
  let mut answers = 0;
  for i in 0..list.len() {
    answers += list[i];
  }
  return answers;
}

fn get_list_answers(path: &str) -> Vec<u32> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut answers = vec![];
  let mut set = HashSet::<char>::new();
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.");
    if text.is_empty() {
      answers.push(set.len() as u32);
      set = HashSet::new();
      continue;
    }
    for field in text.chars() {
      set.insert(field);
    }
  }
  // BufReader does not allow to read the next line, either we can use another approach 
  // or add last line manually at the end
  answers.push(set.len() as u32);
  return answers;
}

fn get_list_same_answers(path: &str) -> Vec<u32> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut answers: Vec<u32> = vec![];
  let mut answers_group: Vec<HashSet<char>> = vec![];
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.");
    if text.is_empty() {
      answers.push(handle_list_answers(answers_group));
      answers_group = vec![];
      continue;
    }
    answers_group.push(text.chars().collect::<HashSet<char>>());
  }
  // BufReader does not allow to read the next line, either we can use another approach 
  // or add last line manually at the end
  answers.push(handle_list_answers(answers_group));
  return answers;
}

// this is done by attempts, need to review this part
fn handle_list_answers(list: Vec<HashSet<char>>) -> u32 {
  let mut answers: HashSet<char> = list[0].clone();
  for i in 0..list.len() {
    let next_answer: HashSet<char> = list[i].clone();
    let intersection = answers.intersection(&next_answer);
    answers = intersection.cloned().collect();
  }
  return answers.len() as u32;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_list_answers() {
    let path = "data/6th_day/test_input.txt";
    let list = get_list_answers(path);
    let answers_number = get_answers(list);

    assert_eq!(answers_number, 11);
  }

  #[test]
  fn test_get_list_same_answers() {
    let path = "data/6th_day/test_input.txt";
    let list = get_list_same_answers(path);
    let answers_number = get_answers(list);

    assert_eq!(answers_number, 6);
  }
}

use std::{
  io::{BufReader, BufRead},
  time::Instant,
  fs::File,
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_19th_day::messages::Messages;

pub fn compute1() -> u64 {
  
  let path = "data/19th_day/input.txt";
  let mut monster_messages = get_monster_messages(path);
  let now = Instant::now();
  let deciphered_messages = monster_messages.get_first_deciphered_message();
  let new_now = Instant::now();
  println!("the duration of first part:{:?}", new_now.duration_since(now));
  let intersepted_number = monster_messages.get_intersepted_number(deciphered_messages);
  return intersepted_number as u64;
}

pub fn compute2() -> u64 {

  let path = "data/19th_day/input.txt";
  let mut monster_messages = get_monster_messages(path);
  let max = monster_messages.received_messages.iter().fold(0, |acc, x| if acc > x.len() {acc} else {x.len()});

  let mut valid_messages = monster_messages.clone().valid_messages;
  *valid_messages.get_mut(&8).unwrap() = "42 | 42 8".to_string();
  *valid_messages.get_mut(&11).unwrap() = "42 31 | 42 11 31".to_string();
  let now = Instant::now();
  let deciphered_messages = monster_messages.get_first_deciphered_message_with_max(max);
  let new_now = Instant::now();
  println!("the duration of second part: {:?}", new_now.duration_since(now));
  let intersepted_number = monster_messages.get_intersepted_number(deciphered_messages);
  return intersepted_number as u64;
}

fn get_monster_messages(path: &str) -> Messages {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut messages = Messages::new();
  let mut breakline = false;
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    if text.as_str() == "" {
      breakline = true;
      continue;
    }
    if !breakline {
      messages.append_valid_message(text);
    } else {
      messages.append_message(text);
    }
  }
  return messages;
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {

    let mut first_messages = Messages::new();
    first_messages.append_valid_message("0: 1 2".to_string());
    first_messages.append_valid_message("1: \"a\"".to_string());
    first_messages.append_valid_message("2: 1 3 | 3 1".to_string());
    first_messages.append_valid_message("3: \"b\"".to_string());
    println!("first messages: {:?}", first_messages);
    let deciphered_messages = first_messages.get_first_deciphered_message();
    println!("deciphered list: {:?}", deciphered_messages);
    let mut list = deciphered_messages.iter();
    assert_eq!(list.next(), Some(&"a a b".to_string()));
    assert_eq!(list.next(), Some(&"a b a".to_string()));
  }

  #[test]
  fn test_part1_complex() {

    let path = "data/19th_day/test_input.txt";
    let mut messages = get_monster_messages(path);

    let now = Instant::now();
    let deciphered_messages = messages.get_first_deciphered_message();
    let new_now = Instant::now();
    println!("The duratin of first test first part: {:?}", new_now.duration_since(now));
    let intersepted_number = messages.get_intersepted_number(deciphered_messages);
    assert_eq!(intersepted_number, 2);
  }

  #[test]
  fn test_part1_more_complex() {

    let path2 = "data/19th_day/test2_input.txt";
    let mut messages2 = get_monster_messages(path2);

    let now2 = Instant::now();
    let deciphered_messages2 = messages2.get_first_deciphered_message();
    let new_now2 = Instant::now();
    println!("The duratin of first test second part: {:?}", new_now2.duration_since(now2));
    let intersepted_number2 = messages2.get_intersepted_number(deciphered_messages2);
    assert_eq!(intersepted_number2, 3);
  }

  #[test]
  fn test_part2() {

    let mut first_messages = Messages::new();
    first_messages.append_valid_message("0: 1 2".to_string());
    first_messages.append_valid_message("1: \"a\"".to_string());
    first_messages.append_valid_message("2: 1 3 | 3 2 1".to_string());
    first_messages.append_valid_message("3: \"b\"".to_string());
    let deciphered_messages = first_messages.get_first_deciphered_message_with_max(3);
    let mut list = deciphered_messages.iter();
    assert_eq!(list.next(), Some(&"a a b".to_string()));
  }

  #[test]
  fn test_part2_complex() {

    let path = "data/19th_day/test_input.txt";
    let mut messages = get_monster_messages(path);
    let max = messages.received_messages.iter().fold(0, |acc, x| if acc > x.len() {acc} else {x.len()});
    assert_eq!(max, 7);

    let now = Instant::now();
    let deciphered_messages = messages.get_first_deciphered_message_with_max(max);
    let new_now = Instant::now();
    println!("The duratin of first test first part: {:?}", new_now.duration_since(now));
    let intersepted_number = messages.get_intersepted_number(deciphered_messages);
    assert_eq!(intersepted_number, 2);
  }

  #[test]
  #[ignore]
  fn test_part2_more_complex() {

    let path2 = "data/19th_day/test2_input.txt";
    let mut messages2 = get_monster_messages(path2);
    let max2 = messages2.received_messages.iter().fold(0, |acc, x| if acc > x.len() {acc} else {x.len()});
    assert_eq!(max2, 45);

    let now2 = Instant::now();
    let deciphered_messages2 = messages2.get_first_deciphered_message_with_max(max2);
    let new_now2 = Instant::now();
    println!("The duratin of second test second part: {:?}", new_now2.duration_since(now2));
    let intersepted_number2 = messages2.get_intersepted_number(deciphered_messages2);
    assert_eq!(intersepted_number2, 12);
  }
}

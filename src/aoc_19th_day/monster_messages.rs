use std::{
  io::{BufReader, BufRead},
  fs::File
};

use threadpool::ThreadPool;
use crate::tools::file_handler::get_buffer_file;
use crate::aoc_19th_day::messages::Messages;

pub fn compute1() -> u64 {
  let path = "data/19th_day/input.txt";
  let monster_messages = get_monster_messages(path);
  let deciphered_messages = Messages::get_first_deciphered_message(monster_messages.clone().valid_messages);
  let intersepted_number = Messages::get_intersepted_number(monster_messages, deciphered_messages);
  return intersepted_number as u64;
}

pub fn compute2() -> u64 {
  let n_workers = 10;
  let pool = ThreadPool::new(n_workers);

  let path = "data/19th_day/input.txt";
  let monster_messages = get_monster_messages(path);
  let max = monster_messages.received_messages.iter().fold(0, |acc, x| if acc > x.len() {acc} else {x.len()});

  let mut valid_messages = monster_messages.clone().valid_messages;
  *valid_messages.get_mut(&8).unwrap() = "42 | 42 8".to_string();
  *valid_messages.get_mut(&11).unwrap() = "42 31 | 42 11 31".to_string();
  println!("messages: {:?}", valid_messages);
  let deciphered_messages = Messages::get_first_deciphered_message_with_max(&pool, valid_messages, max);
  println!("deciphered messages: {:?}", deciphered_messages);
  let intersepted_number = Messages::get_intersepted_number(monster_messages, deciphered_messages);
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

    // simple test
    let mut first_messages = Messages::new();
    first_messages.append_valid_message("0: 1 2".to_string());
    first_messages.append_valid_message("1: \"a\"".to_string());
    first_messages.append_valid_message("2: 1 3 | 3 1".to_string());
    first_messages.append_valid_message("3: \"b\"".to_string());
    println!("first messages: {:?}", first_messages);
    let to_decipher_message = first_messages.clone().valid_messages;
    let deciphered_messages = Messages::get_first_deciphered_message(to_decipher_message);
    println!("deciphered list: {:?}", deciphered_messages);
    assert_eq!(deciphered_messages, ["a a b", "a b a"]);
    
    // complext test
    let path = "data/19th_day/test_input.txt";
    let messages = get_monster_messages(path);
    let deciphered_messages = Messages::get_first_deciphered_message(messages.clone().valid_messages);
    let intersepted_number = Messages::get_intersepted_number(messages, deciphered_messages);
    assert_eq!(intersepted_number, 2);

    // more complext test that received for the second part
    let path2 = "data/19th_day/test2_input.txt";
    let messages2 = get_monster_messages(path2);
    let deciphered_messages2 = Messages::get_first_deciphered_message(messages2.clone().valid_messages);
    let intersepted_number2 = Messages::get_intersepted_number(messages2, deciphered_messages2);
    assert_eq!(intersepted_number2, 3);
  }

  #[test]
  fn test_part2() {
    let n_workers = 10;
    let pool = ThreadPool::new(n_workers);

    // simple test
    let mut first_messages = Messages::new();
    first_messages.append_valid_message("0: 1 2".to_string());
    first_messages.append_valid_message("1: \"a\"".to_string());
    first_messages.append_valid_message("2: 1 3 | 3 2 1".to_string());
    first_messages.append_valid_message("3: \"b\"".to_string());
    let to_decipher_message = first_messages.clone().valid_messages;
    let max = 3;
    let deciphered_messages = Messages::get_first_deciphered_message_with_max(&pool, to_decipher_message, max);
    assert_eq!(deciphered_messages, ["a a b", "a b 1 3 a", "a b 3 2 1 a"]);

    // complex test    
    let path2 = "data/19th_day/test2_input.txt";
    let messages2 = get_monster_messages(path2);
    let max2 = messages2.received_messages.iter().fold(0, |acc, x| if acc > x.len() {acc} else {x.len()});
    assert_eq!(max2, 45);

    let mut valid_messages = messages2.clone().valid_messages;
    *valid_messages.get_mut(&8).unwrap() = "42 | 42 8".to_string();
    *valid_messages.get_mut(&11).unwrap() = "42 31 | 42 11 31".to_string();

    println!("messages: {:?}", valid_messages);
    assert_eq!(1, 2);
    
    let deciphered_messages2 = Messages::get_first_deciphered_message_with_max(&pool, valid_messages, max2);
    let intersepted_number2 = Messages::get_intersepted_number(messages2, deciphered_messages2);
    assert_eq!(intersepted_number2, 12);
  }
}

use std::{
  io::{BufReader, BufRead},
  fs::File
};

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
  let path = "data/19th_day/input.txt";
  let monster_messages = get_monster_messages(path);
  let result = 2020;
  return result as u64;
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
  }

  #[test]
  fn test_part2() {
    let path = "data/19th_day/test_input.txt";
    
  }
}

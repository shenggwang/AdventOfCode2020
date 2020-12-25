use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_16th_day::ticket::{Ticket, Input};

pub fn compute1() -> u64 {
  let path = "data/16th_day/input.txt";
  let mut ticket = get_ticket(path);
  let result = ticket.get_error_rate();
  return result as u64;
}

pub fn compute2() -> u64 {
  let path = "data/16th_day/input.txt";
  let mut ticket = get_ticket(path);
  let map = ticket.get_rules_with_name();
  let vec = Ticket::get_index_of(&map, "departure");
  //println!("map: {:?}", map);
  println!("vec: {:?}", vec);
  //println!("value: {:?}", ticket.get_value(vec));
  return 2020;
}

fn get_ticket(path: &str) -> Ticket {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut ticket = Ticket::new();
  let mut input_type = Input::Rule;
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    if text.as_str() == "" {
      continue;
    } else if text.as_str() == "your ticket:" {
      input_type = Input::Ticket;
      continue;
    } else if text.as_str() == "nearby tickets:" {
      input_type = Input::NearbyTicket;
      continue;
    }
    match input_type {
      Input::Rule => ticket.append_rules(text),
      Input::Ticket => ticket.append_ticket(text),
      Input::NearbyTicket => ticket.append_nearby_tickets(text),
    }
  }
  return ticket;
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/16th_day/test_input.txt";
    let mut ticket = get_ticket(path);
    assert_eq!(ticket.get_error_rate(), 71);
  }

  #[test]
  fn test_part2() {
    let path = "data/16th_day/test2_input.txt";
    let mut ticket = get_ticket(path);
    let map = ticket.get_rules_with_name();
    let vec_class = Ticket::get_index_of(&map, "class");
    let vec_row = Ticket::get_index_of(&map, "row");
    let vec_seat = Ticket::get_index_of(&map, "seat");
    assert_eq!(1, vec_class[0]);
    assert_eq!(0, vec_row[0]);
    assert_eq!(2, vec_seat[0]);
  }
}

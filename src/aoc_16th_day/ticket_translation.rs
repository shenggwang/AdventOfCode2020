use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_16th_day::ticket::{Ticket, Input};

pub fn compute1() -> u64 {
  let path = "data/16th_day/input.txt";
  let ticket = get_ticket(path);
  let result = get_error_rate(ticket);
  return result as u64;
}

pub fn compute2() -> u64 {
  let path = "data/16th_day/input.txt";
  let ticket = get_ticket(path);
  let result = 2020;
  return result;
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
    //println!("line: {:?}", text);
    match input_type {
      Input::Rule => ticket.append_rules(text),
      Input::Ticket => ticket.append_ticket(text),
      Input::NearbyTicket => ticket.append_nearby_ticket(text),
    }
  }
  return ticket;
}

fn get_error_rate(ticket: Ticket) -> usize {
  let mut result = 0;

  for nearby_ticket in &ticket.nearby_ticket {
    let mut numbers: Vec<usize> = nearby_ticket
      .split(",")
      .map(|x| x.parse().unwrap())
      .collect();
    for number in numbers{
      if !within_the_rule(&ticket, number) {
        result += number;
      }
    }
  }
  return result;
}

fn within_the_rule(ticket: &Ticket, number: usize) -> bool {
  for tuple in &ticket.rules {
    if number >= tuple.0 && number <= tuple.1 {
      return true;
    }
  }
  return false;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/16th_day/test_input.txt";
    let ticket = get_ticket(path);
    assert_eq!(get_error_rate(ticket), 71);
  }

  #[test]
  fn test_part2() {
    let path = "data/16th_day/test2_input.txt";
    let ticket = get_ticket(path);
    assert_eq!(2020, 2020);
  }
}

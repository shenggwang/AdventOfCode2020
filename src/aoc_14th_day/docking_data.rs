use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};
use regex::Regex;

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_14th_day::program::Program;

pub fn compute1() -> usize {
  let path = "data/14th_day/input.txt";
  let program = get_program(path);
  let result = handle_program(program);
  return result;
}

pub fn compute2() -> u64 {
  let path = "data/14th_day/input.txt";
  let program = get_program(path);
  let result = 2020;
  return result;
}

fn get_program(path: &str) -> Vec<Program> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut content = vec![];
  let mut program = Program::new();
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    let tuple: Vec<&str> = text.split(" = ").collect();
    //println!("tuple {:?}", tuple);
    if tuple[0] == "mask" {
      if Program::new() != program {
        content.push(program);
      }
      program = Program::from(tuple[1]);
    } else {
      // get address
      let re = Regex::new(r"\d+").unwrap();
      let captures = re.captures(tuple[0]);
      let address: usize = captures.unwrap().get(0).map_or(0, |m| m.as_str().parse().unwrap());
      program.add(address, tuple[1].parse().unwrap());
    }
  }
  content.push(program);
  return content;
}

fn handle_program(programs: Vec<Program>) -> usize {
  let mut map = HashMap::new();
  //println!("programs {:?}", programs);
  for program in programs {
    for (address, decimal) in program.clone().memory {
      map.insert(address, get_result(decimal, program.clone().mask));
    }
  }
  //println!("map: {:?}", map);
  let mut value = 0;
  for (_address, decimal) in map {
    value += decimal;
  }
  return value;
}

fn get_result(value: usize, mask: Vec<char>) -> usize {
  let binary = format!("{:036b}", value);
  //println!("binary before: {:?}", result);
  let result = binary.chars()
    .enumerate()
    .map(|(i, x)| -> char {
      //println!("value: {:?}, mask: {:?} at {:?}", result, mask[i], i);
      if mask[i] != 'X' && mask[i] != x {
        return mask[i];
      }
      return x;
    })
    .collect::<String>();
  
  //println!("binary after: {:?}", result);
  let result_int = usize::from_str_radix(result.as_str(), 2).unwrap();
  //println!("result: {}", result_int);
  return result_int;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/14th_day/test_input.txt";
    let program = get_program(path);
    let result = handle_program(program);
    assert_eq!(result, 165);
  }

  #[test]
  fn test_part2() {
    let path = "data/14th_day/test_input.txt";
    let program = get_program(path);
    assert_eq!(5, 1068781);
  }
}

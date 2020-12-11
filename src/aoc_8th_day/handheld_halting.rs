use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashSet
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_8th_day::instruction::Instruction;

pub fn compute1() -> u32 {
  let path = "data/8th_day/input.txt";
  let list = get_execution_list(path);
  let accumulator = get_accumulator(list);
  return accumulator as u32;
}

pub fn compute2() -> u32 {
  let path = "data/8th_day/input.txt";
  let list = get_execution_list(path);
  let value = 2020;
  return value;
}

fn get_execution_list(path: &str) -> Vec<Instruction> {
  let buffer: BufReader<File> = get_buffer_file(path);
  buffer
    .lines()
    .map(|line| {
      let text: String = line.expect("Unable to read line.");
      let instruction = Instruction::from(text);
      //println!("{}:{}", instruction.command, instruction.number);
      instruction
    })
    .collect()
}

fn get_accumulator(list: Vec<Instruction>) -> usize {
  let mut set = HashSet::<usize>::new();
  let mut accumulator = 0;
  let mut index = 0;
  loop {
    let instruction = &list[index];
    match instruction.command.as_ref() {
      "acc" => {
        if set.contains(&index) {
          //println!("stop at index: {}", index);
          break;
        }
        set.insert(index);
        accumulator += instruction.number;
        index += 1;
      },
      "jmp" => {
        if set.contains(&index) {
          //println!("stop at index: {}", index);
          break;
        }
        set.insert(index);
        if instruction.number < 0 {
          //println!("sub before: {}: {}", index, instruction.number);
          index -= instruction.number.wrapping_abs() as usize;
          //println!("sub after: {}: {}", index, instruction.number);
        } else {
          //println!("add before: {}: {}", index, instruction.number);
          index += instruction.number as usize;
          //println!("add after: {}: {}", index, instruction.number);
        }
      },
      "nop" => {
        if set.contains(&index) {
          //println!("stop at index: {}", index);
          break;
        }
        set.insert(index);
        index += 1;
      },
      _ => {
        println!("No instruction found");
        break;
      },
    }
  } 
  accumulator as usize
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_list_answers() {
    let path = "data/8th_day/test_input.txt";
    let list = get_execution_list(path);
    assert_ne!(list.len(), 0);
    let accumulator = get_accumulator(list);
    assert_eq!(accumulator, 5);
  }
}

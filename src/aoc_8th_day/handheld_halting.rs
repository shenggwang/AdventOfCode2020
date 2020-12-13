use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::LinkedList,
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_8th_day::instruction::Instruction;

pub fn compute1() -> u32 {
  let path = "data/8th_day/input.txt";
  let list = get_execution_list(path);
  let accumulator = get_accumulator_stopped_at_repeated_instruction(list).1;
  return accumulator as u32;
}

pub fn compute2() -> u32 {
  let path = "data/8th_day/input.txt";
  let list = get_execution_list(path);
  let accumulator = get_accumulator_with_instruction_changed(list);
  return accumulator as u32;
}

fn get_execution_list(path: &str) -> Vec<Instruction> {
  let buffer: BufReader<File> = get_buffer_file(path);
  buffer
    .lines()
    .map(|line| {
      let text: String = line.expect("Unable to read line.");
      let instruction = Instruction::from(text);
      instruction
    })
    .collect()
}

fn get_accumulator_stopped_at_repeated_instruction(list: Vec<Instruction>) -> (LinkedList::<usize>, usize, bool) {
  let mut graph = LinkedList::<usize>::new();
  let mut accumulator = 0;
  let mut dag = true;
  let mut index = 0;
  while list.len() > index {
    let instruction = &list[index];
    match instruction.command.as_ref() {
      "acc" => {
        if graph.contains(&index) {
          dag = false;
          break;
        }
        graph.push_back(index);
        accumulator += instruction.number;
        index += 1;
      },
      "jmp" => {
        if graph.contains(&index) {
          dag = false;
          break;
        }
        graph.push_back(index);
        if instruction.number < 0 {
          index -= instruction.number.wrapping_abs() as usize;
        } else {
          index += instruction.number as usize;
        }
      },
      "nop" => {
        if graph.contains(&index) {
          dag = false;
          break;
        }
        graph.push_back(index);
        index += 1;
      },
      _ => {
        println!("No instruction found");
        dag = false;
        break;
      },
    }
  }
  (graph, accumulator as usize, dag)
}

fn swap_instruction(mut new_list: Vec<Instruction>, index: usize) -> Vec<Instruction> {
  if new_list[index].command == "jmp" {
    new_list[index].command = "nop".to_string();
  } else if new_list[index].command == "nop" {
    new_list[index].command = "jmp".to_string();
  }
  new_list
}

fn get_accumulator_with_instruction_changed(list: Vec<Instruction>) -> usize {
  let mut new_list = list.clone();
  let mut graph = get_accumulator_stopped_at_repeated_instruction(list).0;
  let mut count_value = 0;
  while count_value < 1000 {
    let index = graph.pop_back().unwrap();
    new_list = swap_instruction(new_list, index);
    let tuple = get_accumulator_stopped_at_repeated_instruction(new_list.clone());
    if tuple.2 {
      return tuple.1 as usize;
    }
    count_value += 1;
  }
  1
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_accumulator_stopped_at_repeated_instruction() {
    let path = "data/8th_day/test_input.txt";
    let list = get_execution_list(path);
    assert_ne!(list.len(), 0);
    let accumulator = get_accumulator_stopped_at_repeated_instruction(list).1;
    assert_eq!(accumulator, 5);
  }

  #[test]
  fn test_get_accumulator_with_instruction_changed() {
    let path = "data/8th_day/test_input.txt";
    let list = get_execution_list(path);
    let accumulator = get_accumulator_with_instruction_changed(list);
    assert_eq!(accumulator, 6);
  }
}

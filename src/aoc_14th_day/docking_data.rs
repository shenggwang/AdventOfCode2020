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

pub fn compute2() -> usize {
  let path = "data/14th_day/input.txt";
  let program = get_program(path);
  let result = handle_program_memory(program);
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
    for (address, decimal) in &program.memory {
      map.insert(*address, get_result(*decimal, &program.mask));
    }
  }
  //println!("map: {:?}", map);
  let mut value = 0;
  for (_address, decimal) in map {
    value += decimal;
  }
  return value;
}

fn get_result(value: usize, mask: &Vec<char>) -> usize {
  let binary = Program::convert_to_binary(value);
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
  
  return Program::convert_to_usize(result.as_str());
}

fn handle_program_memory(programs: Vec<Program>) -> usize {
  let mut map = HashMap::new();
  //println!("programs {:?}", programs);
  for program in programs {
    for (address, decimal) in &program.memory {
      let floating_values = get_floating(*address, &program.mask);
      //println!("floating: {:?}", floating_values);
      handle_floating(&mut map, floating_values, *decimal);
    }
  }
  //println!("map: {:?}", map);
  let mut value = 0;
  for (_address, decimal) in &map {
    value += decimal;
  }
  return value;
}

fn get_floating(value: usize, mask: &Vec<char>) -> String {
  let binary = format!("{:036b}", value);
  //println!("binary before: {:?}", binary);
  let result = binary.chars()
    .enumerate()
    .map(|(i, x)| -> char {
      //println!("value: {:?}, mask: {:?} at {:?}", result, mask[i], i);
      if mask[i] == 'X' || mask[i] == '1' {
        return mask[i];
      }
      return x;
    })
    .collect::<String>();
  //println!("binary with floating: {:?}", result);
  return result;
}

fn handle_floating(map: &mut HashMap<usize, usize>, floating: String, value: usize) -> () {
  let list = Program::get_all_combinations(floating.clone());
  //println!("all combination: {:?}", list);

  for element in list {
    let mut index = 0;
    let address = floating.chars()
      .map(|x| {
        if x == 'X' {
          index += 1;
          return element.chars().nth(index - 1).unwrap();
        } 
        return x;
      })
      .collect::<String>();
    let converted_address = Program::convert_to_usize(address.as_str());
    //println!("address: {}, addressed converted {:?}", address, converted_address);
    *map.entry(converted_address).or_insert(0) = value;
  }
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
    let path = "data/14th_day/test2_input.txt";
    let program = get_program(path);
    let result = handle_program_memory(program);
    assert_eq!(result, 208);
  }
}

use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashSet
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/13th_day/input.txt";
  let shuttle = get_shuttle(path);
  //println!("shuttle {:?}", shuttle);
  let result = get_earliest_shuttle(shuttle.0, shuttle.1) as u32;
  return result;
}

pub fn compute2() -> u64 {
  let path = "data/13th_day/input.txt";
  let shuttle = get_shuttle_in_string(path);
  let result = get_ordered_shuttle(shuttle.0, shuttle.1) as u64;
  return result;
}

fn get_shuttle(path: &str) -> (usize, Vec<usize>) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut content = vec![];
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    content.push(text);
  }
  //println!("Content: {:?}", content);
  let mut times = content[1]
                .split(",")
                .filter(|&x| x != "x")
                .map(|x| x.parse().unwrap())
                .collect();
  return (content[0].parse().unwrap(), times);
}

fn get_shuttle_in_string(path: &str) -> (usize, Vec<String>) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut content = vec![];
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    content.push(text);
  }
  let mut times = content[1]
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();
  return (content[0].parse().unwrap(), times);
}

fn get_earliest_shuttle(timestamp: usize, shuttles: Vec<usize>) -> usize {
  // tuple of (value, shuttle_id), where value is to calculate the minimum waiting time and shutle_id.
  let mut tuple = (timestamp, 0);
  for shuttle in shuttles {
    let value = shuttle - (timestamp % shuttle);
    if value < tuple.0 {
      tuple = (value, shuttle);
    }
  }
  //println!("shuttle {:?}, {}", tuple, timestamp);
  return (tuple.0 + timestamp - timestamp) * tuple.1;
}

// This is a brute-force solutions, I believe that there is a better way to resolve this
fn get_ordered_shuttle(timestamp: usize, shuttles: Vec<String>) -> usize {

  let mut shuttle_list = shuttles.clone();
  // We are assuming that the first shuttle is not x
  let first_shuttle_id: usize = shuttle_list.remove(0).parse().unwrap();
  let mut current_time = timestamp - timestamp % first_shuttle_id + first_shuttle_id;
  //println!("current_time: {:?}, {}, {}", current_time, timestamp, first_shuttle_id);
  
  while current_time < 10_usize.pow(12) {
    let mut shuttle_size = 0;
    for index in 0..shuttle_list.len() {
      if &shuttle_list[index] == "x" {
        shuttle_size += 1;
        continue;
      }
      let current_shuttle_id = &shuttle_list[index].parse().unwrap();
      let value: usize = current_shuttle_id - current_time % current_shuttle_id;
      if value != index + 1 {
        break;
      }
      shuttle_size += 1;
    }
    if shuttle_size == shuttle_list.len() {
      println!("shuttle size {:?}, current time {:?}, with list {:?}", shuttle_size, current_time, shuttle_list);
      return current_time;
    }
    current_time += first_shuttle_id;
  }
  //println!("shuttle {:?}, {}", tuple, timestamp);
  return 2020;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/13th_day/test_input.txt";
    let shuttle = get_shuttle(path);
    let result = get_earliest_shuttle(shuttle.0, shuttle.1);
    assert_eq!(result, 295);
  }

  #[test]
  fn test_part2() {
    let path = "data/13th_day/test_input.txt";
    let shuttle = get_shuttle_in_string(path);
    let result = get_ordered_shuttle(shuttle.0, shuttle.1);
    assert_eq!(result, 1068781);
  }
}

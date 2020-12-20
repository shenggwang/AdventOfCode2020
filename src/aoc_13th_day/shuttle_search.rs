use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashSet
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/13th_day/input.txt";
  let shuttle = get_shuttle(path);
  let result = handle_value(shuttle.0, shuttle.1) as u32;
  return result;
}

pub fn compute2() -> u32 {
  let path = "data/13th_day/input.txt";
  
  return 2020;
}

fn get_shuttle(path: &str) -> (usize, HashSet<usize>) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut content = vec![];
  let mut set = HashSet::<usize>::new();
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    content.push(text);
  }
  //println!("Content: {:?}", content);
  let mut times = content[1]
                .split(",")
                .map(|value| -> usize {if value != "x" { return value.parse().unwrap(); } return 0;})
                .collect::<HashSet<usize>>();
  times.remove(&0);
  return (content[0].parse().unwrap(), times);
}

fn handle_value(timestamp: usize, shuttles: HashSet<usize>) -> usize {
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/13th_day/test_input.txt";
    let shuttle = get_shuttle(path);
    //println!("shuttle {:?}", shuttle);
    let result = handle_value(shuttle.0, shuttle.1);
    assert_eq!(result, 295);
  }

  #[test]
  fn test_part2() {
    let path = "data/13th_day/test_input.txt";
    
    //assert_eq!(occupied_seats, 26);
  }
}

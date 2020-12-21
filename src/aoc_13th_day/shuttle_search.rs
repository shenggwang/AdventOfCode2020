use std::{
  io::{BufReader, BufRead},
  fs::File
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
  let mut shuttle = get_shuttle_in_string(path);
  let result = get_ordered_shuttle(&mut shuttle) as u64;
  return result;
}

fn get_shuttle(path: &str) -> (usize, Vec<usize>) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut content = vec![];
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    content.push(text);
  }
  let times = content[1]
                .split(",")
                .filter(|&x| x != "x")
                .map(|x| x.parse().unwrap())
                .collect();
  return (content[0].parse().unwrap(), times);
}

fn get_shuttle_in_string(path: &str) -> Vec<(usize, usize)> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut content = vec![];

  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    content.push(text);
  }
  let tuples = content[1]
                .split(",")
                .enumerate()
                .map(|(i, x)| -> (usize, usize) {
                  if x != "x" {
                    //println!("added iterate map: {:?} at {:?}", x, i);
                    let shutle_id = x.parse().unwrap();
                    return (shutle_id, i);
                  }
                  //println!("NOT added iterate map: {:?} at {:?}", x, i);
                  return (0, 0);
                })
                .filter(|x| x != &(0_usize, 0_usize))
                .collect();
  return tuples;
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

// This is a brute-force solutions, this doesn't work as solution.
// After google it, it was suggested 'the chinese remainder theorem'.
fn get_ordered_shuttle(tuples: &mut Vec<(usize, usize)>) -> usize {

  let mut timestamp = 0;
  let mut inc = tuples[0].0;
  for &(shuttle, index) in &tuples[1..] {
    // friggin CRT sieve garbage see: https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Computation
    while (timestamp + index) % shuttle != 0 {
        timestamp += inc;
    }
    // adjust for the next modulo
    inc *= shuttle;
  }
  return timestamp;
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
    let mut shuttle = get_shuttle_in_string(path);
    let result = get_ordered_shuttle(&mut shuttle) as u64;
    assert_eq!(result, 1068781);
  }
}

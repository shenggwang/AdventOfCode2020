use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_12th_day::movement::{Action, Movement};

pub fn compute1() -> u32 {
  let path = "data/12th_day/input.txt";
  let list = get_list(path);
  let distance = handle_list(list);
  return distance;
}

pub fn compute2() -> u32 {
  let path = "data/12th_day/input.txt";
  
  return 2020;
}

fn get_list(path: &str) -> Vec<Movement> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut list = vec![];
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    list.push(Movement::from(text));
  }
  return list;
}

fn handle_list(list: Vec<Movement>) -> u32 {
  let mut initial = (0, 0);
  // direction: 0 or 360 -> right, 90 -> down, 180 -> left, 270 -> top
  let mut direction = 0;
  for movement in list {
    if direction >= 360 {
      direction -= 360;
    } else if direction < 0 {
      direction += 360;
    }
    match movement.action {
      Action::North => initial.1 += movement.value,
      Action::South => initial.1 -= movement.value,
      Action::West => initial.0 -= movement.value,
      Action::East => initial.0 += movement.value,
      Action::Left => direction -= movement.value,
      Action::Right => direction += movement.value,
      Action::Forward => match direction {
        0 => initial.0 += movement.value,
        90 => initial.1 -= movement.value,
        180 => initial.0 -= movement.value,
        270 => initial.1 += movement.value,
        _ => println!("Direction failed!"),
      },
    }
    //println!("Position {:?}", initial);
  }
  return (initial.0.abs() + initial.1.abs()) as u32;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/12th_day/test_input.txt";
    let list = get_list(path);
    let distance = handle_list(list);
    assert_eq!(distance, 25);
  }

  #[test]
  fn test_part2() {
    let path = "data/12th_day/test_input.txt";
    
    //assert_eq!(occupied_seats, 26);
  }
}

use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_12th_day::movement::{Action, Movement};

pub fn compute1() -> u32 {
  let path = "data/12th_day/input.txt";
  let list = get_list(path);
  let distance = handle_list_part1(list);
  return distance;
}

pub fn compute2() -> u32 {
  let path = "data/12th_day/input.txt";
  let list = get_list(path);
  let distance = handle_list_part2(list);
  return distance;
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

fn handle_list_part1(list: Vec<Movement>) -> u32 {
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

fn handle_list_part2(list: Vec<Movement>) -> u32 {
  let mut initial = (0, 0);
  let mut waypoint = (10, 1);
  for movement in list {
    match movement.action {
      Action::North => waypoint.1 += movement.value,
      Action::South => waypoint.1 -= movement.value,
      Action::West => waypoint.0 -= movement.value,
      Action::East => waypoint.0 += movement.value,
      Action::Left => waypoint = change_movement(movement.value, waypoint, true),
      Action::Right => waypoint = change_movement(movement.value, waypoint, false),
      Action::Forward =>  initial = (initial.0 + (waypoint.0 * movement.value), initial.1 + (waypoint.1 * movement.value)),
    }
    //println!("Position {:?}", initial);
  }
  return (initial.0.abs() + initial.1.abs()) as u32;
}

fn change_movement(movement: isize, waypoint: (isize, isize), is_left: bool) -> (isize, isize) {
  if (movement == 90 && is_left) || (movement == 270 && !is_left) {
    return (- waypoint.1, waypoint.0);
  } else if movement == 180 {
    return (- waypoint.0, - waypoint.1);
  } else if (movement == 270 && is_left) || (movement == 90 && !is_left) {
    return (waypoint.1, - waypoint.0);
  }
  return waypoint;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/12th_day/test_input.txt";
    let list = get_list(path);
    let distance = handle_list_part1(list);
    assert_eq!(distance, 25);
  }

  #[test]
  fn test_part2() {
    let path = "data/12th_day/test_input.txt";
    let list = get_list(path);
    let distance = handle_list_part2(list);
    assert_eq!(distance, 286);
  }
}

use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/11th_day/input.txt";
  let (map, width, height) = get_map_and_size(path);
  let final_map = recursive_set_rounds(map, width, height);
  let occupied_seats = count_occupied_seats(final_map);
  return occupied_seats;
}

pub fn compute2() -> u32 {
  let path = "data/11th_day/input.txt";
  //let list = get_list_sorted(path);
  let value = 2020;
  return value;
}

fn get_map_and_size(path: &str) -> (HashMap<usize, char>, usize, usize) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut map = HashMap::new();
  let (mut position, mut width, mut height) = (0, 0, 0);
  let mut total_seats = 0;
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    //println!("line: {}", text);
    for seat in text.chars() {
      //println!("char: {:?} at {}", seat, position);
      if seat == 'L' {
        map.insert(position, '#');
      } else {
        map.insert(position, seat); 
      }
      position += 1;
    }
    if width == 0 {
      width = text.len()
    }
    height += 1;
  }
  //println!("total seat {}", total_seats);
  //println!("Initial map!");
  //show_map(map.clone(), width, height);
  return (map, width, height);
}

fn count_occupied_seats(map: HashMap<usize, char>) -> u32 {
  let mut occupied_seats_count: u32 = 0;
  for (position, seat) in map {
    if seat.clone() == '#' {
      occupied_seats_count += 1;
    }
  }
  return occupied_seats_count;
}

fn recursive_set_rounds(map: HashMap<usize, char>, width: usize, height: usize) -> HashMap<usize, char> {
  let mut new_map = HashMap::new();
  for (position, seat) in map.clone() {
    let mut list = vec![];
    // 4 CORNERS and not a seat
    if position.clone() == 0 
        || position.clone() == width - 1 
        || position.clone() / height == width - 1
        || position.clone() == height * width - 1 
        || map.get(&position).unwrap().clone() == '.' {
      new_map.insert(position, seat);
      continue;
    }
    // TOP
    else if position.clone() < width {
      list.push(position.clone() - 1);
      list.push(position.clone() + 1);
      list.push(position.clone() + width);
      list.push(position.clone() + width - 1);
      list.push(position.clone() + width + 1);
    } 
    // BOTTOM
    else if position.clone() >= width * height - width {
      list.push(position.clone() - 1);
      list.push(position.clone() + 1);
      list.push(position.clone() - width);
      list.push(position.clone() - width - 1);
      list.push(position.clone() - width + 1);
    }
    // LEFT
    else if position.clone() % width == 0 {
      list.push(position.clone() + 1);
      list.push(position.clone() + width);
      list.push(position.clone() + width + 1);
      list.push(position.clone() - width);
      list.push(position.clone() - width + 1);
    } 
    // RIGHT
    else if position.clone() % width == width - 1 {
      list.push(position.clone() - 1);
      list.push(position.clone() + width);
      list.push(position.clone() + width - 1);
      list.push(position.clone() - width);
      list.push(position.clone() - width - 1);
    }
    // any other position
    else {
      list.push(position.clone() + 1);
      list.push(position.clone() - 1);
      list.push(position.clone() + width);
      list.push(position.clone() + width - 1);
      list.push(position.clone() + width + 1);
      list.push(position.clone() - width);
      list.push(position.clone() - width - 1);
      list.push(position.clone() - width + 1);
    }
    new_map.insert(position, handle_value(map.clone(), list, position.clone()));
  }

  //println!("Map yeah!");
  //show_map(new_map.clone(), width, height);
  
  if new_map.clone() == map.clone() {
    return new_map;
  }
  return recursive_set_rounds(new_map, width, height);
}

fn handle_value(map: HashMap<usize, char>, list: Vec<usize>, position: usize) -> char {
  match map.get(&position).unwrap() {
    '#' => is_more_than_four_occupied(map, list),
    'L' => is_no_adjacent_seat_occupied(map, list),
    _ => '.',
  }
}

fn is_more_than_four_occupied(map: HashMap<usize, char>, list: Vec<usize>) -> char {
  let mut count = 0;
  for position in list {
    if map.get(&position).unwrap().clone() == '#' {
      count += 1;
    }
  }
  if count > 0 {
    return 'L';
  }
  return '#';
}

fn is_no_adjacent_seat_occupied(map: HashMap<usize, char>, list: Vec<usize>) -> char {
  for position in list {
    if map.get(&position).unwrap().clone() == '#' {
      return 'L';
    }
  }
  return '#';
}

fn show_map(map: HashMap<usize, char>, width: usize, height: usize) {
  for i in 0..height {
    let mut value: String = String::from("");
    for j in 0..width {
      value.push(map.get(&((i * width) + j)).unwrap().clone());
    }
    println!("{:?}", value);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_diffs_multiplied() {
    let path = "data/11th_day/test_input.txt";
    let (map, width, height) = get_map_and_size(path);
    let final_map = recursive_set_rounds(map, width, height);
    let occupied_seats = count_occupied_seats(final_map);
    assert_eq!(occupied_seats, 37);
  }

  #[test]
  fn test_get_list_same_answers() {
    let path = "data/11th_day/test_input.txt";
    //let list = get_list_sorted(path);

    assert_eq!(2, 2);
  }
}

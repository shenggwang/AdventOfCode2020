use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};

use crate::tools::file_handler::get_buffer_file;
use crate::aoc_11th_day::enums::Movement;

pub fn compute1() -> u32 {
  let path = "data/11th_day/input.txt";
  let map = &mut HashMap::new();
  let (width, height) = get_map_and_size(map, path);
  // Avoid stack overflow by limiting to 200 iterations, feel free to increment this
  let final_map = recursive_round_handle(&mut 200, map, width, height, 4);
  let occupied_seats = count_occupied_seats(final_map);
  return occupied_seats;
}

pub fn compute2() -> u32 {
  let path = "data/11th_day/input.txt";
  let map = &mut HashMap::new();
  let (width, height) = get_map_and_size(map, path);
  // Avoid stack overflow by limiting to 200 iterations, feel free to increment this
  let final_map = recursive_round_handle(&mut 200, map, width, height, 5);
  let occupied_seats = count_occupied_seats(final_map);
  return occupied_seats;
}

fn get_map_and_size(map: &mut HashMap<usize, char>, path: &str) -> (usize, usize) {
  let buffer: BufReader<File> = get_buffer_file(path);
  let (mut position, mut width, mut height) = (0, 0, 0);
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
  //println!("total seat {}, width {}, height {}", position, width, height);
  //println!("Initial map!");
  //show_map(map.clone(), width, height);
  return (width, height);
}

fn count_occupied_seats(map: &HashMap<usize, char>) -> u32 {
  let mut occupied_seats_count: u32 = 0;
  for (position, seat) in map {
    if *seat == '#' {
      occupied_seats_count += 1;
    }
  }
  return occupied_seats_count;
}

fn recursive_round_handle<'a>(count: &mut usize, map: &'a mut HashMap<usize, char>, width: usize, height: usize, number_seat: usize) -> &'a HashMap<usize, char> {
  let previous_map: &HashMap<usize, char> = &map.clone();
  for (position, seat) in previous_map.clone() {
    // 4 CORNERS and not a seat
    if position == 0
        || position == width - 1
        || position == width * (height - 1)
        || position == width * height - 1
        || seat == '.' {
      *map.entry(position).or_insert(seat) = seat;
      continue;
    }

    *map.entry(position).or_insert(seat) = handle_seat(previous_map, position, width, height, number_seat);
  }

  println!("------------------//-----------------");
  show_map(map.clone(), width, height);
  
  *count -= 1;
  if count == &mut 0 || previous_map == map {
    return map;
  }
  // Although it is named previous map, it was updated to be new map. And the map become to be previous map
  return recursive_round_handle(count, map, width, height, number_seat);
}

fn handle_seat(map: &HashMap<usize, char>, position: usize, width: usize, height: usize, number_seat: usize) -> char {

  let mut list = vec![];

  // PART 1
  if number_seat == 4 {
    if !limit_on_left(position, width, height) {
      list.push(Movement::West.go(position, width));
    }
    if !limit_on_right(position, width, height) {
      list.push(Movement::East.go(position, width));
    }
    if !limit_on_top(position, width, height) {
      list.push(Movement::North.go(position, width));
    }
    if !limit_on_bottom(position, width, height) {
      list.push(Movement::South.go(position, width));
    }
    if !(limit_on_top(position, width, height) || limit_on_left(position, width, height)) {
      list.push(Movement::Northwest.go(position, width));
    }
    if !(limit_on_top(position, width, height) || limit_on_right(position, width, height)) {
      list.push(Movement::Northeast.go(position, width));
    }
    if !(limit_on_bottom(position, width, height) || limit_on_left(position, width, height)) {
      list.push(Movement::Southwest.go(position, width));
    }
    if !(limit_on_bottom(position, width, height) || limit_on_right(position, width, height)) {
      list.push(Movement::Southeast.go(position, width));
    }
  } 
  // PART 2
  else if number_seat == 5 {
    if !limit_on_left(position, width, height) {
      list.push(get_next(map, position, width, height, Movement::West));
    }
    if !limit_on_right(position, width, height) {
      list.push(get_next(map, position, width, height, Movement::East));
    }
    if !limit_on_top(position, width, height) {
      list.push(get_next(map, position, width, height, Movement::North));
    }
    if !limit_on_bottom(position, width, height) {
      list.push(get_next(map, position, width, height, Movement::South));
    }
    if !(limit_on_top(position, width, height) || limit_on_left(position, width, height)) {
      list.push(get_next(map, position, width, height, Movement::Northwest));
    }
    if !(limit_on_top(position, width, height) || limit_on_right(position, width, height)) {
      list.push(get_next(map, position, width, height, Movement::Northeast));
    }
    if !(limit_on_bottom(position, width, height) || limit_on_left(position, width, height)) {
      list.push(get_next(map, position, width, height, Movement::Southwest));
    }
    if !(limit_on_bottom(position, width, height) || limit_on_right(position, width, height)) {
      list.push(get_next(map, position, width, height, Movement::Southeast));
    }
  }

  match map.get(&position).unwrap() {
    '#' => is_more_than_given_number_occupied(map, list, number_seat),
    'L' => is_no_adjacent_seat_occupied(map, list),
    _ => '.',
  }
}

fn get_next(map: &HashMap<usize, char>, position: usize, width: usize, height: usize, operation: Movement) -> usize {
  let new_position = operation.go(position, width);
  if map.contains_key(&(new_position)) {
    if *map.get(&(new_position)).unwrap() == '.'
      && !(limit_on_top(new_position, width, height) && operation == Movement::North)
      && !(limit_on_bottom(new_position, width, height) && operation == Movement::South)
      && !(limit_on_left(new_position, width, height) && operation == Movement::West)
      && !(limit_on_right(new_position, width, height) && operation == Movement::East)
      && !(limit_on_top_left(new_position, width, height) && operation == Movement::Northwest)
      && !(limit_on_top_right(new_position, width, height)  && operation == Movement::Northeast)
      && !(limit_on_bottom_left(new_position, width, height) && operation == Movement::Southwest)
      && !(limit_on_bottom_right(new_position, width, height) && operation == Movement::Southeast) {
      return get_next(map, new_position, width, height, operation);
    }
    return new_position;
  }
  return position;
}

fn limit_on_top(position: usize, width: usize, height: usize) -> bool {
  return position < width;
}

fn limit_on_bottom(position: usize, width: usize, height: usize) -> bool {
  return position > width * (height - 1);
}

fn limit_on_left(position: usize, width: usize, height: usize) -> bool {
  return position % width == 0;
}

fn limit_on_right(position: usize, width: usize, height: usize) -> bool {
  return position % width == width - 1;
}


fn limit_on_top_left(position: usize, width: usize, height: usize) -> bool {
  return position == 0 || limit_on_top(position, width, height) || limit_on_left(position, width, height);
}

fn limit_on_top_right(position: usize, width: usize, height: usize) -> bool {
  return position == (width - 1) || limit_on_top(position, width, height) || limit_on_right(position, width, height);
}

fn limit_on_bottom_left(position: usize, width: usize, height: usize) -> bool {
  return position == width * (height - 1) || limit_on_bottom(position, width, height) || limit_on_left(position, width, height);
}

fn limit_on_bottom_right(position: usize, width: usize, height: usize) -> bool {
  return position == (width * height - 1) || limit_on_bottom(position, width, height) || limit_on_right(position, width, height);
}

fn is_more_than_given_number_occupied(map: &HashMap<usize, char>, list: Vec<usize>, number_seat: usize) -> char {
  let mut count = 0;
  for position in list.clone() {
    //println!("map value: {:?} at position {}", *map.get(&position).unwrap(), position);
    if *map.get(&position).unwrap() == '#' {
      count += 1;
    }
  }
  if count >= number_seat {
    return 'L';
  }
  return '#';
}

fn is_no_adjacent_seat_occupied(map: &HashMap<usize, char>, list: Vec<usize>) -> char {
  for position in list {
    //println!("map value: {:?} at position {}", *map.get(&position).unwrap(), position);
    if *map.get(&position).unwrap() == '#' {
      return 'L';
    }
  }
  return '#';
}

fn show_map(map: HashMap<usize, char>, width: usize, height: usize) {
  for i in 0..height {
    let mut value: String = String::from("");
    for j in 0..width {
      value.push(*map.get(&((i * width) + j)).unwrap());
    }
    println!("{:?}", value);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    let path = "data/11th_day/test_input.txt";
    let map = &mut HashMap::new();
    let (width, height) = get_map_and_size(map, path);
    // Avoid stack overflow by limiting to 20 iterations, feel free to increment this
    let final_map = recursive_round_handle(&mut 20, map, width, height, 4);
    let occupied_seats = count_occupied_seats(final_map);
    assert_eq!(occupied_seats, 37);
  }

  #[test]
  fn test_part2() {
    let path = "data/11th_day/test_input.txt";
    let map = &mut HashMap::new();
    let (width, height) = get_map_and_size(map, path);
    // Avoid stack overflow by limiting to 20 iterations, feel free to increment this
    let final_map = recursive_round_handle(&mut 20, map, width, height, 5);
    let occupied_seats = count_occupied_seats(final_map);
    assert_eq!(occupied_seats, 26);
  }
}

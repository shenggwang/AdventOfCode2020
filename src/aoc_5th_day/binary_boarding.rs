use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u32 {
  let path = "data/5th_day/input.txt";
  return get_higher_seat_id(path);
}

pub fn compute2() -> u32 {
  let path = "data/5th_day/input.txt";
  return find_own_seat(path);
}

fn get_higher_seat_id(path: &str) -> u32 {
  let value = get_list_seat(path);
  let higher_seat_id = value.iter().max().expect("Unable to find max");
  return *higher_seat_id; // * makes &u32 -> u32
}

fn find_own_seat(path: &str) -> u32 {
  let list = get_list_seat(path);
  //let higher_seat_id = list.iter().max().expect("Unable to find max");
  //let lower_seat_id = list.iter().min().expect("Unable to find max");
  //println!("lower:{} higher:{} size:{}", lower_seat_id, higher_seat_id, list.len());
  for i in 0..list.len() {
    if list[i + 1] - list[i] > 1 {
      return list[i] + 1;
    } 
  }
  return 2020;
}

fn get_list_seat(path: &str) -> Vec<u32> {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut list = vec![];
  let row_range: Vec<u8> = vec![0, 127];
  let column_range: Vec<u8> = vec![0, 7];
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.");
    let row: u32 = get_value(&text[..7], &row_range) as u32;
    let column: u32 = get_value(&text[7..], &column_range) as u32;
    let seat_id: u32 = row * 8 + column;
    list.push(seat_id);
  }
  list.sort();
  return list;
}

/// i8 vs u8 = -128 until 127 vs 0 until 255
fn get_value(string: &str, v: &Vec<u8>) -> u8 {
  let mut vector: Vec<u8> = v.to_vec();
  for c in string.chars() { 
    match c {
      'F' | 'L' => vector = get_recursive_value(&vector, true).to_vec(),
      'B' | 'R' => vector = get_recursive_value(&vector, false).to_vec(),
      _ => println!("Character is not valid!"),
    }
  }
  return vector[1];
}

fn get_recursive_value(v: &Vec<u8>, is_lower: bool) -> Vec<u8> {
  let mut value: Vec<u8> = Vec::with_capacity(2 as usize);
  if is_lower {
    value.push(v[0]);
    value.push((v[0] + v[1]) / 2);
  } else {
    value.push((v[0] + v[1]) / 2);
    value.push(v[1]);
  }
  return value;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_value() {
    let row: u32 = get_value("FBFBBFF", &vec![0, 127]) as u32;
    let column: u32 = get_value("RLR", &vec![0, 7]) as u32;
    assert_eq!(row, 44);
    assert_eq!(column, 5);
    let seat_id:u32 = row * 8 + column;
    assert_eq!(seat_id, 357);

    let row: u32 = get_value("BFFFBBF", &vec![0, 127]) as u32;
    let column: u32 = get_value("RRR", &vec![0, 7]) as u32;
    assert_eq!(row, 70);
    assert_eq!(column, 7);
    let seat_id:u32 = row * 8 + column;
    assert_eq!(seat_id, 567);

    let row: u32 = get_value("FFFBBBF", &vec![0, 127]) as u32;
    let column: u32 = get_value("RRR", &vec![0, 7]) as u32;
    assert_eq!(row, 14);
    assert_eq!(column, 7);
    let seat_id:u32 = row * 8 + column;
    assert_eq!(seat_id, 119);

    let row: u32 = get_value("BBFFBBF", &vec![0, 127]) as u32;
    let column: u32 = get_value("RLL", &vec![0, 7]) as u32;
    assert_eq!(row, 102);
    assert_eq!(column, 4);
    let seat_id:u32 = row * 8 + column;
    assert_eq!(seat_id, 820);
  }

  #[test]
  fn test_get_higher_seat_id() {
    let path = "data/5th_day/test_input.txt";
    let value = get_higher_seat_id(path);
    assert_eq!(value, 820);
  }
}

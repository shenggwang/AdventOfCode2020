use std::{
  path::Path,
  io::{BufReader, prelude::*},
  fs::File
};

pub fn compute1() -> i32 {
  let path = Path::new("data/3rd_day/input.txt");
  let value: i32 = count_trees(path);
  return value;
}

pub fn compute2() -> i64 {
  let path = Path::new("data/3rd_day/input.txt");
  let slope1: i64 = count_trees_with_slope(path, 1, 1).into();
  let slope2: i64 = count_trees_with_slope(path, 3, 1).into();
  let slope3: i64 = count_trees_with_slope(path, 5, 1).into();
  let slope4: i64 = count_trees_with_slope(path, 7, 1).into();
  let slope5: i64 = count_trees_with_slope(path, 1, 2).into();
  let value: i64 = (slope1 * slope2 * slope3 * slope4 * slope5).into();
  return value;
}

fn count_trees(path: &Path) -> i32 {
  let file = File::open(path).expect("Unable to read file.");
  let buffer = BufReader::new(file);
  let mut x = 0;
  let mut count = 0;
  for line in buffer.lines() {
    let text = line.expect("Unable to read line.");
    let size = text.chars().count() - 1;
    let character = text.chars().nth(x).unwrap();
    if character == '#' {
      count += 1;
    }
    x += 3;
    if x > size {
      x = x - size - 1;
    }
  }
  return count;
}

fn count_trees_with_slope(path: &Path, right: i32, down: i32) -> i32 {
  let file = File::open(path).expect("Unable to read file.");
  let buffer = BufReader::new(file);
  let (mut x, mut y, mut count) = (0, 0, 0);
  for line in buffer.lines() {
    if !(y == 0 || y % down == 0) {
      y += 1;
      continue;
    }
    let text = line.expect("Unable to read line.");
    let size = text.chars().count() - 1;
    let character = text.chars().nth(x).unwrap();
    if character == '#' {
      count += 1;
    }
    x += right.wrapping_abs() as u32 as usize;
    y += 1;
    if x > size {
      x = x - size - 1;
    }
  }
  return count;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_count_trees() {
    let path = Path::new("data/3rd_day/test_input.txt");
    let value: i32 = count_trees(path);
    assert_eq!(value, 7);

    let path = Path::new("data/3rd_day/test_input2.txt");
    let value: i32 = count_trees(path);
    assert_eq!(value, 16);
  }

  #[test]
  fn test_count_trees_with_slope() {
    let path = Path::new("data/3rd_day/test_input2.txt");
    let value: i32 = count_trees_with_slope(path, 3, 1);
    assert_eq!(value, 16);
    let path = Path::new("data/3rd_day/test_input2.txt");
    let value: i32 = count_trees_with_slope(path, 2, 2);
    assert_eq!(value, 4);
  }
}

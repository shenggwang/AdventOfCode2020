use std::{
  path::Path,
  io::{BufReader, prelude::*},
  fs::File
};

pub fn compute1() -> usize {
  let path = Path::new("data/third_day/input.txt");
  let value: usize = count_trees(path);
  return value;
}

pub fn compute2() -> usize {
  let path = Path::new("data/third_day/input.txt");
  let slope1: usize = count_trees_with_slope(path, 1, 1);
  let slope2: usize = count_trees_with_slope(path, 3, 1);
  let slope3: usize = count_trees_with_slope(path, 5, 1);
  let slope4: usize = count_trees_with_slope(path, 7, 1);
  let slope5: usize = count_trees_with_slope(path, 1, 2);
  let value: usize = slope1 * slope2 * slope3 * slope4 * slope5;
  return value;
}

fn count_trees(path: &Path) -> usize {
  let file = File::open(path).expect("Unable to read file.");
  let buffer = BufReader::new(file);
  let (mut x, mut y) = (0, 0);
  let mut count = 0;
  for line in buffer.lines() {
    let text = line.expect("Unable to read line.");
    let size = text.chars().count() - 1;
    let character = text.chars().nth(x).unwrap();
    if character == '#' {
      count += 1;
    }
    x += 3;
    y += 1;
    if x > size {
      println!("end of line {} and {}", x, y);
      x = x - size - 1;
    }
  }
  return count;
}

fn count_trees_with_slope(path: &Path, right: usize, down: usize) -> usize {
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
    x += right;
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
    let path = Path::new("data/third_day/test_input.txt");
    let value: usize = count_trees(path);
    assert_eq!(value, 7);

    let path = Path::new("data/third_day/test_input2.txt");
    let value: usize = count_trees(path);
    assert_eq!(value, 16);
  }

  #[test]
  fn test_count_trees_with_slope() {
    let path = Path::new("data/third_day/test_input2.txt");
    let value: usize = count_trees_with_slope(path, 3, 1);
    assert_eq!(value, 16);
    let path = Path::new("data/third_day/test_input2.txt");
    let value: usize = count_trees_with_slope(path, 2, 2);
    assert_eq!(value, 4);
  }
}

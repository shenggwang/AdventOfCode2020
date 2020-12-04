use std::path::Path;

use crate::tools::file_handler::{
  read_file,
  split_with_expression
};

use regex::Regex;

pub fn compute1() -> i32 {
  let path = Path::new("data/2nd_day/input.txt");
  if let Ok(content) = read_file(path) {
    if let Ok(vector) = split_with_expression(&content, "\n") {
      let value = count_valid(&vector, true);
      return value;
    }
  }
  return 2020;
}

pub fn compute2() -> i32 {
  let path = Path::new("data/2nd_day/input.txt");
  if let Ok(content) = read_file(path) {
    if let Ok(vector) = split_with_expression(&content, "\n") {
      let value = count_valid(&vector, false);
      return value;
    }
  }
  return 2020;
}

fn count_valid(v: &Vec<String>, is_first_part: bool) -> i32 {
  let mut count:i32 = 0;
  for i in 0..v.len() {
    let re = Regex::new(r"\d+-\d+ \w: \w+").unwrap();
    if re.is_match(&v[i].clone()) && is_valid(v[i].clone(), is_first_part) {
      count += 1;
    }
  }
  return count;
}

fn is_valid(line: String, is_first_part: bool) -> bool {

  if let Ok(pair) = split_with_expression(&line, ":") {
    let password: &str = pair[1].trim();

    if let Ok(condition) = split_with_expression(&pair[0], " ") {
      if let Ok(between) = split_with_expression(&condition[0], "-") {
        let min: i32 = between[0].parse().unwrap();
        let max: i32 = between[1].parse().unwrap();
        let character = condition[1].chars().next().expect("string is empty");
        if is_first_part {
          return first_part(password, max, min, character);
        }
        return second_part(password, max, min, character);
      }
    }
  }
  
  return false;
}

fn first_part(password: &str, max: i32, min: i32, character: char) -> bool {
  let mut count: i32 = 0;
  for ch in password.chars() {
    if ch == character {
      count += 1;
    }
  }
  if count >= min && count <= max {
    return true;
  }
  return false;
}

fn second_part(password: &str, max: i32, min: i32, character: char) -> bool {
  if password.chars().nth(min as u32 as usize - 1).unwrap() == character 
    && password.chars().nth(max as u32 as usize - 1).unwrap() != character {
    return true;
  }
  if password.chars().nth(min as u32 as usize - 1).unwrap() != character 
    && password.chars().nth(max as u32 as usize - 1).unwrap() == character {
    return true;
  }
  return false;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_first_part() {
    let result1 = first_part("abcde", 3, 1, 'a');
    assert_eq!(result1, true);
    let result2 = first_part("cdefg", 3, 1, 'b');
    assert_eq!(result2, false);
    let result3 = first_part("ccccccccc", 9, 2, 'c');
    assert_eq!(result3, true);
  }
  #[test]
  fn test_second_part() {
    let result1 = second_part("abcde", 3, 1, 'a');
    assert_eq!(result1, true);
    let result2 = second_part("cdefg", 3, 1, 'b');
    assert_eq!(result2, false);
    let result3 = second_part("ccccccccc", 9, 2, 'c');
    assert_eq!(result3, false);
  }
}

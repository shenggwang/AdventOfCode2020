use std::{
  collections::HashMap
};

use crate::tools::file_handler::{
  read_file,
  split_numbers
};

pub fn compute1() -> i32 {
  if let Ok(content) = read_file("data/1st_day/input.txt") {
    if let Ok(vector) = split_numbers(&content) {
      let value = get_two_entries_sum_2020(&vector);
      return (value[0] * value[1]) as i32;
    }
  }
  return 2020;
}

pub fn compute2() -> i32 {
  if let Ok(content) = read_file("data/1st_day/input.txt") {
    if let Ok(vector) = split_numbers(&content) {
      let value = get_three_entries_sum_2020(&vector);
      return (value[0] * value[1] * value[2]) as i32;
    }
  }
  return 2020;
}

fn get_two_entries_sum_2020(v: &Vec<usize>) -> Vec<usize> {
  let mut two_size_vector: Vec<usize> = Vec::with_capacity(2 as usize);
  let mut map_for_vector = HashMap::new();

  for i in 0..v.len() {
    let key = 2020 - v[i];
    if map_for_vector.contains_key(&key) {
      let value = map_for_vector[&key];
      two_size_vector.push(v[value]);
      two_size_vector.push(v[i]);
      break;
    }
    map_for_vector.insert(v[i], i);
  }
  
  return two_size_vector;
}

fn get_three_entries_sum_2020(v: &Vec<usize>) -> Vec<usize> {
  let three_size_vector: Vec<usize> = Vec::with_capacity(3 as usize);
  let mut vector = v.clone();
  vector.sort();
  for i in 0..vector.len()-1 {
    if i == 0 || (i > 0 && vector[i] != vector[i-1]) {
      let (mut low, mut high, sum):(usize, usize, usize) = (i + 1, vector.len() - 1, 2020 - vector[i]);
      
      while low < high {
        if vector[low] + vector[high] == sum {
          return vec![vector[i], vector[low], vector[high]];
        } else if vector[low] + vector[high] < sum {
          low += 1;
        } else {
          high -= 1;
        }
      }
    }
  }
  
  return three_size_vector;
}

#[cfg(test)]
mod test {
  use super::*;

  fn nums() -> Vec<usize> {
    vec![
      1721,
      979,
      366,
      299,
      675,
      1456,
    ]
  } 

  #[test]
  fn test_get_two_entries_sum_2020() {
    let v = get_two_entries_sum_2020(&Vec::from(nums()));
    assert_eq!(v, [1721, 299]);
    assert_eq!(v[0] * v[1], 514579);
  }
  #[test]
  fn test_get_three_entries_sum_2020() {
    let v = get_three_entries_sum_2020(&Vec::from(nums()));
    assert_eq!(v, [366, 675, 979]);
    assert_eq!(v[0] * v[1] * v[2], 241861950);
  }
}

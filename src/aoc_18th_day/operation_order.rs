use std::{
  io::{BufReader, BufRead},
  fs::File,
  collections::HashMap
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u64 {
  let path = "data/15th_day/input.txt";
  let input = get_calculate(path);
  let result = 2020;
  return result as u64;
}

pub fn compute2() -> u64 {
  let path = "data/15th_day/input.txt";
  let input = get_calculate(path);
  let result = 2020;
  return result as u64;
}

fn get_calculate(path: &str) -> usize {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut value = 0;
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    value += calculate(text.as_str());
  }
  return value;
}

/*
fn calculate(line: &str) -> usize {
  let list: Vec<char> = line.chars().filter(|x| *x != ' ').collect();
  let mut stack = vec![];
  let mut result = vec![];
  for index in 0..list.len() {
    let character = list[index];
    match character {
      '(' => {
        stack.push(character);
      },
      ')' => {
        while let Some(t) = stack.pop() {
          if t == '(' {
            break
          } else {
            result.push(t);
          }
        }
      },
      '*' | '+' => {
        while let Some(t) = stack.last() {
          if *t == '+' || *t == '*' {
            result.push(*t);
            stack.pop();
          } else {
              break;
          }
        }
        stack.push(character);
      },
      _ => result.push(character),
    }
  }
  while let Some(t) = stack.pop() {
    result.push(t);
  }
  // '1'.to_digit(10).unwrap() as usize
  println!("debug: {:?}", result);
  return 2020;
}
*/

fn calculate(line: &str) -> usize {
  let list: Vec<char> = line.chars().filter(|x| *x != ' ').collect();
  let mut previous_number = vec![];
  let mut previous_index = 0;
  let mut previous_operator = ' ';
  let mut all_operator = vec![];
  for index in 0..list.len() {
    let character = list[index];
    match character {
      '(' => {
        if index > 0 && list[index - 1] != '(' {
          all_operator.push(list[index - 1]);
        } else {
          previous_index += 1;  // TODO use push and pop
          previous_number.push(0);
        }
      },
      ')' => {
        if index < list.len() - 1 && list[index + 1] != ')' {
          all_operator.push(list[index + 1]);
        } else {
          previous_index += 1;
          previous_number.push(0);
        }
      },
      '*' | '+' => {
        previous_operator = character
      },
      _ => {
        println!("index: {} value: {}, previous number {:?}, previous_index {}", index, character, previous_number, previous_index);
        let length = previous_number.len();
        if length == 0 {
          previous_number.push(character.to_digit(10).unwrap() as usize);
        } else if previous_number[length - 1] == 0 {
          previous_number[length - 1] = character.to_digit(10).unwrap() as usize;
        } else {
          if previous_operator == '+' {
            previous_number[previous_index] += character.to_digit(10).unwrap() as usize;
          } else {
            previous_number[previous_index] *= character.to_digit(10).unwrap() as usize;
          }
        }
      },
    }
  }
  println!("numbers: {:?}", previous_number);
  println!("operators: {:?}", all_operator);
  return 2020;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    /*
    let test_input1 = "1 + 2 * 3 + 4 * 5 + 6";
    assert_eq!(calculate(test_input1), 71);

    let test_input2 = "1 + (2 * 3) + (4 * (5 + 6))";
    assert_eq!(calculate(test_input2), 51);

    let test_input3 = "2 * 3 + (4 * 5)";
    assert_eq!(calculate(test_input3), 26);

    let test_input4 = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(calculate(test_input4), 437);

    let test_input5 = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(calculate(test_input5), 12240);

    let test_input6 = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(calculate(test_input6), 13632);
    */

  }

  #[test]
  fn test_part2() {
    /* TODO
    let test_input1 = "0,3,6".to_string();
    assert_eq!(number_at_give_value(test_input1, 30000000), 175594);

    let test_input2 = "1,3,2".to_string();
    assert_eq!(number_at_give_value(test_input2, 30000000), 2578);

    let test_input3 = "2,1,3".to_string();
    assert_eq!(number_at_give_value(test_input3, 30000000), 3544142);

    let test_input4 = "1,2,3".to_string();
    assert_eq!(number_at_give_value(test_input4, 30000000), 261214);

    let test_input5 = "2,3,1".to_string();
    assert_eq!(number_at_give_value(test_input5, 30000000), 6895259);

    let test_input6 = "3,2,1".to_string();
    assert_eq!(number_at_give_value(test_input6, 30000000), 18);
    
    let test_input7 = "3,1,2".to_string();
    assert_eq!(number_at_give_value(test_input7, 30000000), 362);
    */
  }
}

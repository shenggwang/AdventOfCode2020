use std::{
  io::{BufReader, BufRead},
  fs::File
};

use crate::tools::file_handler::get_buffer_file;

pub fn compute1() -> u64 {
  let path = "data/18th_day/input.txt";
  let result = get_calculate(path, calculate);
  return result as u64;
}

pub fn compute2() -> u64 {
  let path = "data/18th_day/input.txt";
  let result = get_calculate(path, advanced_calculate);
  return result as u64;
}

fn get_calculate(path: &str, f: fn(Vec<String>) -> usize) -> usize {
  let buffer: BufReader<File> = get_buffer_file(path);
  let mut value = 0;
  for line in buffer.lines() {
    let text: String = line.expect("Unable to read line.").parse().unwrap();
    value += f(build_input(text.as_str()));
  }
  return value;
}

fn calculate(list: Vec<String>) -> usize {
  let mut stack = vec![];
  let mut result = 0;
  // 0 = null; 1 = '+'; 2 = '*'
  let mut operation = 0;
  for index in 0..list.len() {
    let character = list[index].as_str();
    if character.chars().all(char::is_numeric) {
      let number = character.parse().unwrap();
      if result == 0 {
        result = number;
      } 
      if operation != 0 {
        if operation == 1 {
          result += number;
        } else if operation == 2 {
          result *= number;
        }
      }
    } else if character == "+" {
      operation = 1;
    } else if character == "*" {
      operation = 2;
    } else if character == "(" {
      stack.push(result);
      stack.push(operation);
      result = 0;
      operation = 0;
    } else if character == ")" {
      let store_operation = stack.pop().unwrap();
      let store_number = stack.pop().unwrap();
      if store_operation != 0 {
        if store_operation == 1 {
          result += store_number;
        } else if store_operation == 2 {
          result *= store_number;
        }
      }
    }
  }
  return result;
}


fn advanced_calculate(list: Vec<String>) -> usize {

  let mut new_list = sum_operation_first(list);
  let mut timeout = 0;
  while timeout < 100 {
    if new_list.iter().any(|x| x == "+") {
      //println!("list: {:?}", new_list);
      new_list = sum_operation_first(new_list);
      //println!("sum - list after sum operation: {:?}", new_list);
      if new_list.len() != 1 {
        new_list = handle_values_within_parantheses(new_list);
        //println!("handle - list after handling within parentheses: {:?}", new_list);
      }
    } else {
      return calculate(new_list);
    }
    timeout += 1;
  }
  return 2020;
}

fn sum_operation_first(list: Vec<String>) -> Vec<String> {
  let mut new_list = vec![];
  let mut number = 0;
  let sum_operator = "+";
  let end_parentheses = ")";

  let list_size = list.len();

  for index in 0..list_size {
    let character = list[index].as_str();
    if character.chars().all(char::is_numeric) {
      // if: not + n +;
      if index != 0 && index != list_size - 1 && list[index - 1] != sum_operator && list[index + 1] != sum_operator {
        new_list.push(character.to_string());
        number = 0;
      // if: first element
      } else if index == 0 && list[index + 1] != sum_operator {
        new_list.push(character.to_string());
      // if last element
      } else if index == list_size - 1 {
        if list[index - 1] != sum_operator {
          new_list.push(character.to_string());
        } else {
          number += character.parse::<i32>().unwrap();
          new_list.push(number.to_string());
          number = 0;
        }
      // if: + 1
      } else if index != 0 && list[index - 1] == sum_operator {
        if number != 0 {
          number = number + character.parse::<i32>().unwrap();
          // if last element after operator +
          if index == list_size - 1 || (index != list_size - 1 && list[index + 1] != sum_operator) {
            new_list.push(number.to_string());
            number = 0;
          }
        } else if number == 0 && index != list_size - 1 && list[index + 1] != sum_operator {
          new_list.push(character.to_string());
        } else {
          number = number + character.parse::<i32>().unwrap();
        }
      // if: n +
      } else if index != list_size - 1 && list[index + 1] == sum_operator {
        number += character.parse::<i32>().unwrap();
      }
    } else if character == sum_operator {
      if index != list_size - 1 && !list[index + 1].chars().all(char::is_numeric) {
        if number != 0 {
          new_list.push(number.to_string());
          number = 0;
        }
        new_list.push(character.to_string());
      } else if index != 0 && list[index - 1] == end_parentheses {
        new_list.push(character.to_string());
      }
    } else {
      new_list.push(character.to_string());
    }
    //println!("Number: {:?}, character: {:?}, list: {:?}", number, character, new_list);
  }
  return new_list;
}

fn handle_values_within_parantheses(list: Vec<String>) -> Vec<String> {
  let mut new_list: Vec<String> = vec![];
  let mut within_parantheses = vec![];
  let start_parentheses = "(";
  let end_parentheses = ")";
  let sum_operator = "+";
  let mul_operator = "*";
  let list_size = list.len();

  for index in 0..list_size {
    let character = list[index].as_str();
    let within_parantheses_size = within_parantheses.len();
    if character.chars().all(char::is_numeric) || character == sum_operator || character == mul_operator {
      if within_parantheses_size != 0 {
        within_parantheses.push(character.to_string());
      } else {
        new_list.push(character.to_string());
      }
    } else if character == start_parentheses {
      if within_parantheses_size == 0 {
        within_parantheses.push(character.to_string());
      } else {
        for value in within_parantheses {
          new_list.push(value);
        }
        within_parantheses = vec![start_parentheses.to_string()];
      }
    } else if character == end_parentheses {
      if within_parantheses_size == 0 {
        new_list.push(character.to_string());
      } else if !within_parantheses.contains(&sum_operator.to_string()) {
        let vec = within_parantheses.clone();
        let value = calculate(vec);
        new_list.push(value.to_string());
        within_parantheses = vec![];
      } else {
        for value in &within_parantheses {
          new_list.push(value.to_string());
        }
        new_list.push(character.to_string());
      }
    }
    //println!("Within parentheses: {:?}, character: {:?}, list: {:?}", within_parantheses, character, new_list);
  }
  return new_list;
}

fn build_input(line: &str) -> Vec<String> {
  return line.chars().filter(|x| *x != ' ').map(|x| x.to_string()).collect();
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_part1() {
    
    let test_input1 = build_input("1 + 2 * 3 + 4 * 5 + 6");
    assert_eq!(calculate(test_input1), 71);

    let test_input2 = build_input("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(calculate(test_input2), 51);

    let test_input3 = build_input("2 * 3 + (4 * 5)");
    assert_eq!(calculate(test_input3), 26);

    let test_input4 = build_input("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    assert_eq!(calculate(test_input4), 437);

    let test_input5 = build_input("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    assert_eq!(calculate(test_input5), 12240);

    let test_input6 = build_input("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(calculate(test_input6), 13632);

  }

  #[test]
  fn test_part2() {
    
    let test_input1 = build_input("1 + 2 * 3 + 4 * 5 + 6");
    assert_eq!(advanced_calculate(test_input1), 231);

    let test_input2 = build_input("1 + (2 * 3) + (4 * (5 + 6))");
    assert_eq!(advanced_calculate(test_input2), 51);
    
    let test_input3 = build_input("2 * 3 + (4 * 5)");
    assert_eq!(advanced_calculate(test_input3), 46);

    let test_input4 = build_input("5 + (8 * 3 + 9 + 3 * 4 * 3)");
    assert_eq!(advanced_calculate(test_input4), 1445);

    let test_input5 = build_input("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))");
    assert_eq!(advanced_calculate(test_input5), 669060);
    
    let test_input6 = build_input("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
    assert_eq!(advanced_calculate(test_input6), 23340);

  }
}

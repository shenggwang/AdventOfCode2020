use std::{
  error, 
  result,
  path::Path,
  io::{BufReader},
  fs::{File, read_to_string}
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

pub fn get_buffer_file(p: &str) -> BufReader<File> {
  let path = Path::new(p);
  let file = File::open(path).expect("Unable to read file.");
  BufReader::new(file)
}

pub fn read_file(p: &str) -> TResult<String> {
  let path = Path::new(p);
  read_to_string(path).map_err(|e| e.into())
}

pub fn split_numbers(s: &String) -> TResult<Vec<usize>> {
  s.split_whitespace()
    .map(|x| x.parse::<usize>()
    .map_err(|e| e.into())
    )
    .collect()
}

pub fn split_with_expression(s: &String, e: &str) -> TResult<Vec<String>> {
  s.split(e)
    .map(|x| x.parse::<String>()
    .map_err(|e| e.into())
    )
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;
  use regex::Regex;
  
  #[test]
  fn test_read_file() {
    let res = read_file("data/1st_day/test_input.txt");
    assert!(res.is_ok());
    
    if let Ok(s) = res {
      assert_eq!(s, "3\n5");
    }
  }
  #[test]
  fn test_split_numbers() {
    let res = split_numbers(&String::from("3\n5"));
    assert!(res.is_ok());

    if let Ok(v) = res {
      assert_eq!(v, [3, 5]);
    }
  }

  #[test]
  fn test_regex() {
    let re = Regex::new(r"\w+").unwrap();
    let mut count = re.find_iter("this is a test to see regex").count();
    assert_eq!(count, 7);
    count = re.find_iter("21 32 32 a 2").count();
    assert_eq!(count, 5);
  }

  #[test]
  fn test_regex2() {
    let re = Regex::new(r"a|b").unwrap();
    let mut count = re.find_iter("this is a test to see regex").count();
    assert_eq!(count, 1);
    count = re.find_iter("abc a b c 12 2 ba").count();
    assert_eq!(count, 6);
  }
}


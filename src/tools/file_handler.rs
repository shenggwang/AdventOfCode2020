use std::{
  error, 
  result,
  path::Path,
  fs::{read_to_string}
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

pub fn read_file(p: &Path) -> TResult<String> {
  read_to_string(p).map_err(|e| e.into())
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

  #[test]
  fn test_read_file() {
    let path = Path::new("data/first_day/test_input.txt");
    let res = read_file(path);
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
}


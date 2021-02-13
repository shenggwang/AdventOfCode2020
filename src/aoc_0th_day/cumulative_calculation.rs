use std::io::{stdin};
pub fn compute1() -> usize {
  let mut buffer1 = String::new();
  let mut buffer2 = String::new();
  let mut buffer3 = String::new();
  println!("Write the initial value below:");
  stdin().read_line(&mut buffer1).expect("Failed reading line");
  let initial = buffer1.trim().parse::<f64>().unwrap();

  println!("Write the percent below:");
  stdin().read_line(&mut buffer2).expect("Failed reading line");
  let percent = buffer2.trim().parse::<f64>().unwrap();

  println!("Write the years below:");
  stdin().read_line(&mut buffer3).expect("Failed reading line");
  let years = buffer3.trim().parse::<u16>().unwrap();
  println!("Initial value is {} and percent is {} within {} years", initial, percent, years);

  return compound_return(initial, percent, years);
}

pub fn compound_return(initial: f64, percent: f64, years: u16) -> usize {
  let mut value = initial;
  for i in 1..=years {
    value = value + value * (percent / 100.0);
    if i % 10 == 0 || i == years {
      println!("value is {} in {} years", value, i);
    }
  }
  return value.round() as usize;
}


#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_9_dot_5_after_106_years() {
    let value = compound_return(1.0, 9.5, 106);
    assert_eq!(value, 15062);
  }
  #[test]
  fn test_6_dot_5_after_106_years() {
    let value = compound_return(1.0, 6.5, 106);
    assert_eq!(value, 793);
  }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Program {
  // vector of 36 bits
  pub mask: Vec<char>,
  // vector of tuple (memory address, decimal value)
  pub memory: Vec<(usize, usize)>,
}

impl From<&str> for Program {
  fn from(s: &str) -> Self {
    let mask: Vec<char> = s.chars().collect();
    let memory = Vec::new();
    Program {
      mask,
      memory,
    }
  }
}

impl Program {
  pub fn new() -> Program {
    let mask = Vec::new();
    let memory = Vec::new();
    Program {
      mask,
      memory,
    }
  }

  pub fn add(&mut self, n: usize, s: usize) -> () {
    self.memory.push((n, s));
  }

  pub fn convert_to_binary(value: usize) -> String {
    format!("{:036b}", value)
  }

  pub fn convert_to_usize(result: &str) -> usize {
    usize::from_str_radix(result, 2).unwrap()
  }

  pub fn get_all_combinations(floating: String) -> Vec<String> {
    //println!("floating: {:?}", floating);
    let number_of_x = floating.chars().filter(|x| *x == 'X').count();

    let mut all_combinations = vec![];
    let length:usize  = 2_usize.pow(number_of_x as u32);

    for index in 0..length {
      all_combinations.push(
        Program::convert_to_binary(index)
          .chars().rev().take(number_of_x).collect()
      );
    }
    return all_combinations;
  }
}

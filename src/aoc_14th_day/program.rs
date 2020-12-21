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
}

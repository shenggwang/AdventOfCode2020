#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Action {
  North,
  South,
  East,
  West,
  Left,
  Right,
  Forward,
}

#[derive(Clone, Debug)]
pub struct Movement {
  pub action: Action,
  pub value: isize,
}

impl From<String> for Movement {
  fn from(s: String) -> Self {
    let v = s.split_at(1);
    //println!("Splitted: {:?}", v);
    let action: Action = match v.0 {
      "N" => Action::North,
      "S" => Action::South,
      "W" => Action::West,
      "E" => Action::East,
      "L" => Action::Left,
      "R" => Action::Right,
      "F" => Action::Forward,
      _ => Action::Forward,
    };
    let value: isize = v.1.parse().unwrap();
    Movement {
      action,
      value,
    }
  }
}

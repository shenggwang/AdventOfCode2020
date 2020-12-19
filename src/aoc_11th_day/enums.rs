#[derive(PartialEq, Eq)]
pub enum Movement {
  North,
  South,
  West,
  East,
  Northeast,
  Northwest,
  Southeast,
  Southwest,
}

impl Movement {
  pub fn go(&self, position: usize, width: usize) -> usize {
    match self {
      Self::North => position - width,
      Self::South => position + width,
      Self::West => position - 1,
      Self::East => position + 1,
      Self::Northwest => position - (width + 1),
      Self::Northeast => position - (width - 1),
      Self::Southwest => position + (width - 1),
      Self::Southeast => position + (width + 1),
    }
  }
}

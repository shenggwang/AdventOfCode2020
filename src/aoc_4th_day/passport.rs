pub struct Passport {
  birth_year: Option<usize>,
  issue_year: Option<usize>,
  expiration_year: Option<usize>,
  height: Option<String>,
  hair_color: Option<String>,
  eye_color: Option<String>,
  passport_id: Option<String>,
  country_id: Option<String>
}

impl Passport {

  pub fn new() -> Passport {
    Passport {
      birth_year: None,
      issue_year: None,
      expiration_year: None,
      height: None,
      hair_color: None,
      eye_color: None,
      passport_id: None,
      country_id: None,
    }
  }
  pub fn birth_year(&mut self, value: String) {
    self.birth_year = Some(value.parse().unwrap());
  }
  pub fn issue_year(&mut self, value: String) {
    self.issue_year = Some(value.parse().unwrap());
  }
  pub fn expiration_year(&mut self, value: String) {
    self.expiration_year = Some(value.parse().unwrap());
  }
  pub fn height(&mut self, value: String) {
    self.height = Some(value);
  }
  pub fn hair_color(&mut self, value: String) {
    self.hair_color = Some(value);
  }
  pub fn eye_color(&mut self, value: String) {
    self.eye_color = Some(value);
  }
  pub fn passport_id(&mut self, value: String) {
    self.passport_id = Some(value);
  }
  pub fn country_id(&mut self, value: String) {
    self.country_id = Some(value);
  }

  pub fn is_valid(&self) -> bool {
    if self.birth_year.is_some()
      && self.issue_year.is_some()
      && self.expiration_year.is_some()
      && self.height.is_some()
      && self.hair_color.is_some()
      && self.eye_color.is_some()
      && self.passport_id.is_some() {
        return true;
      }
    return false;
  }

  pub fn is_valid_strict(&self) -> bool {
    if self.valid_birth_year()
      && self.valid_issue_year()
      && self.valid_expiration_year()
      && self.valid_height()
      && self.valid_hair()
      && self.valid_eyes()
      && self.valid_passport_id()
      {
        return true;
      }
    return false;
  }

  fn valid_birth_year(&self) -> bool {
    (1920..=2002).contains(&self.birth_year.unwrap_or_default())
  }

  fn valid_issue_year(&self) -> bool {
      (2010..=2020).contains(&self.issue_year.unwrap_or_default())
  }

  fn valid_expiration_year(&self) -> bool {
      (2020..=2030).contains(&self.expiration_year.unwrap_or_default())
  }

  fn valid_height(&self) -> bool {
      if let Some(height) = self.height.as_ref() {
          let range = match &height[height.len() - 2..] {
              "in" => (59..=76),
              "cm" => (150..=193),
              _ => return false,
          };
          range.contains(&height[0..height.len() - 2].parse::<usize>().unwrap_or(0))
      } else {
          false
      }
  }

  fn valid_hair(&self) -> bool {
      Passport::valid_str(self.hair_color.as_ref(), r"^#[0-9a-f]{6}$")
  }

  fn valid_eyes(&self) -> bool {
      Passport::valid_str(self.eye_color.as_ref(), r"^amb|blu|brn|gry|grn|hzl|oth$")
  }

  fn valid_passport_id(&self) -> bool {
      Passport::valid_str(self.passport_id.as_ref(), r"^[0-9]{9}$")
  }

  fn valid_str(maybe_str: Option<&String>, re: &str) -> bool {
      if let Some(str) = maybe_str {
          let re = regex::Regex::new(re).unwrap();
          let captures = re.captures(str.as_str());
          captures.is_some()
      } else {
          false
      }
  }

  /// Fails if any value is None
  pub fn print_value(&mut self) {
    println!("{:?}", &self.birth_year.clone().unwrap());
    println!("{:?}", &self.issue_year.clone().unwrap());
    println!("{:?}", &self.expiration_year.clone().unwrap());
    println!("{:?}", &self.height.clone().unwrap());
    println!("{:?}", &self.hair_color.clone().unwrap());
    println!("{:?}", &self.eye_color.clone().unwrap());
    println!("{:?}", &self.passport_id.clone().unwrap());
    println!("--------------------------------");
    //println!("{:?} with unwraps to {:?}", &self.country_id.clone(), &self.country_id.clone().unwrap());
  }
}

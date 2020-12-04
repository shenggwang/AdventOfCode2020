pub struct Passport {
  /*
    byr (Birth Year)
    iyr (Issue Year)
    eyr (Expiration Year)
    hgt (Height)
    hcl (Hair Color)
    ecl (Eye Color)
    pid (Passport ID)
    cid (Country ID) [optional]
    */
  byr: Option<String>,
  iyr: Option<String>,
  eyr: Option<String>,
  hgt: Option<String>,
  hcl: Option<String>,
  ecl: Option<String>,
  pid: Option<String>,
  cid: Option<String>
}

impl Passport {
  pub fn new() -> Passport {
    Passport {
      byr: None,
      iyr: None,
      eyr: None,
      hgt: None,
      hcl: None,
      ecl: None,
      pid: None,
      cid: None,
    }
  }
  pub fn byr(&mut self, value: String) {
      self.byr = Some(value);
  }
  pub fn print_value(&mut self) {
    let byr = &self.byr;
    println!("{:?} with unwraps to {:?}", byr.clone(), byr.clone().unwrap());
  }
}
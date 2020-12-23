use regex::Regex;
pub enum Input {
  Rule,
  Ticket,
  NearbyTicket,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Ticket {
  pub rules: Vec<(usize, usize)>,
  pub ticket: Vec<String>,
  pub nearby_ticket: Vec<String>
}

impl Ticket {
  pub fn new() -> Ticket {
    let rules = Vec::new();
    let ticket = Vec::new();
    let nearby_ticket = Vec::new();
    Ticket {
      rules,
      ticket,
      nearby_ticket,
    }
  }
  pub fn append_rules(&mut self, rule_line: String) -> () {
    let re = Regex::new(r"\d+-\d+").unwrap();
    let captures = re.captures(rule_line.as_str()).unwrap();
    let first_part = captures.get(0).map_or("", |m| m.as_str());
    // BUG Regex can only catch the first "\d+-\d+", thus the second part is null
    //let second_part = captures.get(1).map_or("", |m| m.as_str());
    let second_part: Vec<&str> = rule_line.split(" or ").collect();

    self.handle_rule(first_part);
    self.handle_rule(second_part[1]);
  }

  pub fn append_ticket(&mut self, ticket_line: String) -> () {
    self.ticket.push(ticket_line);
  }
  pub fn append_nearby_ticket(&mut self, ticket_line: String) -> () {
    self.nearby_ticket.push(ticket_line);
  }

  fn handle_rule(&mut self, part: &str) {
    let rule: Vec<usize> = part
      .split("-")
      .map(|x| x.parse().unwrap())
      .collect();
    for index in 0..self.rules.clone().len() {
      let tuple = self.rules[index];
      //println!("tuple: {:?} && rule: {:?}", tuple, rule);
      if rule[0] > tuple.0 && rule[0] < tuple.1 || (rule[1] > tuple.0 && rule[1] < tuple.1) {
        let minor = if rule[0] < tuple.0 { rule[0] } else { tuple.0 };
        let major = if rule[1] > tuple.1 { rule[1] } else { tuple.1 };
        self.rules[index] = (minor, major);
        return;
      }
    }
    self.rules.push((rule[0], rule[1]));
  }
}

use std::{
  fmt::Debug,
  hash::Hash,
  collections::HashMap
};
use regex::Regex;


pub enum Input {
  Rule,
  Ticket,
  NearbyTicket,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Ticket {
  pub rules: Vec<(String, String, String)>,
  pub ticket: Vec<String>,
  pub nearby_tickets: Vec<String>
}

impl Ticket {
  pub fn new() -> Ticket {
    let rules = Vec::new();
    let ticket = Vec::new();
    let nearby_tickets = Vec::new();
    Ticket {
      rules,
      ticket,
      nearby_tickets,
    }
  }
  pub fn append_rules(&mut self, rule_line: String) -> () {
    self.rules.push(Ticket::parse_rules(rule_line));
  }
  pub fn append_ticket(&mut self, ticket_line: String) -> () {
    self.ticket.push(ticket_line);
  }
  pub fn append_nearby_tickets(&mut self, ticket_line: String) -> () {
    self.nearby_tickets.push(ticket_line);
  }

  pub fn get_rules(&self) -> Vec<(usize, usize)>{
    let mut vec = vec![];
    let rules = &self.rules.clone();
    for tuple in rules {
      self.handle_rule(&mut vec, &tuple.1);
      self.handle_rule(&mut vec, &tuple.2);
    }
    return vec;
  }

  pub fn get_error_rate(&mut self) -> usize {
    let mut result = 0;

    for nearby_ticket in self.nearby_tickets.clone() {
      let numbers: Vec<usize> = nearby_ticket
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
      for number in numbers {
        if !self.within_the_rule(number) {
          result += number;
        }
      }
    }
    return result;
  }

  pub fn get_rules_with_name(&mut self) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    // TODO give rule to ticket number
    // 1 - get first rule
    // 2 - get index that all the nearby_rules fit
    // 3 - save the index as key and corresponding String as value
    // 4 - get second rule
    // 5 - get index that all the nearby_rules fit
    // 6 - if there is one already fit then move on
    // 7 - if there is no more that fit it then put it to that one and go nack to that one and iterate it

    let rules = &self.rules;
    let nearby_tickets = &self.nearby_tickets;
    for index in 0..rules.len() {
      let tuple = &rules[index];
      // Gets the size of first ticket. (note: all tickets has same size)
      let ticket_size = nearby_tickets[0].split(",").count();
      for i in 0..ticket_size {
        if map.contains_key(&i) {
            // TODO if one there is already one that fits here
            
            continue;
        }
        let value = nearby_tickets.clone()
          .iter()
          .filter(|x| x
            .split(",")
            .nth(i)
            .filter(|x| self.is_within(x.parse().unwrap(), tuple.1.as_str()) 
              || self.is_within(x.parse().unwrap(), tuple.2.as_str()))
            .is_some()
          )
          .count();
        if value == nearby_tickets.len() {
          map.insert(i, tuple.0.to_string());
          break;
        }
      }
    }
    return map;
  }

  pub fn get_value(&self, list: Vec<usize>) -> usize {
    let vec: Vec<usize> = self.ticket[0].split(",").map(|x| x.parse().unwrap()).collect();
    let mut value = 0;
    for index in list.iter() {
      //value += vec[index];
    }
    return value;
  }

  pub fn get_index_of(map: &HashMap<usize, String>, rule_name: &str) -> Vec<usize> {
    let mut vec = vec![];
    for (k, v) in map.iter() {
      //println!("key: {:?}, value: {:?}", *k, *v);
      if v.contains(rule_name) {
        vec.push(*k);
      }
    }
    return vec;
  }

  fn parse_rules(rule_line: String) -> (String, String, String) {
    let rule_name: Vec<&str> = rule_line.split(":").collect();
    let re = Regex::new(r"\d+-\d+").unwrap();
    let captures = re.captures(rule_line.as_str()).unwrap();
    let first_part = captures.get(0).map_or("", |m| m.as_str());
    // BUG Regex can only catch the first "\d+-\d+", thus the second part is null
    //let second_part = captures.get(1).map_or("", |m| m.as_str());
    let second_part: Vec<&str> = rule_line.split(" or ").collect();

    (rule_name[0].to_string(), first_part.to_string(), second_part[1].to_string())
  }

  fn is_within(&self, number: usize, part: &str) -> bool {
    let rule: Vec<usize> = part
      .split("-")
      .map(|x| x.parse().unwrap())
      .collect();
    if rule[0] <= number && rule[1] >= number {
      return true;
    }
    return false;
  }

  fn handle_rule(&self, vec: &mut Vec<(usize, usize)>, part: &str) -> () {
    let rule: Vec<usize> = part
      .split("-")
      .map(|x| x.parse().unwrap())
      .collect();

    for index in 0..vec.clone().len() {
      let tuple = vec[index];
      //println!("tuple: {:?} && rule: {:?}", tuple, rule);
      if rule[0] > tuple.0 && rule[0] < tuple.1 || (rule[1] > tuple.0 && rule[1] < tuple.1) {
        let minor = if rule[0] < tuple.0 { rule[0] } else { tuple.0 };
        let major = if rule[1] > tuple.1 { rule[1] } else { tuple.1 };
        vec[index] = (minor, major);
        return;
      }
    }
    vec.push((rule[0], rule[1]));
  }

  fn within_the_rule(&mut self, number: usize) -> bool {
    for tuple in self.get_rules() {
      if number >= tuple.0 && number <= tuple.1 {
        return true;
      }
    }
    return false;
  }
}

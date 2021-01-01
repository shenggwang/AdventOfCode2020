use std::{
  fmt::Debug,
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

  pub fn get_rules_with_name(&mut self) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    let rules = &self.rules;
    let my_ticket= &self.ticket.clone()[0];
    let nearby_tickets = &self.nearby_tickets;

    // Gets the size of first ticket. (note: all tickets has same size)
    let ticket_size = my_ticket.split(",").count();
    let my_ticket = my_ticket.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    for index in 0..rules.len() {
      let tuple = &rules[index];
      for i in 0..ticket_size {
        let value = nearby_tickets.clone()
          .iter()
          .filter(|x| {
            let number = x.split(",")
              .nth(i).unwrap();
            let condition = self.is_between_two_rules(number.parse().unwrap(), tuple.1.as_str(), tuple.2.as_str());
            return !condition;
          })
          .count();
        // check if it also works for my ticket
        let condition = self.is_between_two_rules(my_ticket[i], tuple.1.as_str(), tuple.2.as_str());
        if value == nearby_tickets.len() && !condition {
          map.entry(i).or_insert(vec![]).push(tuple.0.to_string());
        }
      }
    }
    let mut output_map = HashMap::new();
    return Ticket::select_correct_one(&mut map, &mut output_map);
  }

  pub fn get_list_of_rules_name_by_substring(self, rule_name: &str) -> Vec<String> {
    let mut vec = vec![];
    let rules = &self.rules.clone();
    for tuple in rules {
      let name = &tuple.0;
      if name.contains(rule_name) {
        vec.push(name.clone());
      }
    }
    return vec;
  }

  pub fn get_index_of(map: &HashMap<String, usize>, rule_name: &str) -> usize {

    return *map.get(rule_name).unwrap();
  }

  fn select_correct_one(map: &mut HashMap<usize, Vec<String>>, output_map: &mut HashMap<String, usize>) -> HashMap<String, usize> {

    for (rule_name, number) in output_map.iter() {
      if map.contains_key(number) {
        let value = map.entry(*number).or_insert(vec![]);
        if value.len() == 1 {
          map.remove(number);
        }
        for (_, list_rule_name) in map.iter_mut() {
          let index = list_rule_name.iter().position(|x| *x == *rule_name).unwrap();
          list_rule_name.remove(index);
          //println!("index: {:?}, list: {:?}", index, list_rule_name);
        }
      }
    }

    if map.len() == 0 {
      return output_map.clone();
    }

    for (number, list_rule_name) in map.iter() {
      if list_rule_name.len() == 1 {
        let rule_name = list_rule_name[0].clone();
        output_map.insert(rule_name, *number);
      }
    }
    return Ticket::select_correct_one(map, output_map);
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

  fn is_between_two_rules(&self, number: usize, first_rule: &str, second_rule: &str) -> bool {
    let rule_1: Vec<usize> = first_rule
      .split("-")
      .map(|x| x.parse().unwrap())
      .collect();
    let rule_2: Vec<usize> = second_rule
      .split("-")
      .map(|x| x.parse().unwrap())
      .collect();

    if number > rule_1[1] && number < rule_2[0]{
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

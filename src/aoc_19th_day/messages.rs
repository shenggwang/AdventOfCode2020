use std::{
  fmt::Debug,
  collections::{HashSet, HashMap},
  iter::FromIterator,
  sync::mpsc::channel
};
use threadpool::ThreadPool;
use regex::Regex;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Messages {
  pub valid_messages: HashMap<usize, String>,
  pub received_messages: Vec<String>
}

impl Messages {
  pub fn new() -> Messages {
    let valid_messages = HashMap::new();
    let received_messages = Vec::new();
    Messages {
      valid_messages,
      received_messages,
    }
  }
  pub fn append_valid_message(&mut self, valid_message: String) -> () {
    let message: Vec<&str> = valid_message.split(":").collect();
    let message_id: usize = message[0].parse::<usize>().unwrap();
    let message_value: String = message[1].replace("\"", "");

    self.valid_messages.insert(message_id, message_value);
  }
  pub fn append_message(&mut self, received_message: String) -> () {
    self.received_messages.push(received_message);
  }

  pub fn get_first_deciphered_message(deciphered_messages: HashMap<usize, String>) -> Vec<String> {
    let value = deciphered_messages.get(&0).unwrap().to_string();
    let cloned_deciphered_messages = deciphered_messages.clone();
    
    let mut list_valid_messages: Vec<String> = Messages::decipher_string_to_list(&cloned_deciphered_messages, value);
    let re = Regex::new(r"\d").unwrap();
    loop {
      let flag = list_valid_messages.iter().any(|x| re.is_match(x));
      if !flag {
        return list_valid_messages;
      }
      list_valid_messages = Messages::decipher_list_to_list(&cloned_deciphered_messages, list_valid_messages);
    }
  }

  pub fn get_first_deciphered_message_with_max(
    threadpool: &ThreadPool,
    deciphered_messages: HashMap<usize, String>, 
    max: usize) -> Vec<String> {
    let value = deciphered_messages.get(&0).unwrap().to_string();
    let cloned_deciphered_messages = deciphered_messages.clone();
    
    let mut list_valid_messages: Vec<String> = Messages::decipher_string_to_list(&cloned_deciphered_messages, value);
    let re = Regex::new(r"a|b").unwrap();
    loop {
      //let max_flag = list_valid_messages.iter().any(|x| x.len() > max);
      let number_flag = list_valid_messages.iter()
        .any(|x| re.find_iter(x).count() > max - 1);
      //println!("max flag {:?}, number flag {:?}, boolean: {:?}", max_flag, number_flag, !max_flag && number_flag);
      if number_flag {
        return list_valid_messages;
      }
      list_valid_messages = Messages::decipher_list_to_list(&cloned_deciphered_messages, list_valid_messages);
    }
  }

  pub fn decipher_list_to_list(deciphered_messages: &HashMap<usize, String>, vector: Vec<String>) -> Vec<String> {
    let mut list_valid_messages: Vec<String> = vec![];
    for message in vector.clone() {
      list_valid_messages.extend(Messages::decipher_string_to_list(deciphered_messages, message));
    }
    return list_valid_messages;
  }

  pub fn decipher_string_to_list(deciphered_messages: &HashMap<usize, String>, value: String) -> Vec<String> {

    let vector: Vec<char> = value.chars().collect();
    let vector_size = vector.len();
    let mut list_valid_messages: Vec<String> = vec![];
    let mut index = 0;
    while index < vector_size {
      let value_at_index = vector[index];
      if value_at_index == ' ' {
        index += 1;
        continue;
      }
      if value_at_index.is_digit(10) {
        let mut number: usize = value_at_index.to_digit(10).unwrap() as usize;
        while index < vector_size - 1 && vector[index + 1].is_digit(10) {
          number = (number * 10) + vector[index + 1].to_digit(10).unwrap() as usize;
          index += 1;
        }
        
        let content: String = deciphered_messages.get(&number).unwrap().trim().to_string();
        if content.contains('|') {
          let pair_content: Vec<String> = content.split("|").map(|x| x.trim().to_string()).collect();
          if list_valid_messages.len() == 0 {
            list_valid_messages.push(pair_content[0].to_string());
            list_valid_messages.push(pair_content[1].to_string());
          } else {
            for i in 0..list_valid_messages.len() {
              let stored_value = list_valid_messages[i].clone();
              list_valid_messages[i] = format!("{} {}", stored_value, pair_content[0]);
              list_valid_messages.push(format!("{} {}", stored_value, pair_content[1]));
            }
          }
        } else {
          if list_valid_messages.len() == 0 {
            list_valid_messages.push(content.to_string());
          } else {
            for i in 0..list_valid_messages.len() {
              list_valid_messages[i] = format!("{} {}", list_valid_messages[i], content.to_string());
            }
          }
        }
      } else {
        if list_valid_messages.len() == 0 {
          list_valid_messages.push(value_at_index.to_string());
        } else {
          for i in 0..list_valid_messages.len() {
            list_valid_messages[i] = format!("{} {}", list_valid_messages[i], value_at_index);
          }
        }
      }
      index += 1;
    }
    return list_valid_messages;
  }

  pub fn get_intersepted_number(messages: Messages, deciphered_messages: Vec<String>) -> usize {
    let set: HashSet<String> = HashSet::from_iter(messages.received_messages.iter().cloned());
    let intersection: Vec<_> = deciphered_messages.iter()
      .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect::<String>())
      .filter(|x| set.contains(&x.to_string()))
      .collect();
    return intersection.len();
  }

}

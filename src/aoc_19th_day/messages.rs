use std::{
  fmt::Debug,
  collections::{HashSet, HashMap},
  iter::FromIterator
};
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
      if flag {
        list_valid_messages = Messages::decipher_list_to_list(&cloned_deciphered_messages, list_valid_messages);
        //println!("list: {:?}, any number flag: {:?}", list_valid_messages, flag);
        continue;
      }
      return list_valid_messages;
    }
  }

  pub fn decipher_list_to_list(deciphered_messages: &HashMap<usize, String>, vector: Vec<String>) -> Vec<String> {
    let mut list_valid_messages: Vec<String> = vec![];
    for message in vector.clone() {
      //println!("message: {:?} on list: {:?}", &message, list_valid_messages);
      list_valid_messages.extend(Messages::decipher_string_to_list(deciphered_messages, message));
    }
    return list_valid_messages;
  }

  pub fn decipher_string_to_list(deciphered_messages: &HashMap<usize, String>, value: String) -> Vec<String> {

    let vector: Vec<char> = value.chars().collect();
    let vector_size = vector.len();
    let mut list_valid_messages: Vec<String> = vec![];
    let mut is_last_round = true;
    let mut index = 0;
    while index < vector_size {
      let value_at_index = vector[index];
      if value_at_index == ' ' {
        index += 1;
        continue;
      }
      if value_at_index.is_digit(10) && is_last_round {
        //println!("digit detected: {:?}", value_at_index);
        is_last_round = false;
        let mut number: usize = value_at_index.to_digit(10).unwrap() as usize;
        while index < vector_size - 1 && vector[index + 1].is_digit(10) {
          //println!("value larger than 9: {:?}, index {:?}", vector, index);
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
          //println!("add content {:?} on list : {:?}", content, list_valid_messages);
          if list_valid_messages.len() == 0 {
            list_valid_messages.push(content.to_string());
          } else {
            for i in 0..list_valid_messages.len() {
              list_valid_messages[i] = format!("{} {}", list_valid_messages[i], content.to_string());
            }
          }
        }
      } else {
        //println!("list {:?}, value at {:?}: {:?}", list_valid_messages, index, value_at_index);
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

  pub fn get_deciphered_messages(deciphered_messages: &mut HashMap<usize, String>) -> &HashMap<usize, String> {
    let cloned_deciphered_messages = deciphered_messages.clone();
    let mut is_last_round = true;
    //println!("map: {:?}", cloned_deciphered_messages);
    for (key, value) in cloned_deciphered_messages {
      let vector: Vec<char> = value.chars().collect();
      let vector_size = vector.len();
      // For the case there is either "a" or "b"
      if vector_size == 1 && (vector[0] == 'a' || vector[0] == 'b') {
        continue;
      }
      let mut new_string: String = "".to_string();
      let mut index = 0;
      while index < vector_size {
        let value_at_index = vector[index];
        if value_at_index == ' ' {
          index += 1;
          continue;
        }
        if value_at_index.is_digit(10) {
          is_last_round = false;
          let mut number: usize = value_at_index.to_digit(10).unwrap() as usize;
          while index < vector_size - 1 && vector[index + 1].is_digit(10) {
            //println!("test: {:?}, index {:?}", vector, index);
            number = (number * 10) + vector[index + 1].to_digit(10).unwrap() as usize;
            index += 1;
          }
          new_string = format!("{} {}", new_string, deciphered_messages.get(&number).unwrap());
        } else {
          new_string = format!("{} {}", new_string, value_at_index);
        }
        index += 1;
      }
      *deciphered_messages.get_mut(&key).unwrap() = new_string;
    }

    if is_last_round {
      return deciphered_messages;
    }
    return Messages::get_deciphered_messages(deciphered_messages);
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

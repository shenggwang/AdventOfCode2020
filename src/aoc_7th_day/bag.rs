use std::collections::HashMap;

pub struct BagRule {
  pub num: usize,
  pub bag_type: String,
}

impl BagRule {
  pub fn contains_recur(&self, bag: &str, collection: &HashMap<String, Vec<BagRule>>) -> bool {
    if self.bag_type == bag {
      return true;
    }
    collection
    .get(&self.bag_type)
    .unwrap()
    .iter()
    .any(|br| br.contains_recur(bag, collection))
  }

  pub fn bag_count(&self, collection: &HashMap<String, Vec<BagRule>>, prev_count: usize) -> usize {
    let rules = collection.get(&self.bag_type).unwrap();
    if rules.is_empty() {
      prev_count
    } else {
      rules
        .iter()
        .map(|br| br.bag_count(collection, br.num * prev_count))
        .sum::<usize>()
        + prev_count
    }
  }
}

impl From<&str> for BagRule {
  fn from(s: &str) -> Self {
    match s.find(" ") {
      Some(n) => {
        let num: usize = s[0..n].parse().unwrap();
        BagRule {
          num,
          bag_type: String::from(s[n + 1..].trim_end_matches("s")),
        }
      }
      // no bags
      None => {
        panic!("boom")
      }
    }
  }
}
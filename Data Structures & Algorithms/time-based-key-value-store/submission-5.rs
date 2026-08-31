use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct TimeMap {
  data: HashMap<String, Vec<(i32, String)>>
}

impl TimeMap {
  fn new() -> Self {
    Self {
      data: HashMap::new()
    }
  }

  fn set(&mut self, key: String, value: String, timestamp: i32) {
    match self.data.entry(key) {
      Entry::Occupied(mut v) => { v.get_mut().push((timestamp, value)); },
      Entry::Vacant(v) => { v.insert(vec![(timestamp, value)]); }
    }
  }

  fn get(&self, key: String, timestamp: i32) -> String {
    match self.data.get(&key) {
      Some(v) => {
        let (mut l, mut r) = (0, v.len() as i32);
        while l < r {
          let m = l + (r - l) / 2;
          if v[m as usize].0 <= timestamp {
            l = m + 1;
          } else {
            r = m;
          }
        }

        if l == 0 {
          "".to_string()
        } else {
          v[(l - 1) as usize].1.clone()
        }
      }
      None => "".to_string()
    }
  }
}
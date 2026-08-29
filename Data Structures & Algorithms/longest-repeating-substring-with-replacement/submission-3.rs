impl Solution {
  pub fn character_replacement(s: String, k: i32) -> i32 {
    use std::collections::HashMap;

    let bytes = s.as_bytes();
    let mut count = [0i32; 26];
    let mut l = 0;
    let mut result = 0;
    let mut max = 0;

    for r in 0..bytes.len() {
      count[Self::idx(bytes[r])] += 1;
      max = max.max(count[Self::idx(bytes[r])]);

      while (r - l + 1) as i32 - max > k {
        count[Self::idx(bytes[l])] -= 1;
        l += 1;
      }

      result = result.max(r - l + 1);
    }
    
    result as i32
  }

  fn idx(c: u8) -> usize {
    (c - b'A') as usize
  }
}
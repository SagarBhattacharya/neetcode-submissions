impl Solution {
  pub fn character_replacement(s: String, k: i32) -> i32 {
    use std::collections::HashMap;

    let bytes = s.as_bytes();
    let mut count = HashMap::new();
    let mut l = 0;
    let mut result = 0;
    let mut maxf = 0;

    for r in 0..bytes.len() {
      *count.entry(bytes[r]).or_insert(0) += 1;
      maxf = *count.values().max().unwrap_or(&0);

      while (r - l + 1) - maxf > k as usize {
        *count.entry(bytes[l]).or_insert(0) -= 1;
        l += 1;
      }

      result = result.max(r - l + 1);
    }
    
    result as i32
  }
}
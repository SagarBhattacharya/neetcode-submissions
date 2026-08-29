impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let mut start = 0;
    let bytes = s.as_bytes();

    for end in 0..bytes.len() {
      if bytes[start..end].contains(&bytes[end]) {
        while bytes[start] != bytes[end] {
          start += 1;
        }
        start += 1;
      }
      longest = longest.max(end - start + 1);
    }
    longest as i32
  }
}

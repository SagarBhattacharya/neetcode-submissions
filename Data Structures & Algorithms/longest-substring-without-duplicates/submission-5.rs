impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest = 0;
    let (mut start, mut end) = (0, 0);
    let bytes = s.as_bytes();

    for i in 0..bytes.len() {
      if bytes[start..end].contains(&bytes[i]) {
        longest = longest.max(end - start);
        while bytes[start] != bytes[i] {
          start += 1;
        }
        start += 1;
        end += 1;
      } else {
        end += 1;
      }
    }
    longest = longest.max(end - start);

    longest as i32
  }
}
impl Solution {
  pub fn is_palindrome(s: String) -> bool {
    let normalized = s
      .to_lowercase()
      .chars()
      .filter(|c| c.is_alphanumeric())
      .collect::<Vec<char>>();

    let (mut start, mut end) = (0, normalized.len().saturating_sub(1));
    while start < end {
      if normalized[start] != normalized[end] {
        return false;
      }
      start += 1;
      end -= 1;
    }
    true
  }
}
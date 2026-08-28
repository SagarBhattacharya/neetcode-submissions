impl Solution {
  pub fn encode(strs: Vec<String>) -> String {
    let total_len: usize = strs.iter().map(|s| s.len() + 10).sum();
    let mut encoded = String::with_capacity(total_len);
    for s in strs {
      encoded.push_str(&s.len().to_string());
      encoded.push('#');
      encoded.push_str(&s);
    }
    encoded
  }

  pub fn decode(s: String) -> Vec<String> {
    let mut strs = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
      let mut j = i;
      while bytes[j] != b'#' {
        j += 1;
      }
      let len = bytes[i..j]
        .iter()
        .fold(0usize, |acc, &b| acc * 10 + (b - b'0') as usize);

      let start = j + 1;
      let end = start + len;
      strs.push(s[start..end].to_string());
      i = end;
    }

    strs
  }
}

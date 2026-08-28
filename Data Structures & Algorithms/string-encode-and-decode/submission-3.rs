impl Solution {
  pub fn encode(strs: Vec<String>) -> String {
    let mut encoded = String::new();
    for s in strs {
      encoded.push_str(&format!("{}#{}", s.len(), s));
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

      let len: usize = s[i..j].parse().expect("valid integer");
      let start = j + 1;
      let end = start + len;
      strs.push(s[start..end].to_string());
      i = end;
    }

    strs
  }
}

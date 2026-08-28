impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
    let mut bytes = Vec::new();
    for s in strs {
      let meta = (s.len() as u32).to_be_bytes();
      bytes.extend_from_slice(&meta);
      bytes.extend_from_slice(s.as_bytes());
    }
    bytes.into_iter().map(|b| b as char).collect()
  }

  pub fn decode(s: String) -> Vec<String> {
    let bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();

    let mut strs = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
      let length = u32::from_be_bytes(
        bytes[i..i + 4].try_into().unwrap()
      ) as usize;

      i += 4;
      strs.push(String::from_utf8(bytes[i..i + length].to_vec()).unwrap());
      i += length;
    }

    strs
  }
}

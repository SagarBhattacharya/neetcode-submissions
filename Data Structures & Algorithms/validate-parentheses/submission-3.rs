impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for c in s.bytes() {
      match c {
        b'(' | b'{' | b'[' => stack.push(c),
        b')' => if stack.pop() != Some(b'(') { return false },
        b'}' => if stack.pop() != Some(b'{') { return false },
        b']' => if stack.pop() != Some(b'[') { return false },
        _ => {}
      }
    }
    stack.is_empty()
  }
}

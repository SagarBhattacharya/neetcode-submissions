impl Solution {
  pub fn is_valid(s: String) -> bool {
    let opening = "([{";
    let mut stack = vec![];
    let mut valid = true;

    for c in s.bytes() {
      if opening.contains(c as char) {
        stack.push(c);
        continue;
      } else {
        match c {
          b')' => {
            match stack.pop() {
              Some(b'(') => {},
              _ => {
                valid = false;
                break;
              },
            }
          },
          b'}' => {
            match stack.pop() {
              Some(b'{') => {},
              _ => {
                valid = false;
                break;
              },
            }
          },
          b']' => {
            match stack.pop() {
              Some(b'[') => {},
              _ => {
                valid = false;
                break;
              },
            }
          },
          _ => {}
        }
      }
    }
    if !stack.is_empty() { valid = false;}
    valid
  }
}

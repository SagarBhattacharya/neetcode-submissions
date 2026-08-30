impl Solution {
  pub fn min_window(s: String, t: String) -> String {
    if t.len() > s.len() { return "".to_string() }

    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut requirement = [0i32; 52];
    let mut window = [0i32; 52];

    tb.iter().for_each(
      |&c| requirement[Self::idx(c)] += 1
    );

    let mut l = 0;
    let mut best = None;
    let mut satisfied = 0;

    for r in 0..sb.len() {
      let needed = requirement[Self::idx(sb[r])];
      if needed > 0 {
        window[Self::idx(sb[r])] += 1;
        if window[Self::idx(sb[r])] <= needed {
            satisfied += 1;
        }
      }

      while satisfied == tb.len() {
        match best {
          Some((bl, br)) if r-l+1 < br-bl => best = Some((l, r+1)),
          None => best = Some((l, r+1)),
          _ => {}
        }

        let needed = requirement[Self::idx(sb[l])];
        if needed > 0 {
          if window[Self::idx(sb[l])] <= needed {
            satisfied -= 1;
          }
          window[Self::idx(sb[l])] -= 1;
        }
        l += 1;
      }
    }

    if let Some((bl, br)) = best {
      s[bl..br].to_string()
    } else {
      "".to_string()
    }
  }

  fn idx(c: u8) -> usize {
    if c.is_ascii_lowercase() {
      (c - b'a') as usize + 26
    } else {
      (c - b'A') as usize
    }
  }
}
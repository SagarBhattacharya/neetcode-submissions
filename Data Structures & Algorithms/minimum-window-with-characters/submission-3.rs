impl Solution {
  pub fn min_window(s: String, t: String) -> String {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    if tb.len() > sb.len() {
      return "".to_string();
    }

    let mut needed = [0i32; 52];
    tb.iter().for_each(
      |&c| needed[Self::idx(c)] += 1
    );

    let mut window = [0i32; 52];

    let mut l = 0;
    let mut best = None;
    for r in 0..sb.len() {
      window[Self::idx(sb[r])] += 1;

      while Self::valid_window(&needed, &window) {
        match best {
          Some((bl, br)) if r-l+1 < br-bl => best = Some((l, r+1)),
          None => best = Some((l, r+1)),
          _ => {}
        }

        window[Self::idx(sb[l])] -= 1;
        l += 1;
      }
    }

    if let Some((bl, br)) = best {
      s[bl..br].to_string()
    } else {
      "".to_string()
    }
  }

  fn valid_window(
    needed: &[i32; 52],
    window: &[i32; 52]
  ) -> bool {
    needed.iter()
      .zip(window.iter())
      .map(|(&n, &w)| n - w)
      .all(|i| i <= 0)
  }

  fn idx(c: u8) -> usize {
    if c.is_ascii_lowercase() {
      (c - b'a') as usize + 26
    } else {
      (c - b'A') as usize
    }
  }
}
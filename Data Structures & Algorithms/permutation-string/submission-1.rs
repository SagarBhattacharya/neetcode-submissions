impl Solution {
  pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
      return false;
    }

    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    let mut need = [0i32; 26];
    let mut window = [0i32; 26];

    b1.iter()
      .for_each(|&c| need[Self::idx(c)] += 1);
    b2.iter().take(b1.len())
      .for_each(|&c| window[Self::idx(c)] += 1);

    if window == need {
      return true;
    }

    for r in b1.len()..b2.len() {
      let entering = Self::idx(b2[r]);
      let leaving = Self::idx(b2[r - b1.len()]);

      window[entering] += 1;
      window[leaving] -= 1;

      if window == need {
        return true;
      }
    }

    false
  }

  fn idx(c: u8) -> usize {
    (c - b'a') as usize
  }
}
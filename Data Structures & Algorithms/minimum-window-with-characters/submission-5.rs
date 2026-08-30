impl Solution {
  pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    use std::ops::{AddAssign, SubAssign};

    if t.len() > s.len() { return "".to_string() }

    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut requirement = HashMap::new();
    tb.iter().for_each(
      |&c| requirement.entry(c)
        .or_insert(0)
        .add_assign(1)
    );

    let mut window = HashMap::new();

    let mut l = 0;
    let mut best = None;
    let mut satisfied = 0;

    for r in 0..sb.len() {
      if let Some(&needed) = requirement.get(&sb[r]) {
        let count = window.entry(sb[r]).or_insert(0);
        count.add_assign(1);
        (*count <= needed).then(|| satisfied += 1);
      }

      while satisfied == tb.len() {
        match best {
          Some((bl, br)) if r-l+1 < br-bl => best = Some((l, r+1)),
          None => best = Some((l, r+1)),
          _ => {}
        }

        if let Some(&needed) = requirement.get(&sb[l]) {
          let count = window.entry(sb[l]).or_insert(0);
          (*count <= needed).then(|| satisfied -= 1);
          count.sub_assign(1);
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
}
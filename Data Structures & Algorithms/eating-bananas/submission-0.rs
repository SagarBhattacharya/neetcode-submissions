impl Solution {
  pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut l = 1;
    let mut r = piles.iter().copied()
      .max().unwrap_or(1);

    while l <= r {
      let k = l + (r - l) / 2;
      let n = piles.iter()
        .map(|&p| (p + k - 1) / k)
        .sum::<i32>();

      if n <= h {
        r = k-1;
      } else {
        l=k+1;
      }
    }

    l
  }
}
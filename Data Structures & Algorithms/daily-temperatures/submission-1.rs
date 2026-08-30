impl Solution {
  pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0i32; temperatures.len()];
    let mut warmer = vec![];

    for (i, temp) in
      temperatures.iter()
        .enumerate().rev()
    {
      while let Some((t, idx)) = warmer.last().copied() {
        if temp >= t {
          warmer.pop();
        } else {
          result[i] = (idx - i) as i32;
          break;
        };
      }
      warmer.push((temp, i));
    }

    result
  }
}
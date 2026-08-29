impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();

    let mut prefix_maximum = vec![0; n];
    for i in 1..n {
      prefix_maximum[i] = height[i-1].max(prefix_maximum[i-1]);
    }

    let mut suffix_maximum = vec![0; n];
    for i in (0..n-1).rev() {
      suffix_maximum[i] = height[i+1].max(suffix_maximum[i+1]);
    }
    
    prefix_maximum.iter()
      .zip(suffix_maximum.iter())
      .zip(height.iter())
      .map(|((p, s), h)| p.min(s) - h)
      .filter(|&i| i > 0)
      .sum()
  }
}
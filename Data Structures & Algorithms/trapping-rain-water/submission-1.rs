impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    
    let mut prefix_maximum = vec![0; n];
    for i in 0..n {
      prefix_maximum[i] = height[i.saturating_sub(1)]
        .max(prefix_maximum[i.saturating_sub(1)]);
    }

    let mut suffix_maximum = vec![0; n];
    for i in (0..n).rev() {
      suffix_maximum[i] = *height.get(i + 1).unwrap_or(&0)
        .max(suffix_maximum.get(i + 1).unwrap_or(&0));
    }
    
    let mut total_water = 0;
    for i in 0..n {
      let water = prefix_maximum[i].min(suffix_maximum[i]) - height[i];
      if water > 0 {
        total_water += water;
      }
    }
    
    total_water
  }
}